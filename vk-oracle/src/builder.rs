use crate::{dependency::*, oracle::*, parse::*, registry::*};
use heck::{ToShoutySnakeCase, ToSnakeCase};
use std::{
    collections::{HashMap, HashSet},
    iter,
};
use vk_parse as vk;

trait ToDimSeparated {
    fn to_dim_separated(&self) -> String;
}

impl ToDimSeparated for &str {
    fn to_dim_separated(&self) -> String {
        let mut input = self.chars().peekable();
        let mut output = String::new();
        while let Some(c) = input.next() {
            if ('1'..='4').contains(&c) {
                let d = input.next();
                if d == Some('D') && input.peek().copied().map(|s| s.is_uppercase()).unwrap_or(true) {
                    output.push('_');
                    output.push(c);
                    output.push('D');
                    output.push('_');
                } else {
                    output.push(c);
                    if let Some(d) = d {
                        output.push(d);
                    }
                }
            } else {
                output.push(c);
            }
        }
        output
    }
}

impl ToDimSeparated for String {
    fn to_dim_separated(&self) -> String {
        self.as_str().to_dim_separated()
    }
}

trait ToShortName {
    fn to_short_name(&self) -> String;
}

impl ToShortName for str {
    fn to_short_name(&self) -> String {
        let without_prefix = self
            .strip_prefix("Vk")
            .or_else(|| self.strip_prefix("PFN_vk"))
            .or_else(|| self.strip_prefix("VK_"))
            .or_else(|| self.strip_prefix("vk"))
            .unwrap_or(self);
        without_prefix.to_dim_separated()
    }
}

trait UnifyWith<T> {
    fn unify_with(&mut self, value: T);
}

impl<T> UnifyWith<T> for Option<T>
where
    T: PartialEq,
{
    fn unify_with(&mut self, value: T) {
        if let Some(current) = self.as_ref() {
            if *current != value {
                panic!("cannot unify different values");
            }
        } else {
            self.replace(value);
        }
    }
}

struct ExtensionInfo<'a> {
    ext: &'a vk::Extension,
    category: ExtensionCategory,
    promoted_to_version: Option<Version>,
    dependencies: DependencyExpr<&'a str>,
    has_instance_dependency: bool,
}

impl<'a> ExtensionInfo<'a> {
    pub fn affects_category(&self, category: ExtensionCategory) -> bool {
        self.category == category || (category == ExtensionCategory::Instance && self.has_instance_dependency)
    }
}

struct CommandInfo<'a> {
    cmd: &'a vk::Command,
    category: Option<CommandCategory>,
    aliases: Vec<&'a str>,
    dependencies: DependencyExpr<&'a str>,
}

trait CmdExt {
    fn guess_category(&self) -> CommandCategory;
}

impl CmdExt for vk::Command {
    fn guess_category(&self) -> CommandCategory {
        match self.spec_name() {
            "vkGetInstanceProcAddr"
            | "vkCreateInstance"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceExtensionProperties"
            | "vkEnumerateInstanceVersion" => CommandCategory::Global,
            "vkGetDeviceProcAddr" => CommandCategory::Instance,
            _ => {
                let cmd_def = match self {
                    vk::Command::Definition(cmd_def) => cmd_def,
                    _ => panic!("command missing definition"),
                };
                let is_first_param_from_device = cmd_def
                    .params
                    .iter()
                    .find(|param| param.is_vulkan_api())
                    .and_then(|param| param.definition.type_name.as_deref())
                    .map(|type_name| matches!(type_name, "VkDevice" | "VkCommandBuffer" | "VkQueue"))
                    .unwrap_or(false);
                if is_first_param_from_device {
                    CommandCategory::Device
                } else {
                    CommandCategory::Instance
                }
            }
        }
    }
}

trait TypeSpecExt {
    fn function_pointer_code(&self) -> &str;
}

impl TypeSpecExt for vk::TypeSpec {
    fn function_pointer_code(&self) -> &str {
        match self {
            vk::TypeSpec::Funcpointer(code) => code.code.as_str(),
            vk::TypeSpec::Code(code) => code.code.as_str(),
            _ => panic!("missing code for function pointer type"),
        }
    }
}

struct TypedEnumInfo<'a> {
    enum_type_name: &'a str,
    ext_name: Option<&'a str>,
}

struct GlobalEnumInfo {
    is_referenced: bool,
}

enum EnumInfoDetail<'a> {
    Typed(TypedEnumInfo<'a>),
    Global(GlobalEnumInfo),
}

struct EnumInfo<'a> {
    en: &'a vk::Enum,
    detail: EnumInfoDetail<'a>,
}

struct BitmaskTypeInfo<'a> {
    bit_width: BitWidth,
    values_type_name: Option<&'a str>,
}

struct EnumTypeInfo<'a> {
    bitmask_type_name: Option<&'a str>,
    values: Vec<&'a str>,
}

struct AggregateTypeInfo<'a> {
    is_union: bool,
    members: &'a [vk::TypeMember],
    extended_by: Vec<&'a str>,
}

impl<'a> AggregateTypeInfo<'a> {
    fn iter_member_definitions(&self) -> impl Iterator<Item = &'a vk::TypeMemberDefinition> {
        self.members.iter().filter_map(|member| match member {
            vk::TypeMember::Definition(def) if def.is_vulkan_api() => Some(def),
            _ => None,
        })
    }
}

struct MemberInfo<'a> {
    def: &'a vk::TypeMemberDefinition,
    decl: CVariableDecl<'a>,
    name: String,
}

impl<'a> MemberInfo<'a> {
    fn merge_with(&mut self, other: Self) {
        let bit_count = self.decl.ty.bit_count.unwrap() + other.decl.ty.bit_count.unwrap();
        self.name = format!("{}_and_{}", self.name, other.name);
        if bit_count == 32 {
            self.decl.ty.base = CBaseType::BuiltIn(CBuiltInType::U32);
            self.decl.ty.bit_count = None;
        } else if bit_count < 32 {
            self.decl.ty.bit_count = Some(bit_count);
        } else {
            panic!("bitfield types did not merge to a u32")
        }
    }
}

trait TypeDeclExt {
    fn array_as_pointer(self) -> Self;
    fn apply_len_terms(&mut self, len: &str, alt_len: Option<&str>);
    fn type_index(&self) -> Option<TypeIndex>;
}

impl TypeDeclExt for TypeDecl {
    fn array_as_pointer(self) -> Self {
        match self {
            Self::Array(array_decl) => {
                let is_multi_element = match &array_decl.array.size {
                    ArraySize::Literal(literal) => match *literal {
                        Literal::Int(n) => n > 1,
                        Literal::U32(n) => n > 1,
                        Literal::U64(n) => n > 1,
                        Literal::F32(_) => panic!("unexpected array size type"),
                    },
                    _ => true,
                };
                Self::Pointer(PointerDecl {
                    is_const: array_decl.is_const,
                    array_hint: is_multi_element.then_some(array_decl.array),
                    element_type: array_decl.element_type,
                })
            }
            non_array => non_array,
        }
    }

    fn apply_len_terms(&mut self, len: &str, alt_len: Option<&str>) {
        let mut type_decl = self;
        for len_part in len.split(',') {
            match type_decl {
                TypeDecl::Array(array_decl) => {
                    if len_part == "null-terminated" {
                        array_decl.array.is_null_terminated = true;
                    }
                    type_decl = array_decl.element_type.as_mut();
                }
                TypeDecl::Pointer(pointer_decl) => {
                    if len_part != "1" {
                        let array = pointer_decl.array_hint.get_or_insert(ArrayInfo {
                            size: ArraySize::Unknown,
                            is_null_terminated: false,
                        });
                        if len_part == "null-terminated" {
                            array.is_null_terminated = true;
                        } else if len_part.contains("latexmath:") {
                            if let Some(alt_len) = alt_len {
                                println!("TODO: parse {alt_len:?}");
                            }
                        } else {
                            let idents: Vec<_> = len_part
                                .split("->")
                                .map(|s| s.to_dim_separated().to_snake_case())
                                .collect();
                            array.size = ArraySize::Named(idents.join("."));
                        }
                    }
                    type_decl = pointer_decl.element_type.as_mut();
                }
                _ => panic!("cannot handle null-terminated length"),
            }
        }
    }

    fn type_index(&self) -> Option<TypeIndex> {
        match self {
            TypeDecl::Type(type_index) => Some(*type_index),
            _ => None,
        }
    }
}

struct BaseTypeInfo<'a> {
    decl: CVariableDecl<'a>,
}

enum TypeInfoDetail<'a> {
    Alias(&'a str),
    BaseType(BaseTypeInfo<'a>),
    Bitmask(BitmaskTypeInfo<'a>),
    Handle,
    FunctionPointer,
    Enum(EnumTypeInfo<'a>),
    Aggregate(AggregateTypeInfo<'a>),
}

impl<'a> TypeInfoDetail<'a> {
    fn get_enum_mut(&mut self) -> Option<&mut EnumTypeInfo<'a>> {
        match self {
            Self::Enum(type_info) => Some(type_info),
            _ => None,
        }
    }

    fn get_enum(&self) -> Option<&EnumTypeInfo<'a>> {
        match self {
            Self::Enum(type_info) => Some(type_info),
            _ => None,
        }
    }

    fn get_bitmask(&self) -> Option<&BitmaskTypeInfo<'a>> {
        match self {
            Self::Bitmask(type_info) => Some(type_info),
            _ => None,
        }
    }
}

struct TypeInfo<'a> {
    ty: &'a vk::Type,
    is_referenced: bool,
    detail: TypeInfoDetail<'a>,
}

type TypeInfoMap<'a> = HashMap<&'a str, TypeInfo<'a>>;

trait TypeInfoMapExt<'a> {
    fn add_enum_value(&mut self, type_name: &'a str, value_name: &'a str);
}

impl<'a> TypeInfoMapExt<'a> for TypeInfoMap<'a> {
    fn add_enum_value(&mut self, type_name: &'a str, value_name: &'a str) {
        let type_info = self.get_mut(type_name).unwrap();
        type_info.detail.get_enum_mut().unwrap().values.push(value_name);
    }
}

type ExtensionInfoMap<'a> = HashMap<&'a str, ExtensionInfo<'a>>;

trait ExtensionInfoMapExt<'a> {
    fn dependencies_supported(&self, dependencies: &DependencyExpr<&'a str>) -> bool;
}

impl<'a> ExtensionInfoMapExt<'a> for ExtensionInfoMap<'a> {
    fn dependencies_supported(&self, dependencies: &DependencyExpr<&'a str>) -> bool {
        let mut all_extensions_supported = true;
        dependencies.visit_extensions(|dep_name| {
            if !self.contains_key(dep_name) {
                all_extensions_supported = false;
            }
        });
        all_extensions_supported
    }
}

pub struct OracleBuilder<'a> {
    registry: &'a vk::Registry,
    vulkan_version: (u16, u16),
    header_version: u16,
    tags_by_name: HashMap<&'a str, &'a vk::Tag>,
    type_info_by_name: TypeInfoMap<'a>,
    extension_info_by_name: ExtensionInfoMap<'a>,
    command_info_by_name: HashMap<&'a str, CommandInfo<'a>>,
    enum_info_by_name: HashMap<&'a str, EnumInfo<'a>>,
    all_enum: Vec<&'a vk::Enum>,
    external_type_names: HashMap<&'a str, usize>,
}

#[derive(Default)]
struct IndexMaps<'a> {
    constant_index_by_name: HashMap<&'a str, ConstantIndex>,
    type_index_by_name: HashMap<&'a str, TypeIndex>,
    extension_index_by_name: HashMap<&'a str, ExtensionIndex>,
    command_index_by_name: HashMap<&'a str, CommandIndex>,
}

enum SliceParameterKind {
    Required,
    Optional,
    Separate,
}

trait SliceLengthParameterTransformExt {
    fn add(&mut self, param_index: usize, kind: SliceParameterKind);
}

impl SliceLengthParameterTransformExt for SliceLengthParameterTransform {
    fn add(&mut self, param_index: usize, kind: SliceParameterKind) {
        match kind {
            SliceParameterKind::Required => self.required_param.push(param_index),
            SliceParameterKind::Optional => self.optional_param.push(param_index),
            SliceParameterKind::Separate => self.separate_param.push(param_index),
        }
    }
}

trait ParameterTransformExt {
    fn add_slice_length(&mut self, param_index: usize, kind: SliceParameterKind);
}

impl ParameterTransformExt for ParameterTransform {
    fn add_slice_length(&mut self, param_index: usize, kind: SliceParameterKind) {
        take_mut::take(self, |transform| match transform {
            ParameterTransform::None => {
                let mut slice_length_info = SliceLengthParameterTransform {
                    required_param: Vec::new(),
                    optional_param: Vec::new(),
                    separate_param: Vec::new(),
                };
                slice_length_info.add(param_index, kind);
                ParameterTransform::FromSliceLength(slice_length_info)
            }
            ParameterTransform::FromSliceLength(mut slice_length_info) => {
                slice_length_info.add(param_index, kind);
                ParameterTransform::FromSliceLength(slice_length_info)
            }
            _ => panic!("slice length parameter already has another use"),
        });
    }
}

impl<'a> OracleBuilder<'a> {
    pub fn new(registry: &'a vk::Registry) -> Self {
        let mut builder = Self {
            registry,
            vulkan_version: (0, 0),
            header_version: 0,
            tags_by_name: HashMap::new(),
            type_info_by_name: HashMap::new(),
            extension_info_by_name: HashMap::new(),
            command_info_by_name: HashMap::new(),
            enum_info_by_name: HashMap::new(),
            all_enum: Vec::new(),
            external_type_names: HashMap::new(),
        };
        builder.collect_tags();
        builder.collect_types();
        builder.collect_enums();
        builder.collect_extensions();
        builder.collect_commands();
        builder.propagate_references();
        builder.move_enum_references_to_bitmasks();
        builder
    }

    fn collect_tags(&mut self) {
        for tag in self.registry.iter_tags() {
            if self.tags_by_name.insert(tag.spec_name(), tag).is_some() {
                panic!("duplicate tag name");
            }
        }
    }

    fn collect_types(&mut self) {
        let mut values_for_bitmasks = Vec::new();
        for ty in self.registry.iter_types() {
            let detail = if let Some(alias) = ty.alias.as_deref() {
                TypeInfoDetail::Alias(alias)
            } else {
                match ty.category.as_deref() {
                    Some("basetype") => {
                        let vk::TypeSpec::Code(code) = &ty.spec else {
                            panic!("expected code for basetype");
                        };
                        if let Some(decl) = parse_typedef(code.code.as_str()) {
                            TypeInfoDetail::BaseType(BaseTypeInfo { decl })
                        } else {
                            continue;
                        }
                    }
                    Some("bitmask") => {
                        let bit_width = match &ty.spec {
                            vk::TypeSpec::Code(vk::TypeCode { code, .. }) => {
                                let def = parse_typedef(code).unwrap();
                                match def.ty.base {
                                    CBaseType::Named("VkFlags") => BitWidth::U32,
                                    CBaseType::Named("VkFlags64") => BitWidth::U64,
                                    _ => panic!("unknown bitmask type {:?}", def.ty),
                                }
                            }
                            _ => {
                                panic!("failed to find bitmask size for {ty:?}")
                            }
                        };
                        let values_name = ty.requires.as_deref().or(ty.bitvalues.as_deref());
                        if let Some(values_name) = values_name {
                            values_for_bitmasks.push((values_name, ty.spec_name()));
                        }
                        TypeInfoDetail::Bitmask(BitmaskTypeInfo {
                            values_type_name: values_name,
                            bit_width,
                        })
                    }
                    Some("handle") => TypeInfoDetail::Handle,
                    Some("funcpointer") => TypeInfoDetail::FunctionPointer,
                    Some("enum") => TypeInfoDetail::Enum(EnumTypeInfo {
                        bitmask_type_name: None,
                        values: Vec::new(),
                    }),
                    Some("struct") | Some("union") => match &ty.spec {
                        vk::TypeSpec::Members(members) => TypeInfoDetail::Aggregate(AggregateTypeInfo {
                            is_union: ty.category.as_deref() == Some("union"),
                            members: members.as_slice(),
                            extended_by: Vec::new(),
                        }),
                        _ => panic!("unexpected type spec for struct"),
                    },
                    Some("define") => {
                        if let vk::TypeSpec::Code(type_code) = &ty.spec {
                            let code = type_code.code.as_str();
                            if let Some(ver) = try_parse_header_version(code) {
                                self.header_version = ver;
                            }
                            if let Some((maj, min)) = try_parse_header_version_complete(code) {
                                self.vulkan_version = (maj, min);
                            }
                        }
                        continue;
                    }
                    _ => continue,
                }
            };
            let spec_name = ty.spec_name();
            if self
                .type_info_by_name
                .insert(
                    spec_name,
                    TypeInfo {
                        ty,
                        is_referenced: false,
                        detail,
                    },
                )
                .is_some()
            {
                panic!("duplicate type name {spec_name}");
            }
        }

        // link extension structs from their base structs
        for ty in self.registry.iter_types() {
            if let Some(structextends) = ty.structextends.as_deref() {
                let ext_name = ty.spec_name();
                if !matches!(
                    self.type_info_by_name.get(ext_name).unwrap().detail,
                    TypeInfoDetail::Aggregate(_)
                ) {
                    panic!("expected an aggregrate type for extends type");
                }
                for base_name in structextends.split(',') {
                    let type_info = self.type_info_by_name.get_mut(base_name).unwrap();
                    match &mut type_info.detail {
                        TypeInfoDetail::Aggregate(aggregate_info) => aggregate_info.extended_by.push(ext_name),
                        _ => {
                            panic!("expected an aggregrate type for base type")
                        }
                    }
                }
            }
        }

        // link the bitmask values type back to the bitmask type
        for (values_name, bitmask_name) in values_for_bitmasks.drain(..) {
            if let Some(type_info) = self.type_info_by_name.get_mut(values_name) {
                type_info.detail.get_enum_mut().unwrap().bitmask_type_name = Some(bitmask_name);
            }
        }
    }

    fn collect_enums(&mut self) {
        for enums in self.registry.iter_enums() {
            match enums.kind.as_deref() {
                Some("enum") | Some("bitmask") => {
                    let enum_type_name = enums.name.as_deref().expect("missing enum type name");
                    for en in enums
                        .children
                        .iter()
                        .filter_map(|enums_child| match enums_child {
                            vk::EnumsChild::Enum(en) => Some(en),
                            _ => None,
                        })
                        .filter(|en| en.is_vulkan_api())
                    {
                        let spec_name = en.spec_name();
                        let detail = EnumInfoDetail::Typed(TypedEnumInfo {
                            enum_type_name,
                            ext_name: None,
                        });
                        if self
                            .enum_info_by_name
                            .insert(spec_name, EnumInfo { en, detail })
                            .is_some()
                        {
                            panic!("duplicate enum value {spec_name}");
                        }
                        self.all_enum.push(en);
                        self.type_info_by_name.add_enum_value(enum_type_name, spec_name);

                        // check bitmask values match the size of the bitmask type
                        if let Some(bitwidth) = enums.bitwidth.map(|n| match n {
                            32 => BitWidth::U32,
                            64 => BitWidth::U64,
                            _ => panic!("unknown bit width"),
                        }) {
                            let enum_type_info = self.type_info_by_name.get(enum_type_name).unwrap();
                            let bitmask_type_name =
                                enum_type_info.detail.get_enum().unwrap().bitmask_type_name.unwrap();
                            let bitmask_type_info = self.type_info_by_name.get(bitmask_type_name).unwrap();
                            match &bitmask_type_info.detail {
                                TypeInfoDetail::Bitmask(bitmask_detail) => {
                                    if bitwidth != bitmask_detail.bit_width {
                                        panic!("bitmask bit width mismatch!");
                                    }
                                }
                                _ => panic!("unexpected bitmask type detail"),
                            };
                        }
                    }
                }
                _ => {
                    for en in enums
                        .children
                        .iter()
                        .filter_map(|enums_child| match enums_child {
                            vk::EnumsChild::Enum(en) => Some(en),
                            _ => None,
                        })
                        .filter(|en| en.is_vulkan_api())
                    {
                        let spec_name = en.spec_name();
                        let detail = EnumInfoDetail::Global(GlobalEnumInfo { is_referenced: false });
                        if self
                            .enum_info_by_name
                            .insert(spec_name, EnumInfo { en, detail })
                            .is_some()
                        {
                            panic!("duplicate enum value {spec_name}");
                        }
                        self.all_enum.push(en);
                    }
                }
            }
        }
    }

    fn collect_extensions(&mut self) {
        // collect all supported extensions
        for ext in self.registry.iter_extensions() {
            let spec_name = ext.spec_name();
            let category = match ext.ext_type.as_deref() {
                Some("instance") => ExtensionCategory::Instance,
                Some("device") => ExtensionCategory::Device,
                _ => panic!("unknown extension type {:?} for extension {}", ext.ext_type, ext.name),
            };
            let promoted_to_version = ext.promotedto.as_deref().and_then(try_parse_version);
            let dependencies = ext
                .depends
                .as_deref()
                .map(|s| parse_depends(s))
                .unwrap_or(DependencyExpr::Always);

            if self
                .extension_info_by_name
                .insert(
                    spec_name,
                    ExtensionInfo {
                        ext,
                        category,
                        promoted_to_version,
                        dependencies,
                        has_instance_dependency: false,
                    },
                )
                .is_some()
            {
                panic!("duplicate extension {}", ext.name);
            }
        }

        // build info about which device extensions affect the instance through dependencies
        {
            let mut visited = HashSet::new();
            let mut finished = HashSet::new();
            let mut stack: Vec<_> = self.extension_info_by_name.keys().copied().collect();
            while let Some(name) = stack.pop() {
                if visited.insert(name) {
                    // push back onto the stack behind all dependencies
                    stack.push(name);
                    self.extension_info_by_name[name]
                        .dependencies
                        .visit_extensions(|dep_name| {
                            stack.push(dep_name);
                        });
                    continue;
                }
                if finished.insert(name) {
                    // mark as having an instance dependency if any dependency affects the instance
                    let mut has_instance_dependency = false;
                    if self.extension_info_by_name[name].category == ExtensionCategory::Device {
                        self.extension_info_by_name[name]
                            .dependencies
                            .visit_extensions(|dep_name| {
                                if self.extension_info_by_name[dep_name].affects_category(ExtensionCategory::Instance) {
                                    has_instance_dependency = true;
                                }
                            });
                        if has_instance_dependency {
                            self.extension_info_by_name
                                .get_mut(name)
                                .unwrap()
                                .has_instance_dependency = true;
                        }
                    }
                }
            }
        }
    }

    fn collect_commands(&mut self) {
        // gather the set of command names, types and enum values from features and extensions
        let mut referenced_command_names = HashSet::new();
        for (ext_name, item) in self
            .registry
            .iter_features()
            .flat_map(|feature| iter::repeat(None).zip(feature.children.iter()))
            .chain(self.registry.iter_extensions().flat_map(|ext| {
                let ext_name = ext.spec_name();
                iter::repeat(Some(ext_name)).zip(ext.children.iter())
            }))
            .filter(|(_, ext_child)| ext_child.is_vulkan_api())
            .filter_map(|(ext_name, ext_child)| match ext_child {
                vk::ExtensionChild::Require { depends, items, .. } => {
                    let require_deps = depends.as_deref().map(parse_depends).unwrap_or(DependencyExpr::Always);
                    self.extension_info_by_name
                        .dependencies_supported(&require_deps)
                        .then_some((ext_name, items))
                }
                _ => None,
            })
            .flat_map(|(ext_name, items)| iter::repeat(ext_name).zip(items.iter()))
        {
            match item {
                vk::InterfaceItem::Command { name, .. } => {
                    referenced_command_names.insert(name.as_str());
                }
                vk::InterfaceItem::Type { name, .. } => {
                    if let Some(type_info) = self.type_info_by_name.get_mut(name.as_str()) {
                        type_info.is_referenced = true;
                    }
                }
                vk::InterfaceItem::Enum(en) if en.is_vulkan_api() => {
                    let spec_name = en.spec_name();
                    if let Some(enum_type_name) = match &en.spec {
                        vk::EnumSpec::Alias { extends, .. } => extends.as_deref(),
                        vk::EnumSpec::Offset { extends, .. } => Some(extends.as_str()),
                        vk::EnumSpec::Bitpos { extends, .. } => extends.as_deref(),
                        vk::EnumSpec::Value { extends, .. } => extends.as_deref(),
                        vk::EnumSpec::None => None,
                        _ => unimplemented!(),
                    } {
                        let detail = EnumInfoDetail::Typed(TypedEnumInfo {
                            enum_type_name,
                            ext_name,
                        });
                        self.enum_info_by_name
                            .entry(spec_name)
                            .and_modify(|enum_info| {
                                match &enum_info.detail {
                                    EnumInfoDetail::Typed(typed_enum_info) => {
                                        if typed_enum_info.enum_type_name != enum_type_name {
                                            panic!("enum type name mismatch");
                                        }
                                    }
                                    _ => panic!("no existing enum type name {}", enum_info.en.spec_name()),
                                }
                                // TODO: check this spec has the same numerical value as the existing info?
                            })
                            .or_insert_with(|| {
                                self.all_enum.push(en);
                                self.type_info_by_name.add_enum_value(enum_type_name, spec_name);
                                EnumInfo { en, detail }
                            });
                    } else if let Some(enum_info) = self.enum_info_by_name.get_mut(spec_name) {
                        match &mut enum_info.detail {
                            EnumInfoDetail::Global(global_enum_info) => global_enum_info.is_referenced = true,
                            _ => panic!("enum is not global"),
                        }
                    } else if !spec_name.ends_with("_SPEC_VERSION") && !spec_name.ends_with("_EXTENSION_NAME") {
                        println!("ignoring required constant {spec_name}");
                    }
                }
                _ => {}
            }
        }

        // collect referenced commands
        let mut aliases = Vec::new();
        for cmd in self.registry.iter_commands() {
            let spec_name = cmd.spec_name();
            if !referenced_command_names.contains(spec_name) {
                continue;
            }

            if self
                .command_info_by_name
                .insert(
                    spec_name,
                    CommandInfo {
                        cmd,
                        category: None,
                        aliases: Vec::new(),
                        dependencies: DependencyExpr::Never,
                    },
                )
                .is_some()
            {
                panic!("duplicate command {spec_name}");
            }
            if let Some(alias_of) = cmd.alias_of() {
                aliases.push((alias_of, spec_name));
            }
        }
        for (name, alias) in aliases.drain(..) {
            self.command_info_by_name.get_mut(name).unwrap().aliases.push(alias);
        }

        // add dependencies and set category where known
        for feature in self.registry.iter_features() {
            let version = try_parse_version(&feature.name).unwrap();
            for cmd_name in feature
                .children
                .iter()
                .filter(|ext_child| ext_child.is_vulkan_api())
                .filter_map(|ext_child| match ext_child {
                    vk::ExtensionChild::Require { items, .. } => Some(items),
                    _ => None,
                })
                .flat_map(|items| items.iter())
                .filter_map(|item| match item {
                    vk::InterfaceItem::Command { name, .. } => Some(name.as_str()),
                    _ => None,
                })
            {
                let cmd_info = self.command_info_by_name.get_mut(cmd_name).unwrap();
                take_mut::take(&mut cmd_info.dependencies, |current_deps| {
                    DependencyExpr::Or(vec![current_deps, DependencyExpr::Version(version)])
                });
            }
        }
        for ext in self.registry.iter_extensions() {
            let ext_name = ext.spec_name();
            let category = CommandCategory::from(self.extension_info_by_name[ext_name].category);

            for (depends, items) in ext
                .children
                .iter()
                .filter(|ext_child| ext_child.is_vulkan_api())
                .filter_map(|ext_child| match ext_child {
                    vk::ExtensionChild::Require { depends, items, .. } => Some((depends, items)),
                    _ => None,
                })
            {
                let require_deps = depends.as_deref().map(parse_depends).unwrap_or(DependencyExpr::Always);
                if !self.extension_info_by_name.dependencies_supported(&require_deps) {
                    continue;
                }

                let require_deps = DependencyExpr::And(vec![DependencyExpr::Extension(ext_name), require_deps]);
                for cmd_name in items.iter().filter_map(|item| match item {
                    vk::InterfaceItem::Command { name, .. } => Some(name.as_str()),
                    _ => None,
                }) {
                    let cmd_info = self.command_info_by_name.get_mut(cmd_name).unwrap();
                    cmd_info.category.unify_with(category);
                    take_mut::take(&mut cmd_info.dependencies, |current_deps| {
                        DependencyExpr::Or(vec![current_deps, require_deps.clone()])
                    });
                }
            }
        }

        // simplify and check dependencies
        for cmd_info in self.command_info_by_name.values_mut() {
            cmd_info.dependencies.simplify();
            if matches!(cmd_info.dependencies, DependencyExpr::Never) {
                panic!("command {} can never be loaded", cmd_info.cmd.spec_name());
            }
        }

        // copy category from aliases
        let mut category_pushes = Vec::new();
        for cmd_info in self.command_info_by_name.values() {
            if let Some(category) = cmd_info.category {
                if let Some(alias_of) = cmd_info.cmd.alias_of() {
                    category_pushes.push((alias_of, category));
                }
            }
        }
        for (name, category) in category_pushes.drain(..) {
            self.command_info_by_name
                .get_mut(name)
                .unwrap()
                .category
                .unify_with(category);
        }

        // guess remaining categories (for commands that are not aliases)
        for cmd_info in self.command_info_by_name.values_mut() {
            if cmd_info.category.is_none() {
                cmd_info.category = Some(cmd_info.cmd.guess_category());
            }
        }

        // copy category to aliases
        for cmd_info in self.command_info_by_name.values() {
            if let Some(category) = cmd_info.category {
                for &alias in &cmd_info.aliases {
                    category_pushes.push((alias, category));
                }
            }
        }
        for (name, category) in category_pushes.drain(..) {
            self.command_info_by_name
                .get_mut(name)
                .unwrap()
                .category
                .unify_with(category);
        }
    }

    fn propagate_references(&mut self) {
        let mut track_external_type = |type_name: &'a str| {
            let next_index = self.external_type_names.len();
            self.external_type_names.entry(type_name).or_insert(next_index);
        };

        // propagate references from commands to types
        for cmd_info in self
            .registry
            .iter_commands()
            .filter_map(|cmd| self.command_info_by_name.get(cmd.spec_name()))
            .filter(|&cmd_info| cmd_info.cmd.alias_of().is_none())
        {
            let cmd_def = match cmd_info.cmd {
                vk::Command::Definition(cmd_def) => cmd_def,
                _ => panic!("cannot guess category for command alias"),
            };
            for param in cmd_def.params.iter().filter(|param| param.is_vulkan_api()) {
                let decl = parse_variable_decl(&param.definition.code);
                if let CBaseType::Named(type_name) = decl.ty.base {
                    if let Some(type_info) = self.type_info_by_name.get_mut(type_name) {
                        type_info.is_referenced = true;
                    } else {
                        track_external_type(type_name);
                    }
                }
            }
        }

        // propagate references between types
        let mut visited = HashSet::new();
        let mut stack: Vec<_> = self
            .registry
            .iter_types()
            .filter_map(|ty| self.type_info_by_name.get(ty.spec_name()))
            .filter(|type_info| type_info.is_referenced)
            .map(|type_info| type_info.ty.spec_name())
            .collect();
        while let Some(type_name) = stack.pop() {
            if visited.insert(type_name) {
                // mark as referenced
                let type_info = self.type_info_by_name.get_mut(type_name).unwrap();
                type_info.is_referenced = true;

                // push dependencies onto the stack
                match &type_info.detail {
                    TypeInfoDetail::Alias(alias_type_name) => {
                        stack.push(alias_type_name);
                    }
                    TypeInfoDetail::BaseType(_) => {}

                    TypeInfoDetail::Bitmask(_) => {}
                    TypeInfoDetail::Enum(enum_type_info) => {
                        if let Some(bitmask_type_name) = enum_type_info.bitmask_type_name {
                            stack.push(bitmask_type_name);
                        }
                    }
                    TypeInfoDetail::Handle => {}
                    TypeInfoDetail::FunctionPointer => {
                        let code = type_info.ty.spec.function_pointer_code();
                        let decl = parse_func_pointer_typedef(code);
                        for decl in iter::once(&decl.proto).chain(&decl.parameters) {
                            if let CBaseType::Named(member_type_name) = decl.ty.base {
                                if self.type_info_by_name.contains_key(member_type_name) {
                                    stack.push(member_type_name);
                                } else {
                                    track_external_type(member_type_name);
                                }
                            }
                        }
                    }
                    TypeInfoDetail::Aggregate(struct_type_info) => {
                        for def in struct_type_info.iter_member_definitions() {
                            let decl = parse_variable_decl(def.code.as_str());
                            if let CBaseType::Named(member_type_name) = decl.ty.base {
                                if self.type_info_by_name.contains_key(member_type_name) {
                                    stack.push(member_type_name);
                                } else {
                                    track_external_type(member_type_name);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn move_enum_references_to_bitmasks(&mut self) {
        let mut to_ref = Vec::new();
        let mut to_deref = Vec::new();
        for type_info in self
            .type_info_by_name
            .values()
            .filter(|type_info| type_info.is_referenced)
        {
            if let Some(bitmask_type_name) = {
                let mut enum_type_info = type_info;
                loop {
                    match &enum_type_info.detail {
                        TypeInfoDetail::Alias(alias_type_name) => {
                            enum_type_info = &self.type_info_by_name[alias_type_name]
                        }
                        TypeInfoDetail::Enum(enum_type_info) => break enum_type_info.bitmask_type_name,
                        _ => break None,
                    }
                }
            } {
                to_ref.push(bitmask_type_name);
                to_deref.push(type_info.ty.spec_name());
            }
        }
        for name in to_ref.drain(..) {
            self.type_info_by_name.get_mut(name).unwrap().is_referenced = true;
        }
        for name in to_deref.drain(..) {
            self.type_info_by_name.get_mut(name).unwrap().is_referenced = false;
        }
    }

    fn iter_referenced_type_info(&self) -> impl Iterator<Item = &TypeInfo<'a>> {
        self.registry
            .iter_types()
            .filter_map(|ty| self.type_info_by_name.get(ty.spec_name()))
            .filter(|type_info| type_info.is_referenced)
    }

    fn iter_referenced_command_info(&self) -> impl Iterator<Item = &CommandInfo<'a>> {
        self.registry
            .iter_commands()
            .filter_map(|cmd| self.command_info_by_name.get(cmd.spec_name()))
    }

    fn iter_referenced_enum_info(&self) -> impl Iterator<Item = &EnumInfo<'a>> {
        self.all_enum
            .iter()
            .filter_map(|en| self.enum_info_by_name.get(en.spec_name()))
            .filter(|enum_info| match &enum_info.detail {
                EnumInfoDetail::Typed(typed_enum_info) => {
                    let mut type_info = &self.type_info_by_name[typed_enum_info.enum_type_name];
                    if let Some(bitmask_type_name) = type_info.detail.get_enum().unwrap().bitmask_type_name {
                        type_info = &self.type_info_by_name[bitmask_type_name];
                    }
                    type_info.is_referenced
                }
                EnumInfoDetail::Global(global_enum_info) => global_enum_info.is_referenced,
            })
    }

    fn get_enum_short_name(&self, value_name: &'a str, type_name: &'a str, is_bitmask: bool) -> String {
        let mut type_name = type_name.to_owned();
        if is_bitmask {
            // HACK: replace FlagBits2 with _2_ to match prefix of the enum name
            if let Some(index) = type_name.find("FlagBits2") {
                type_name = format!("{}_2_{}", &type_name[..index], &type_name[(index + 9)..]);
            }
        }
        let shouty_type_name = type_name.to_shouty_snake_case();

        let mut name_parts: Vec<&str> = value_name
            .split('_')
            .zip(shouty_type_name.split('_').chain(iter::repeat("")))
            .skip_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();

        let tag_match = name_parts
            .last()
            .copied()
            .filter(|&tag| self.tags_by_name.contains_key(tag));
        if tag_match.is_some() {
            name_parts.pop();
        }
        if is_bitmask && name_parts.last() == Some(&"BIT") {
            name_parts.pop();
        }
        if let Some(tag) = tag_match {
            if !shouty_type_name.ends_with(tag) {
                name_parts.push(tag)
            }
        }

        name_parts.join("_").to_lowercase()
    }

    fn get_enum_value(&self, en: &'a vk::Enum, bitmask_width: Option<BitWidth>, ext_name: Option<&'a str>) -> Literal {
        match &en.spec {
            vk::EnumSpec::Alias { alias, .. } => {
                self.get_enum_value(self.enum_info_by_name[alias.as_str()].en, bitmask_width, ext_name)
            }
            vk::EnumSpec::Bitpos { bitpos, .. } => match bitmask_width.unwrap() {
                BitWidth::U32 => Literal::U32(1 << bitpos),
                BitWidth::U64 => Literal::U64(1 << bitpos),
            },
            vk::EnumSpec::Value { value, .. } => {
                let cexpr = parse_constant_expr(value.as_str());
                match bitmask_width {
                    Some(BitWidth::U32) => Literal::U32(cexpr.try_into().unwrap()),
                    Some(BitWidth::U64) => Literal::U64(cexpr.try_into().unwrap()),
                    None => match cexpr {
                        CConstant::AnyInt(n) => Literal::Int(n),
                        CConstant::UInt32(n) => Literal::U32(n),
                        CConstant::UInt64(n) => Literal::U64(n),
                        CConstant::Float(n) => Literal::F32(n),
                    },
                }
            }
            vk::EnumSpec::Offset {
                offset, extnumber, dir, ..
            } => {
                let num =
                    extnumber.unwrap_or_else(|| self.extension_info_by_name[ext_name.unwrap()].ext.number.unwrap());
                let value = 1_000_000_000 + (num - 1) * 1000 + offset;
                let value = if *dir { value } else { -value };
                Literal::Int(value as isize)
            }
            _ => unimplemented!(),
        }
    }

    fn type_decl_from_parsed(&self, ty: &CType<'a>, index_maps: &IndexMaps<'a>) -> TypeDecl {
        // convert base type
        let mut type_decl = match ty.base {
            CBaseType::BuiltIn(built_in) => TypeDecl::BuiltIn(built_in),
            CBaseType::Named(name) => {
                // ensure we use the bitmask type instead of the bitmask value type
                let mut name = name;
                if let Some(enum_type_info) = self
                    .type_info_by_name
                    .get(name)
                    .and_then(|type_info| type_info.detail.get_enum())
                {
                    if let Some(bitmask_type_name) = enum_type_info.bitmask_type_name {
                        name = bitmask_type_name;
                    }
                }
                if let Some(type_index) = index_maps.type_index_by_name.get(name).copied() {
                    TypeDecl::Type(type_index)
                } else {
                    panic!("uknown type name {name}")
                }
            }
        };

        // handle inner level indirection
        if let Some(is_const) = match ty.decoration {
            CDecoration::None | CDecoration::Const => None,
            CDecoration::Pointer | CDecoration::PointerToPointer => Some(false),
            CDecoration::PointerToConst | CDecoration::PointerToConstPointerToConst => Some(true),
        } {
            type_decl = TypeDecl::Pointer(PointerDecl {
                is_const,
                array_hint: None,
                element_type: Box::new(type_decl),
            });
        }
        if let Some(is_const) = match ty.decoration {
            CDecoration::None | CDecoration::Const | CDecoration::Pointer | CDecoration::PointerToConst => None,
            CDecoration::PointerToPointer => Some(false),
            CDecoration::PointerToConstPointerToConst => Some(true),
        } {
            type_decl = TypeDecl::Pointer(PointerDecl {
                is_const,
                array_hint: None,
                element_type: Box::new(type_decl),
            });
        }

        // handle array indirection
        if let Some(array_size) = ty.array_size {
            type_decl = TypeDecl::Array(ArrayDecl {
                array: ArrayInfo {
                    size: match array_size {
                        CArraySize::Literal(n) => ArraySize::Literal(Literal::Int(n as isize)),
                        CArraySize::Ident(s) => ArraySize::Constant(index_maps.constant_index_by_name[s]),
                    },
                    is_null_terminated: false,
                },
                is_const: matches!(ty.decoration, CDecoration::Const),
                element_type: Box::new(type_decl),
            });
        }

        type_decl
    }

    fn match_version_by_name(&self, spec_name: &'a str, type_decl: &mut TypeDecl, index_maps: &IndexMaps<'a>) {
        match spec_name {
            "apiVersion" => {
                if matches!(type_decl, TypeDecl::BuiltIn(BuiltInDecl::U32)) {
                    *type_decl = TypeDecl::Type(index_maps.type_index_by_name["Version"]);
                }
            }
            "pApiVersion" => {
                if let TypeDecl::Pointer(pointer_decl) = type_decl {
                    if matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::U32)) {
                        *pointer_decl.element_type.as_mut() = TypeDecl::Type(index_maps.type_index_by_name["Version"]);
                    }
                }
            }
            _ => {}
        }
    }

    fn parameter_from_parsed(&self, decl: &CVariableDecl<'a>, index_maps: &IndexMaps<'a>) -> ParameterDecl {
        let mut result = ParameterDecl {
            name: decl.name.to_dim_separated().to_snake_case(),
            ty: self.type_decl_from_parsed(&decl.ty, index_maps).array_as_pointer(),
            transform: ParameterTransform::None,
            is_optional: false,
        };
        self.match_version_by_name(decl.name, &mut result.ty, index_maps);
        result
    }

    fn make_index_maps(&self, external_types: &[&'a str]) -> IndexMaps<'a> {
        let mut index_maps = IndexMaps::default();

        for enum_info in self.iter_referenced_enum_info() {
            let index = ConstantIndex::from_position(index_maps.constant_index_by_name.len());
            index_maps
                .constant_index_by_name
                .insert(enum_info.en.spec_name(), index);
        }

        for name in external_types.iter().copied().chain(
            self.iter_referenced_type_info()
                .map(|type_info| type_info.ty.spec_name()),
        ) {
            let index = TypeIndex::from_position(index_maps.type_index_by_name.len());
            index_maps.type_index_by_name.insert(name, index);
        }

        for ext in self.registry.iter_extensions() {
            let index = ExtensionIndex::from_position(index_maps.extension_index_by_name.len());
            index_maps.extension_index_by_name.insert(ext.spec_name(), index);
        }

        for cmd_info in self.iter_referenced_command_info() {
            let index = CommandIndex::from_position(index_maps.command_index_by_name.len());
            index_maps.command_index_by_name.insert(cmd_info.cmd.spec_name(), index);
        }

        index_maps
    }

    pub fn build(self) -> Oracle {
        let mut external_types: Vec<_> = self.external_type_names.iter().map(|(&k, &v)| (v, k)).collect();
        external_types.push((external_types.len(), "Version")); // extra Version type
        external_types.sort();
        let external_types: Vec<_> = external_types.iter().copied().map(|(_, name)| name).collect();

        let index_maps = self.make_index_maps(&external_types);

        let mut types = Vec::new();
        for &name in &external_types {
            let opaque_ptr = || {
                TypeDecl::Pointer(PointerDecl {
                    is_const: false,
                    array_hint: None,
                    element_type: Box::new(TypeDecl::External(ExternalDecl::Opaque)),
                })
            };
            let decl = match name {
                // X11
                "Display" => TypeDecl::External(ExternalDecl::Opaque),
                "VisualID" => TypeDecl::External(ExternalDecl::CULong),
                "Window" => TypeDecl::External(ExternalDecl::CULong),
                "RROutput" => TypeDecl::External(ExternalDecl::CULong),
                // wayland
                "wl_display" => TypeDecl::External(ExternalDecl::Opaque),
                "wl_surface" => TypeDecl::External(ExternalDecl::Opaque),
                // Windows
                "HINSTANCE" => opaque_ptr(),
                "HWND" => opaque_ptr(),
                "HANDLE" => opaque_ptr(),
                "HMONITOR" => opaque_ptr(),
                "SECURITY_ATTRIBUTES" => TypeDecl::External(ExternalDecl::Opaque),
                "DWORD" => TypeDecl::External(ExternalDecl::CULong),
                "LPCWSTR" => TypeDecl::Pointer(PointerDecl {
                    is_const: true,
                    array_hint: None,
                    element_type: Box::new(TypeDecl::BuiltIn(BuiltInDecl::U16)),
                }),
                // xcb
                "xcb_connection_t" => TypeDecl::External(ExternalDecl::Opaque),
                "xcb_window_t" => TypeDecl::BuiltIn(BuiltInDecl::U32),
                "xcb_visualid_t" => TypeDecl::BuiltIn(BuiltInDecl::U32),
                // DirectFB
                "IDirectFB" => TypeDecl::External(ExternalDecl::Opaque),
                "IDirectFBSurface" => TypeDecl::External(ExternalDecl::Opaque),
                // Android
                "ANativeWindow" => TypeDecl::External(ExternalDecl::Opaque),
                "AHardwareBuffer" => TypeDecl::External(ExternalDecl::Opaque),
                // Metal
                "CAMetalLayer" => TypeDecl::External(ExternalDecl::Opaque),
                "MTLDevice_id" => opaque_ptr(),
                "MTLCommandQueue_id" => opaque_ptr(),
                "MTLBuffer_id" => opaque_ptr(),
                "MTLTexture_id" => opaque_ptr(),
                "MTLSharedEvent_id" => opaque_ptr(),
                "IOSurfaceRef" => opaque_ptr(),
                // Zircon
                "zx_handle_t" => TypeDecl::BuiltIn(BuiltInDecl::U32),
                // Open Harmony OS
                "OHNativeWindow" => TypeDecl::External(ExternalDecl::Opaque),
                "OHBufferHandle" => TypeDecl::External(ExternalDecl::Opaque),
                "OH_NativeBuffer" => TypeDecl::External(ExternalDecl::Opaque),
                // Extra
                "Version" => TypeDecl::BuiltIn(BuiltInDecl::U32),
                _ => panic!("unknown external type: {name}"),
            };

            let index = index_maps.type_index_by_name[name];
            types.push(Type {
                index,
                spec_name: name.to_owned(),
                short_name: name.to_owned(),
                detail: TypeDetail::External(decl),
            });
        }
        for type_info in self.iter_referenced_type_info() {
            let spec_name = type_info.ty.spec_name();
            let index = index_maps.type_index_by_name[spec_name];
            match &type_info.detail {
                TypeInfoDetail::Alias(alias_type_name) => {
                    types.push(Type {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: spec_name.to_short_name(),
                        detail: TypeDetail::Alias(TypeDecl::Type(
                            index_maps.type_index_by_name.get(alias_type_name).copied().unwrap(),
                        )),
                    });
                }
                TypeInfoDetail::BaseType(base_type_info) => types.push(Type {
                    index,
                    spec_name: spec_name.to_owned(),
                    short_name: spec_name.to_short_name(),
                    detail: { TypeDetail::Alias(self.type_decl_from_parsed(&base_type_info.decl.ty, &index_maps)) },
                }),
                TypeInfoDetail::Enum(enum_type_info) => {
                    let values = enum_type_info
                        .values
                        .iter()
                        .map(|s| index_maps.constant_index_by_name[s])
                        .collect();
                    types.push(Type {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: spec_name.to_short_name(),
                        detail: TypeDetail::Enum(EnumType {
                            bitmask_width: None,
                            values,
                        }),
                    });
                }
                TypeInfoDetail::Bitmask(bitmask_type_info) => {
                    let values = if let Some(values_type_name) = bitmask_type_info.values_type_name {
                        let values_type_info = &self.type_info_by_name[values_type_name];
                        values_type_info
                            .detail
                            .get_enum()
                            .unwrap()
                            .values
                            .iter()
                            .map(|s| index_maps.constant_index_by_name[s])
                            .collect()
                    } else {
                        Vec::new()
                    };
                    types.push(Type {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: spec_name.to_short_name(),
                        detail: TypeDetail::Enum(EnumType {
                            bitmask_width: Some(bitmask_type_info.bit_width),
                            values,
                        }),
                    });
                }
                TypeInfoDetail::FunctionPointer => {
                    let code = type_info.ty.spec.function_pointer_code();
                    let decl = parse_func_pointer_typedef(code);

                    let return_type = self.type_decl_from_parsed(&decl.proto.ty, &index_maps);
                    let parameters = decl
                        .parameters
                        .iter()
                        .map(|decl| self.parameter_from_parsed(decl, &index_maps))
                        .collect();

                    types.push(Type {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: spec_name.to_short_name(),
                        detail: TypeDetail::FunctionPointer(FunctionPointerType {
                            return_type,
                            parameters,
                        }),
                    });
                }
                TypeInfoDetail::Aggregate(aggregate_type_info) => {
                    // collect struct members
                    let mut member_info: Vec<_> = aggregate_type_info
                        .iter_member_definitions()
                        .map(|def| {
                            let decl = parse_variable_decl(def.code.as_str());
                            MemberInfo {
                                def,
                                decl,
                                name: decl.name.to_owned(),
                            }
                        })
                        .collect();

                    // merge bitfield members into non-bitfield types
                    if !aggregate_type_info.is_union {
                        while let Some(member_index) = member_info
                            .iter()
                            .enumerate()
                            .find(|(_, member)| member.decl.ty.bit_count.is_some())
                            .map(|(i, _)| i)
                        {
                            // assume we can merge with the next member
                            let temp = member_info.remove(member_index + 1);
                            member_info[member_index].merge_with(temp);
                        }
                    }

                    let mut members: Vec<_> = member_info
                        .iter()
                        .map(|member_info| {
                            let mut type_decl = self.type_decl_from_parsed(&member_info.decl.ty, &index_maps);

                            // match u32 to Version
                            self.match_version_by_name(&member_info.name, &mut type_decl, &index_maps);

                            // apply length hints to types
                            if let Some(len) = member_info.def.len.as_deref() {
                                type_decl.apply_len_terms(len, member_info.def.altlen.as_deref());
                            }

                            let default = member_info
                                .def
                                .values
                                .as_deref()
                                .map(|s| index_maps.constant_index_by_name.get(s).copied().unwrap());

                            // parse optionality
                            let is_optional = member_info
                                .def
                                .optional
                                .as_deref()
                                .map(|s| s.starts_with("true"))
                                .unwrap_or(false);

                            MemberDecl {
                                spec_name: member_info.name.clone(),
                                short_name: member_info.name.to_dim_separated().to_snake_case(),
                                ty: type_decl,
                                default,
                                is_optional,
                                setter_transform: ParameterTransform::None,
                            }
                        })
                        .collect();

                    let mut transforms_temp = Vec::new();
                    for _ in 0..members.len() {
                        transforms_temp.push(ParameterTransform::None);
                    }

                    for (member_index, (member, info)) in members.iter().zip(member_info.iter()).enumerate() {
                        // skip members with constant value
                        if member.default.is_some() {
                            continue;
                        }

                        // match bool
                        if matches!(&info.decl.ty.base, CBaseType::Named("VkBool32"))
                            && info.decl.ty.decoration == CDecoration::None
                            && info.decl.ty.array_size.is_none()
                        {
                            transforms_temp[member_index] = ParameterTransform::FromBool;
                            continue;
                        }

                        // match pointers to slices
                        if let TypeDecl::Pointer(pointer_decl) = &member.ty {
                            let slice_element_is_sized =
                                !matches!(*pointer_decl.element_type, TypeDecl::BuiltIn(BuiltInDecl::Void));
                            if slice_element_is_sized || info.def.noautovalidity.is_none() {
                                if let Some(ArraySize::Named(len_name)) =
                                    pointer_decl.array_hint.as_ref().map(|array| &array.size)
                                {
                                    let Some(len_member_index) = members
                                        .iter()
                                        .enumerate()
                                        .find_map(|(index, member)| (member.short_name == *len_name).then_some(index))
                                    else {
                                        panic!("failed to match length {len_name}");
                                    };
                                    let kind = if info.def.noautovalidity.as_deref() == Some("true") {
                                        SliceParameterKind::Separate
                                    } else if member.is_optional {
                                        SliceParameterKind::Optional
                                    } else {
                                        SliceParameterKind::Required
                                    };
                                    transforms_temp[member_index] =
                                        ParameterTransform::FromSlice(SliceParameterTransform { length_check: None });
                                    transforms_temp[len_member_index].add_slice_length(member_index, kind);
                                }
                            }
                        }
                    }

                    for (member, transform) in members.iter_mut().zip(transforms_temp.drain(..)) {
                        member.setter_transform = transform;
                    }

                    types.push(Type {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: spec_name.to_short_name(),
                        detail: TypeDetail::Aggregate(AggregateType {
                            is_union: aggregate_type_info.is_union,
                            members,
                            extends: type_info
                                .ty
                                .structextends
                                .as_deref()
                                .iter()
                                .flat_map(|s| {
                                    s.split(',')
                                        .filter_map(|name| index_maps.type_index_by_name.get(name).copied())
                                })
                                .collect(),
                            extended_by: aggregate_type_info
                                .extended_by
                                .iter()
                                .filter_map(|&name| index_maps.type_index_by_name.get(name).copied())
                                .collect(),
                            returned_only_hint: type_info
                                .ty
                                .returnedonly
                                .as_deref()
                                .map(|s| s == "true")
                                .unwrap_or(false),
                        }),
                    });
                }
                TypeInfoDetail::Handle => {
                    let vk::TypeSpec::Code(code) = &type_info.ty.spec else {
                        panic!("missing code for handle type");
                    };
                    let handle_type = code
                        .markup
                        .iter()
                        .filter_map(|markup| match markup {
                            vk::TypeCodeMarkup::Type(s) => match s.as_str() {
                                "VK_DEFINE_HANDLE" => Some(HandleType::USize),
                                "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => Some(HandleType::U64),
                                _ => panic!("unknown handle type"),
                            },
                            _ => None,
                        })
                        .collect_one()
                        .expect("missing handle type");
                    types.push(Type {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: spec_name.to_short_name(),
                        detail: TypeDetail::Handle(handle_type),
                    })
                }
            }
        }

        let mut constants = Vec::new();
        for enum_info in self.iter_referenced_enum_info() {
            let spec_name = enum_info.en.spec_name();
            let index = index_maps.constant_index_by_name[spec_name];
            match &enum_info.detail {
                EnumInfoDetail::Typed(typed_enum_info) => {
                    let mut enum_type_name_for_index = typed_enum_info.enum_type_name;
                    let mut bitmask_width = None;
                    if let Some(bitmask_type_name) = self.type_info_by_name[enum_type_name_for_index]
                        .detail
                        .get_enum()
                        .unwrap()
                        .bitmask_type_name
                    {
                        enum_type_name_for_index = bitmask_type_name;
                        bitmask_width = Some(
                            self.type_info_by_name[bitmask_type_name]
                                .detail
                                .get_bitmask()
                                .unwrap()
                                .bit_width,
                        );
                    }
                    let value = match enum_info.en.alias_of() {
                        Some(alias) => ConstantValue::Alias(index_maps.constant_index_by_name[alias]),
                        None => ConstantValue::Literal(self.get_enum_value(
                            enum_info.en,
                            bitmask_width,
                            typed_enum_info.ext_name,
                        )),
                    };
                    constants.push(Constant {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: self.get_enum_short_name(
                            spec_name,
                            typed_enum_info.enum_type_name,
                            bitmask_width.is_some(),
                        ),
                        enum_type_index: Some(index_maps.type_index_by_name[enum_type_name_for_index]),
                        value,
                    });
                }
                EnumInfoDetail::Global(_) => {
                    let value = match enum_info.en.alias_of() {
                        Some(alias) => ConstantValue::Alias(index_maps.constant_index_by_name[alias]),
                        None => ConstantValue::Literal(self.get_enum_value(enum_info.en, None, None)),
                    };
                    constants.push(Constant {
                        index,
                        spec_name: spec_name.to_owned(),
                        short_name: spec_name.to_short_name(),
                        enum_type_index: None,
                        value,
                    });
                }
            }
        }

        let mut extensions = Vec::new();
        for ext in self.registry.iter_extensions() {
            let spec_name = ext.spec_name();
            let ext_info = self.extension_info_by_name.get(spec_name).unwrap();
            extensions.push(Extension {
                index: index_maps.extension_index_by_name[spec_name],
                spec_name: spec_name.to_owned(),
                short_name: spec_name.to_short_name(),
                category: ext_info.category,
                promoted_to_version: ext_info.promoted_to_version,
                dependencies: ext_info
                    .dependencies
                    .map_extensions(|name| index_maps.extension_index_by_name[name]),
                has_instance_dependency: ext_info.has_instance_dependency,
            });
        }

        let mut commands = Vec::new();
        for cmd_info in self.iter_referenced_command_info() {
            let spec_name = cmd_info.cmd.spec_name();

            let category = cmd_info.category.unwrap();
            let dependencies = cmd_info
                .dependencies
                .map_extensions(|name| index_maps.extension_index_by_name[name]);

            let detail = if let Some(name) = cmd_info.cmd.alias_of() {
                CommandDetail::Alias(index_maps.command_index_by_name[name])
            } else {
                let cmd_def = match cmd_info.cmd {
                    vk::Command::Definition(cmd_def) => cmd_def,
                    _ => panic!("command is missing definition"),
                };

                let original_constant_index = |mut index: ConstantIndex| -> ConstantIndex {
                    while let ConstantValue::Alias(alias_index) = &constants[index].value {
                        index = *alias_index;
                    }
                    index
                };
                let find_constants = |s: Option<&str>| -> Vec<ConstantIndex> {
                    s.into_iter()
                        .flat_map(|s| s.split(','))
                        .filter_map(|s| index_maps.constant_index_by_name.get(s).copied())
                        .map(original_constant_index)
                        .collect()
                };
                let success_codes = find_constants(cmd_def.successcodes.as_deref());
                let success_is_enumerate_result = cmd_def
                    .successcodes
                    .as_deref()
                    .map(|s| s == "VK_SUCCESS,VK_INCOMPLETE")
                    .unwrap_or(false);

                let error_codes = find_constants(cmd_def.errorcodes.as_deref());

                let return_type = {
                    let decl = parse_variable_decl(&cmd_def.proto.code);
                    self.type_decl_from_parsed(&decl.ty, &index_maps)
                };
                let return_is_result = return_type
                    .type_index()
                    .map(|type_index| types[type_index].spec_name.as_str() == "VkResult")
                    .unwrap_or(false);
                if return_is_result == success_codes.is_empty() {
                    panic!("return type and success code mismatch: {spec_name}");
                }

                let is_returned_only = |type_decl: &TypeDecl| {
                    type_decl
                        .type_index()
                        .and_then(|index| match &types[index].detail {
                            TypeDetail::Aggregate(aggregrate_type) => Some(aggregrate_type),
                            _ => None,
                        })
                        .map(|aggregate_type| aggregate_type.returned_only_hint)
                        .unwrap_or(false)
                };

                let has_default = |type_decl: &TypeDecl| {
                    type_decl
                        .type_index()
                        .and_then(|index| match &types[index].detail {
                            TypeDetail::Aggregate(aggregrate_type) => Some(aggregrate_type),
                            _ => None,
                        })
                        .map(|aggregate_type| {
                            aggregate_type
                                .members
                                .iter()
                                .any(|member_decl| member_decl.default.is_some())
                        })
                        .unwrap_or(false)
                };

                let mut parameters: Vec<_> = cmd_def
                    .params
                    .iter()
                    .filter(|param| param.is_vulkan_api())
                    .map(|param| {
                        let decl = parse_variable_decl(&param.definition.code);
                        let mut result = self.parameter_from_parsed(&decl, &index_maps);
                        if let Some(len) = param.len.as_deref() {
                            result.ty.apply_len_terms(len, param.altlen.as_deref());
                        }
                        if param
                            .optional
                            .as_deref()
                            .map(|s| s.starts_with("true"))
                            .unwrap_or(false)
                        {
                            result.is_optional = true;
                        }
                        result
                    })
                    .collect();

                let mut transforms_temp = Vec::new();
                for _ in 0..parameters.len() {
                    transforms_temp.push(ParameterTransform::None);
                }

                let mut enumerate_output_transforms = Vec::new();
                for (param_index, param) in parameters.iter().enumerate() {
                    // match member handle on first parameter
                    if param_index == 0
                        && param
                            .ty
                            .type_index()
                            .and_then(|type_index| match types[type_index].spec_name.as_str() {
                                "VkInstance" => Some(CommandCategory::Instance),
                                "VkDevice" => Some(CommandCategory::Device),
                                _ => None,
                            })
                            .map(|type_category| type_category == category)
                            .unwrap_or(false)
                    {
                        transforms_temp[param_index] = ParameterTransform::FromMemberHandle;
                        continue;
                    }

                    // match bool
                    if param
                        .ty
                        .type_index()
                        .map(|type_index| types[type_index].spec_name.as_str() == "VkBool32")
                        .unwrap_or(false)
                    {
                        transforms_temp[param_index] = ParameterTransform::FromBool;
                        continue;
                    }

                    // match pointers to slices
                    if let TypeDecl::Pointer(pointer_decl) = &param.ty {
                        if let Some(ArraySize::Named(len_name)) =
                            pointer_decl.array_hint.as_ref().map(|array| &array.size)
                        {
                            if let Some(len_param_index) = parameters
                                .iter()
                                .enumerate()
                                .find_map(|(index, param)| (param.name == *len_name).then_some(index))
                            {
                                let length_is_pointer = matches!(parameters[len_param_index].ty, TypeDecl::Pointer(_));
                                if length_is_pointer {
                                    let slice_element_is_sized =
                                        !matches!(*pointer_decl.element_type, TypeDecl::BuiltIn(BuiltInDecl::Void));
                                    let slice_element_is_likely_output_only =
                                        is_returned_only(&pointer_decl.element_type)
                                            || !has_default(&pointer_decl.element_type);
                                    if slice_element_is_sized && slice_element_is_likely_output_only {
                                        enumerate_output_transforms.push(EnumerateOutputTransform {
                                            count_param: len_param_index,
                                            elements_param: param_index,
                                        });
                                    }
                                } else {
                                    transforms_temp[param_index] =
                                        ParameterTransform::FromSlice(SliceParameterTransform { length_check: None });
                                    let kind = if param.is_optional {
                                        SliceParameterKind::Optional
                                    } else {
                                        SliceParameterKind::Required
                                    };
                                    transforms_temp[len_param_index].add_slice_length(param_index, kind);
                                }
                            } else {
                                transforms_temp[param_index] = ParameterTransform::FromSlice(SliceParameterTransform {
                                    length_check: Some(len_name.clone()),
                                });
                            }
                        }
                    }
                }
                for (param, transform) in parameters.iter_mut().zip(transforms_temp.drain(..)) {
                    param.transform = transform;
                }

                let decl_is_bool32 = |decl: &TypeDecl| {
                    decl.type_index()
                        .map(|type_index| types[type_index].spec_name.as_str() == "VkBool32")
                        .unwrap_or(false)
                };
                let last_param_output_is_bool32 = parameters
                    .last()
                    .filter(|param| matches!(param.transform, ParameterTransform::None) && !param.is_optional)
                    .and_then(|param| match &param.ty {
                        TypeDecl::Pointer(pointer_decl) => Some(pointer_decl),
                        _ => None,
                    })
                    .filter(|&pointer_decl| !pointer_decl.is_const && pointer_decl.array_hint.is_none())
                    .filter(|&pointer_decl| {
                        !matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::Void))
                            && !has_default(&pointer_decl.element_type)
                    })
                    .map(|pointer_decl| decl_is_bool32(&pointer_decl.element_type));

                let output_transform = if return_is_result {
                    if enumerate_output_transforms.len() == 1 && success_is_enumerate_result {
                        CommandOutputTransform::IntoEnumerate(enumerate_output_transforms.pop().unwrap())
                    } else if let Some(into_bool) = last_param_output_is_bool32.filter(|_| success_codes.len() == 1) {
                        let last_param = parameters.last_mut().unwrap();
                        last_param.transform = ParameterTransform::FromOutput;
                        CommandOutputTransform::IntoObject(ObjectOutputTransform {
                            output_param: parameters.len() - 1,
                            into_bool,
                        })
                    } else {
                        CommandOutputTransform::None
                    }
                } else {
                    let return_is_bool32 = decl_is_bool32(&return_type);
                    let return_is_void = matches!(return_type, TypeDecl::BuiltIn(BuiltInDecl::Void));

                    if enumerate_output_transforms.len() == 1 && return_is_void {
                        CommandOutputTransform::IntoEnumerate(enumerate_output_transforms.pop().unwrap())
                    } else if return_is_bool32 {
                        CommandOutputTransform::IntoBool
                    } else if let Some(into_bool) = last_param_output_is_bool32.filter(|_| return_is_void) {
                        let last_param = parameters.last_mut().unwrap();
                        last_param.transform = ParameterTransform::FromOutput;
                        CommandOutputTransform::IntoObject(ObjectOutputTransform {
                            output_param: parameters.len() - 1,
                            into_bool,
                        })
                    } else {
                        CommandOutputTransform::None
                    }
                };

                let type_index = TypeIndex::from_position(types.len());
                types.push(Type {
                    index: type_index,
                    spec_name: spec_name.to_owned(),
                    short_name: spec_name.to_short_name(),
                    detail: TypeDetail::FunctionPointer(FunctionPointerType {
                        return_type,
                        parameters,
                    }),
                });

                let load_on_instance =
                    category == CommandCategory::Device && cmd_info.cmd.guess_category() == CommandCategory::Instance;

                let aliases = cmd_info
                    .aliases
                    .iter()
                    .map(|&name| index_maps.command_index_by_name[name])
                    .collect();

                CommandDetail::Function(CommandFunction {
                    load_on_instance,
                    function_type_index: type_index,
                    aliases,
                    success_codes,
                    error_codes,
                    output_transform,
                })
            };

            commands.push(Command {
                index: index_maps.command_index_by_name[spec_name],
                spec_name: spec_name.to_owned(),
                short_name: spec_name.to_short_name(),
                category,
                dependencies,
                detail,
            });
        }

        Oracle {
            header_version: (self.vulkan_version.0, self.vulkan_version.1, self.header_version),
            types,
            constants,
            extensions,
            commands,
        }
    }
}
