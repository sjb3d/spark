mod c_parse;

use crate::c_parse::*;
use heck::{ShoutySnakeCase, SnakeCase};
use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fmt;
use std::fmt::Write as FmtWrite;
use std::fs::File;
use std::io;
use std::io::Write as IoWrite;
use std::iter;
use std::path::Path;
use std::process::Command as Spawn;
use vk_parse as vk;

#[derive(Debug)]
enum Error {
    Io(io::Error),
    Fmt(fmt::Error),
    Parse(vk::FatalError),
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}
impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::Fmt(err)
    }
}
impl From<vk::FatalError> for Error {
    fn from(err: vk::FatalError) -> Self {
        Error::Parse(err)
    }
}

type WriteResult = Result<(), Error>;

trait CollectOne {
    type Item;
    fn collect_one(self) -> Option<Self::Item>;
}

impl<I: Iterator> CollectOne for I {
    type Item = I::Item;
    fn collect_one(mut self) -> Option<Self::Item> {
        match self.next() {
            Some(item) => {
                if self.next().is_some() {
                    None
                } else {
                    Some(item)
                }
            }
            None => None,
        }
    }
}

const TYPE_PREFIX: &str = "Vk";
const FN_PREFIX: &str = "vk";
const PFN_PREFIX: &str = "PFN_vk";
const CONST_PREFIX: &str = "VK_";

const NAMED_TYPE_VKBOOL32: CBaseType = CBaseType::Named("VkBool32");
const NAMED_TYPE_VKVERSION: CBaseType = CBaseType::Named("VkVersion");
const NAMED_TYPE_VKINSTANCE: CBaseType = CBaseType::Named("VkInstance");
const NAMED_TYPE_VKDEVICE: CBaseType = CBaseType::Named("VkDevice");

trait SkipPrefix {
    fn skip_prefix(self, prefix: &str) -> Self;
}

impl SkipPrefix for &str {
    fn skip_prefix(self, prefix: &str) -> Self {
        let len = prefix.len();
        if &self[0..len] != prefix {
            panic!("cannot remove prefix {} from {}", prefix, self);
        }
        &self[len..]
    }
}

impl SkipPrefix for CArraySize<'_> {
    fn skip_prefix(self, prefix: &str) -> Self {
        match self {
            CArraySize::Ident(s) => CArraySize::Ident(s.skip_prefix(prefix)),
            CArraySize::Literal(n) => CArraySize::Literal(n),
        }
    }
}

trait GetTypeName {
    fn get_type_name(&self) -> &str;
}

impl GetTypeName for vk::Type {
    fn get_type_name(&self) -> &str {
        if self.alias.is_some() {
            self.name.as_deref().expect("missing bitmask or enum alias type name")
        } else {
            match self.category.as_deref() {
                Some("basetype") | Some("bitmask") | Some("handle") | Some("funcpointer") => {
                    if let vk::TypeSpec::Code(ref code) = self.spec {
                        code.markup
                            .iter()
                            .filter_map(|markup| match markup {
                                vk::TypeCodeMarkup::Name(name) => Some(name.as_str()),
                                _ => None,
                            })
                            .collect_one()
                            .expect("missing bitmask or enum type name")
                    } else {
                        panic!("failed to get type name for {:?}", self)
                    }
                }
                Some("enum") | Some("struct") | Some("union") => self.name.as_deref().expect("missing struct name"),
                _ => panic!("cannot get type name for {:?}", self),
            }
        }
    }
}

trait GetBitmaskValueName {
    fn get_bitmask_value_name(&self) -> Option<String>;
}

impl GetBitmaskValueName for vk::Type {
    fn get_bitmask_value_name(&self) -> Option<String> {
        self.requires.clone().or_else(|| {
            // TODO: look for "bitvalues" attribute once supported by vk-parse
            // HACK: just replace "Flags" with "FlagBits" for now
            let type_name = self.get_type_name();
            type_name
                .find("Flags")
                .map(|offset| format!("{}FlagBits{}", &type_name[..offset], &type_name[(offset + 5)..]))
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Category {
    Loader,
    Instance,
    Device,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Category::Loader => write!(f, "Loader"),
            Category::Instance => write!(f, "Instance"),
            Category::Device => write!(f, "Device"),
        }
    }
}

trait ExtensionExtra {
    fn get_category(&self) -> Category;
    fn is_supported(&self) -> bool;
    fn is_blacklisted(&self) -> bool;
}

impl ExtensionExtra for vk::Extension {
    fn get_category(&self) -> Category {
        match self.ext_type.as_deref() {
            Some("instance") => Category::Instance,
            Some("device") => Category::Device,
            _ => panic!("unknown extension type {:?}", self),
        }
    }
    fn is_supported(&self) -> bool {
        self.supported.as_deref() == Some("vulkan")
    }
    fn is_blacklisted(&self) -> bool {
        matches!(self.author.as_deref(), Some("GGP") | Some("QNX"))
            || (self.provisional && self.name != "VK_KHR_portability_subset")
            || self.supported.as_deref() == Some("disabled")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CommandReturnValue {
    Void,
    Result,
    Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum BitMaskSize {
    N32,
    N64,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum EnumType {
    Bitmask(BitMaskSize),
    Value,
}

enum EnumEntryValue {
    Number { value: i64, comment: Option<String> },
    Alias(String),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum AggregateType {
    Struct,
    Union,
}

fn get_rust_variable_name(camel_case: &str) -> String {
    let var_name = camel_case.to_snake_case();
    match var_name.as_str() {
        "type" => "ty".to_owned(),
        _ => var_name,
    }
}

trait GetCommandCategory {
    fn get_command_category(&self) -> Category;
}

impl GetCommandCategory for vk::CommandDefinition {
    fn get_command_category(&self) -> Category {
        match self.proto.name.as_str() {
            "vkGetInstanceProcAddr"
            | "vkCreateInstance"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceExtensionProperties"
            | "vkEnumerateInstanceVersion" => Category::Loader,
            "vkGetDeviceProcAddr" => Category::Instance,
            _ => {
                let is_first_param_from_device = self
                    .params
                    .get(0)
                    .and_then(|param| param.definition.type_name.as_deref())
                    .map(|type_name| matches!(type_name, "VkDevice" | "VkCommandBuffer" | "VkQueue"))
                    .unwrap_or(false);
                if is_first_param_from_device {
                    Category::Device
                } else {
                    Category::Instance
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u16,
    minor: u16,
}

impl Version {
    const fn from_raw_parts(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }

    fn try_from_feature(s: &str) -> Option<Self> {
        c_try_parse_version(s).map(|(major, minor)| Version::from_raw_parts(major, minor))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CommandRef<'a> {
    Feature(Version),
    Extension(&'a str),
}

#[derive(Debug, Copy, Clone)]
struct CommandRefPair<'a> {
    primary: CommandRef<'a>,
    secondary: Option<CommandRef<'a>>,
}

impl<'a> CommandRefPair<'a> {
    fn is_core_vulkan_1_0(&self) -> bool {
        self.primary == CommandRef::Feature(Version::from_raw_parts(1, 0)) && self.secondary.is_none()
    }

    fn matches(&self, other: &CommandRefPair<'a>) -> bool {
        (self.primary == other.primary && self.secondary == other.secondary)
            || (Some(self.primary) == other.secondary && self.secondary == Some(other.primary))
    }
}

#[derive(Debug)]
struct CommandInfo<'a> {
    cmd_def: &'a vk::CommandDefinition,
    alias: Option<&'a str>,
    category: Option<Category>,
    refs: Vec<CommandRefPair<'a>>,
}

impl<'a> CommandInfo<'a> {
    fn is_core_vulkan_1_0(&self) -> bool {
        self.refs[0].is_core_vulkan_1_0()
    }
}

fn type_name_is_void(type_name: &str) -> bool {
    type_name == "c_void"
}

fn slice_type_name(type_name: &str) -> &str {
    if type_name_is_void(type_name) {
        "T"
    } else {
        type_name
    }
}

fn slice_as_ptr(name: &str, type_name: &str, is_mutable: bool, in_option: bool) -> String {
    if is_mutable {
        format!(
            "{}{}.map_or(ptr::null_mut(), |s| s as *mut _){}",
            name,
            if in_option {
                ".and_then(|s| s.first_mut())"
            } else {
                ".first_mut()"
            },
            if type_name_is_void(type_name) { "as *mut _" } else { "" }
        )
    } else {
        format!(
            "{}{}.map_or(ptr::null(), |s| s as *const _){}",
            name,
            if in_option {
                ".and_then(|s| s.first())"
            } else {
                ".first()"
            },
            if type_name_is_void(type_name) {
                "as *const _"
            } else {
                ""
            }
        )
    }
}

fn slice_len(name: &str, type_name: &str) -> String {
    if type_name_is_void(type_name) {
        format!("mem::size_of_val({})", name)
    } else {
        format!("{}.len()", name)
    }
}

struct StructMember<'a> {
    def: &'a vk::TypeMemberDefinition,
    decl: CVariableDecl<'a>,
    merged_name: String,
}

impl StructMember<'_> {
    fn name(&self) -> &str {
        if self.merged_name.is_empty() {
            self.decl.name
        } else {
            self.merged_name.as_str()
        }
    }

    fn merge_with(&mut self, other: StructMember) {
        let bit_count = self.decl.ty.bit_count.unwrap() + other.decl.ty.bit_count.unwrap();
        self.merged_name = format!("{}_and_{}", self.name(), other.name());
        if bit_count == 32 {
            self.decl.ty.base = CBaseType::U32;
            self.decl.ty.bit_count = None;
        } else {
            self.decl.ty.bit_count = Some(bit_count);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SliceInfo {
    name: String,
    type_name: String,
    is_mutable: bool,
    is_optional: bool,
}

impl SliceInfo {
    fn is_generic(&self) -> bool {
        type_name_is_void(&self.type_name)
    }

    fn get_type_name(&self) -> &str {
        slice_type_name(&self.type_name)
    }

    fn get_as_ptr(&self, in_option: bool) -> String {
        slice_as_ptr(&self.name, &self.type_name, self.is_mutable, in_option)
    }

    fn get_len(&self) -> String {
        slice_len(&self.name, &self.type_name)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LibParamType {
    CDecl,
    Bool,
    MemberHandle,
    CStr {
        is_optional: bool,
    },
    NonOptional {
        inner_type_name: String,
    },
    SliceLenShared {
        name: String,
        slice_infos: Vec<SliceInfo>,
    },
    SliceLenSingle {
        slice_infos: Vec<SliceInfo>,
    },
    Slice {
        inner_type_name: String,
        is_mutable: bool,
        is_optional: bool,
        len_expr: Option<String>,
    },
    Ref {
        inner_type_name: String,
        is_optional: bool,
    },
    MutRef {
        inner_type_name: String,
    },
    ReturnObject {
        inner_type_name: String,
    },
    ReturnVecLenShared,
    ReturnVecLenSingle {
        slice_name: String,
    },
    ReturnVec {
        inner_type_name: String,
    },
}

#[derive(Debug)]
struct LibParam {
    name: String,
    ty: LibParamType,
}

#[derive(Debug, PartialEq, Eq)]
enum LibReturnType {
    CDecl,
    None,
    Object,
    VecUnknownLen,
    ResultEmpty,
    ResultEnum,
    ResultEnumAndObject,
    ResultObject,
    ResultVecUnknownLen,
    ResultVecKnownLen { len_expr: String },
    ResultMultiVecUnknownLen,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LibReturnTransform {
    None,
    ToInstance,
    ToDevice,
    ToBool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LibCommandStyle {
    Default,
    ToVecUnknownLen,
    ToVecKnownLen,
    Array,
    Single,
}

struct Generator<'a> {
    registry: &'a vk::Registry,
    extension_by_name: HashMap<&'a str, &'a vk::Extension>,
    type_by_name: HashMap<&'a str, &'a vk::Type>,
    used_type_names: HashSet<&'a str>,
    type_name_blacklist: HashSet<&'a str>,
    tag_names: HashSet<&'a str>,
    bitmask_from_value: HashMap<String, &'a str>,
    enums_by_name: HashMap<&'a str, Vec<&'a vk::Enum>>,
    constant_enums: Vec<&'a vk::Enum>,
    extension_by_enum_name: HashMap<&'a str, &'a vk::Extension>,
    cmd_names: Vec<&'a str>,
    cmd_info_by_name: HashMap<&'a str, CommandInfo<'a>>,
}

impl<'a> Generator<'a> {
    fn get_type_iterator(&self) -> impl Iterator<Item = &'a vk::Type> {
        self.registry
            .0
            .iter()
            .filter_map(|registry_child| match registry_child {
                vk::RegistryChild::Types(types) => Some(types),
                _ => None,
            })
            .flat_map(|types| types.children.iter())
            .filter_map(|types_child| match types_child {
                vk::TypesChild::Type(ty) => Some(ty),
                _ => None,
            })
    }

    fn collect_extensions(&mut self) {
        for registry_child in &self.registry.0 {
            if let vk::RegistryChild::Extensions(extensions) = registry_child {
                for ext in &extensions.children {
                    self.extension_by_name.insert(&ext.name, ext);
                }
            }
        }
    }

    fn check_type_derives(&self, base_type: CBaseType) -> bool {
        match base_type {
            CBaseType::Void
            | CBaseType::Char
            | CBaseType::Int
            | CBaseType::U8
            | CBaseType::U16
            | CBaseType::U32
            | CBaseType::U64
            | CBaseType::I8
            | CBaseType::I16
            | CBaseType::I32
            | CBaseType::I64
            | CBaseType::USize => true,
            CBaseType::F32 | CBaseType::F64 => false,
            CBaseType::Named(type_name) => {
                if let Some(ty) = self.type_by_name.get(type_name) {
                    match ty.category.as_deref() {
                        Some("basetype") | Some("bitmask") | Some("enum") => true,
                        Some("struct") => {
                            if let vk::TypeSpec::Members(ref members) = ty.spec {
                                members
                                    .iter()
                                    .filter_map(|member| match member {
                                        vk::TypeMember::Definition(ref member_def) => Some(member_def),
                                        _ => None,
                                    })
                                    .map(|member_def| c_parse_variable_decl(member_def.code.as_str()))
                                    .all(|decl| {
                                        decl.ty.array_size.is_none()
                                            && decl.ty.decoration == CDecoration::None
                                            && self.check_type_derives(decl.ty.base)
                                    })
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
        }
    }

    fn collect_types(&mut self) {
        for ty in self.get_type_iterator() {
            let category = ty.category.as_deref();
            if let Some("basetype") | Some("bitmask") | Some("enum") | Some("handle") | Some("funcpointer")
            | Some("struct") | Some("union") = category
            {
                let name = ty.get_type_name();
                if self.type_by_name.insert(name, ty).is_some() {
                    panic!("duplicate type name from {:?}", ty)
                }
                if let Some("bitmask") = category {
                    if let Some(value) = ty.get_bitmask_value_name() {
                        if self.bitmask_from_value.insert(value, name).is_some() {
                            panic!("duplicate value for bitmask {}", name);
                        }
                    }
                }
                if let Some("funcpointer") = category {
                    if let vk::TypeSpec::Code(ref code) = ty.spec {
                        let decl = c_parse_func_pointer_typedef(code.code.as_str());
                        if let CBaseType::Named(name) = decl.proto.ty.base {
                            self.used_type_names.insert(name);
                        }
                        for param in decl.parameters.iter() {
                            if let CBaseType::Named(name) = param.ty.base {
                                self.used_type_names.insert(name);
                            }
                        }
                    }
                }
                if let Some("struct") | Some("union") = category {
                    if let vk::TypeSpec::Members(ref members) = ty.spec {
                        for member_def in members.iter().filter_map(|member| match member {
                            vk::TypeMember::Definition(ref member_def) => Some(member_def),
                            _ => None,
                        }) {
                            let decl = c_parse_variable_decl(member_def.code.as_str());
                            if let CBaseType::Named(name) = decl.ty.base {
                                self.used_type_names.insert(name);
                            }
                        }
                    }
                }
            }
        }
        for registry_child in &self.registry.0 {
            if let vk::RegistryChild::Extensions(extensions) = registry_child {
                for ext in extensions.children.iter().filter(|ext| ext.is_blacklisted()) {
                    for item in ext
                        .children
                        .iter()
                        .filter_map(|ext_child| match ext_child {
                            vk::ExtensionChild::Require { items, .. } => Some(items),
                            _ => None,
                        })
                        .flat_map(|items| items.iter())
                    {
                        if let vk::InterfaceItem::Type { name, .. } = item {
                            self.type_name_blacklist.insert(name.as_str());
                        }
                    }
                }
            }
        }
        for name in self.type_name_blacklist.iter() {
            println!("Blacklisted: {}", name);
        }
    }

    fn collect_tags(&mut self) {
        for tag in self
            .registry
            .0
            .iter()
            .filter_map(|registry_child| match registry_child {
                vk::RegistryChild::Tags(tags) => Some(tags),
                _ => None,
            })
            .flat_map(|types| types.children.iter())
        {
            if !self.tag_names.insert(tag.name.as_str()) {
                panic!("duplicate tag name from {:?}", tag);
            }
        }
    }

    fn collect_extension_enum(&mut self, en: &'a vk::Enum) {
        let extends = match en.spec {
            vk::EnumSpec::Alias { ref extends, .. } => extends.as_deref(),
            vk::EnumSpec::Offset { ref extends, .. } => Some(extends.as_str()),
            vk::EnumSpec::Bitpos { ref extends, .. } => extends.as_deref(),
            vk::EnumSpec::Value { ref extends, .. } => extends.as_deref(),
            vk::EnumSpec::None => None,
            _ => panic!("enum spec type not handled"),
        };
        if let Some(name) = extends {
            let enums = self.enums_by_name.get_mut(name).expect("missing enum to extend");
            if enums.iter().any(|e| e.name == en.name) {
                println!("Ignoring duplicated enum {}", en.name);
            } else {
                enums.push(en);
            }
        }
    }

    fn collect_enums(&mut self) {
        for registry_child in &self.registry.0 {
            match registry_child {
                vk::RegistryChild::Enums(enums) => match enums.kind.as_deref() {
                    Some("enum") | Some("bitmask") => {
                        let name = enums.name.as_deref().expect("missing enum name");
                        let enums = enums
                            .children
                            .iter()
                            .filter_map(|enums_child| match enums_child {
                                vk::EnumsChild::Enum(en) => Some(en),
                                _ => None,
                            })
                            .collect();
                        if self.enums_by_name.insert(name, enums).is_some() {
                            panic!("duplicate enum name {}", name);
                        }
                    }
                    _ => {
                        self.constant_enums
                            .extend(enums.children.iter().filter_map(|enums_child| match enums_child {
                                vk::EnumsChild::Enum(en) => Some(en),
                                _ => None,
                            }));
                    }
                },
                vk::RegistryChild::Feature(feature) => {
                    for item in feature
                        .children
                        .iter()
                        .filter_map(|ext_child| match ext_child {
                            vk::ExtensionChild::Require { items, .. } => Some(items),
                            _ => None,
                        })
                        .flat_map(|items| items.iter())
                    {
                        match item {
                            vk::InterfaceItem::Type { name, .. } => {
                                self.used_type_names.insert(name.as_str());
                            }
                            vk::InterfaceItem::Enum(en) => {
                                self.collect_extension_enum(en);
                            }
                            _ => {}
                        }
                    }
                }
                vk::RegistryChild::Extensions(extensions) => {
                    for ext in extensions.children.iter().filter(|ext| !ext.is_blacklisted()) {
                        for item in ext
                            .children
                            .iter()
                            .filter_map(|ext_child| match ext_child {
                                vk::ExtensionChild::Require { items, .. } => Some(items),
                                _ => None,
                            })
                            .flat_map(|items| items.iter())
                        {
                            match item {
                                vk::InterfaceItem::Type { name, .. } => {
                                    self.used_type_names.insert(name.as_str());
                                }
                                vk::InterfaceItem::Enum(en) => {
                                    self.collect_extension_enum(en);
                                    self.extension_by_enum_name.insert(en.name.as_str(), ext);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn collect_commands(&mut self) {
        let mut cmd_aliases: Vec<(&str, &str)> = Vec::new();
        for registry_child in &self.registry.0 {
            if let vk::RegistryChild::Commands(commands) = registry_child {
                for cmd_child in &commands.children {
                    match cmd_child {
                        vk::Command::Alias { name, alias } => {
                            self.cmd_names.push(name.as_str());
                            cmd_aliases.push((name.as_str(), alias.as_str()));
                        }
                        vk::Command::Definition(cmd_def) => {
                            self.cmd_names.push(cmd_def.proto.name.as_str());
                            self.cmd_info_by_name.insert(
                                cmd_def.proto.name.as_str(),
                                CommandInfo {
                                    cmd_def,
                                    alias: None,
                                    category: None,
                                    refs: Vec::new(),
                                },
                            );
                            let decl = c_parse_function_decl(cmd_def.code.as_str());
                            if let CBaseType::Named(name) = decl.proto.ty.base {
                                self.used_type_names.insert(name);
                            }
                            for param in decl.parameters.iter() {
                                if let CBaseType::Named(name) = param.ty.base {
                                    self.used_type_names.insert(name);
                                }
                            }
                        }
                        _ => panic!("command type not handled"),
                    }
                }
            }
        }
        for (name, alias) in &cmd_aliases {
            let cmd_def = self
                .cmd_info_by_name
                .get(alias)
                .expect("command alias not found")
                .cmd_def;
            self.cmd_info_by_name.insert(
                name,
                CommandInfo {
                    cmd_def,
                    alias: Some(alias),
                    category: None,
                    refs: Vec::new(),
                },
            );
        }
        for registry_child in &self.registry.0 {
            match registry_child {
                vk::RegistryChild::Feature(feature) => {
                    for name in feature
                        .children
                        .iter()
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
                        let info = self.cmd_info_by_name.get_mut(name).expect("missing command info");

                        let cmd_category = Some(info.cmd_def.get_command_category());
                        info.category = info.category.or(cmd_category);
                        assert_eq!(info.category, cmd_category);

                        info.refs.push(CommandRefPair {
                            primary: CommandRef::Feature(Version::try_from_feature(&feature.name).unwrap()),
                            secondary: None,
                        });
                    }
                }
                vk::RegistryChild::Extensions(extensions) => {
                    for ext in extensions
                        .children
                        .iter()
                        .filter(|ext| ext.is_supported() && !ext.is_blacklisted())
                    {
                        let ext_category = Some(ext.get_category());

                        for (cmd_ref, items) in ext.children.iter().filter_map(|ext_child| match ext_child {
                            vk::ExtensionChild::Require {
                                feature,
                                extension,
                                items,
                                ..
                            } => {
                                let cmd_ref = feature
                                    .as_deref()
                                    .and_then(Version::try_from_feature)
                                    .map(CommandRef::Feature)
                                    .or_else(|| extension.as_deref().map(|s| CommandRef::Extension(s)));
                                Some((cmd_ref, items))
                            }
                            _ => None,
                        }) {
                            for name in items.iter().filter_map(|item| match item {
                                vk::InterfaceItem::Command { name, .. } => Some(name.as_str()),
                                _ => None,
                            }) {
                                let info = self.cmd_info_by_name.get_mut(name).expect("missing command info");

                                info.category = info.category.or(ext_category);
                                assert_eq!(info.category, ext_category);

                                let ref_pair = CommandRefPair {
                                    primary: CommandRef::Extension(ext.name.as_str()),
                                    secondary: cmd_ref,
                                };
                                if !info.refs.iter().any(|p| p.matches(&ref_pair)) {
                                    info.refs.push(ref_pair);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn new(registry: &'a vk::Registry) -> Self {
        let mut gen = Self {
            registry,
            extension_by_name: HashMap::new(),
            type_by_name: HashMap::new(),
            used_type_names: HashSet::new(),
            type_name_blacklist: HashSet::new(),
            tag_names: HashSet::new(),
            bitmask_from_value: HashMap::new(),
            enums_by_name: HashMap::new(),
            constant_enums: Vec::new(),
            extension_by_enum_name: HashMap::new(),
            cmd_names: Vec::new(),
            cmd_info_by_name: HashMap::new(),
        };
        gen.collect_extensions();
        gen.collect_types();
        gen.collect_tags();
        gen.collect_enums();
        gen.collect_commands();
        gen
    }

    fn is_non_null_type(&self, base: CBaseType) -> bool {
        base.try_name()
            .and_then(|name| self.type_by_name.get(name))
            .and_then(|ref ty| ty.category.as_deref())
            .map(|s| s == "funcpointer" || s == "handle")
            .unwrap_or(false)
    }

    fn get_rust_type_name(&self, base_type: CBaseType, use_option: bool, vk_prefix: Option<&str>) -> String {
        match base_type {
            CBaseType::Void => "c_void".to_owned(),
            CBaseType::Char => "c_char".to_owned(),
            CBaseType::Int => "c_int".to_owned(),
            CBaseType::F32 => "f32".to_owned(),
            CBaseType::F64 => "f64".to_owned(),
            CBaseType::U8 => "u8".to_owned(),
            CBaseType::U16 => "u16".to_owned(),
            CBaseType::U32 => "u32".to_owned(),
            CBaseType::U64 => "u64".to_owned(),
            CBaseType::I8 => "i8".to_owned(),
            CBaseType::I16 => "i16".to_owned(),
            CBaseType::I32 => "i32".to_owned(),
            CBaseType::I64 => "i64".to_owned(),
            CBaseType::USize => "usize".to_owned(),
            CBaseType::Named(type_name) => {
                let type_name = self.bitmask_from_value.get(type_name).copied().unwrap_or(type_name);
                if type_name.starts_with(TYPE_PREFIX) {
                    if self.is_non_null_type(base_type) && use_option {
                        format!(
                            "Option<{}{}>",
                            vk_prefix.unwrap_or(""),
                            type_name.skip_prefix(TYPE_PREFIX)
                        )
                    } else {
                        format!("{}{}", vk_prefix.unwrap_or(""), type_name.skip_prefix(TYPE_PREFIX))
                    }
                } else if type_name.starts_with(PFN_PREFIX) {
                    if use_option {
                        format!(
                            "Option<{}Fn{}>",
                            vk_prefix.unwrap_or(""),
                            type_name.skip_prefix(PFN_PREFIX)
                        )
                    } else {
                        format!("{}Fn{}", vk_prefix.unwrap_or(""), type_name.skip_prefix(PFN_PREFIX))
                    }
                } else {
                    format!("{}{}", vk_prefix.unwrap_or(""), type_name)
                }
            }
        }
    }

    fn get_rust_default(&self, base_type: CBaseType) -> String {
        match base_type {
            CBaseType::Void => panic!("cannot set a default"),
            CBaseType::Char => "c_char::default()".to_owned(),
            CBaseType::Int => "c_int::default()".to_owned(),
            CBaseType::F32 => "f32::default()".to_owned(),
            CBaseType::F64 => "f64::default()".to_owned(),
            CBaseType::U8 => "u8::default()".to_owned(),
            CBaseType::U16 => "u16::default()".to_owned(),
            CBaseType::U32 => "u32::default()".to_owned(),
            CBaseType::U64 => "u64::default()".to_owned(),
            CBaseType::I8 => "i8::default()".to_owned(),
            CBaseType::I16 => "i16::default()".to_owned(),
            CBaseType::I32 => "i32::default()".to_owned(),
            CBaseType::I64 => "i64::default()".to_owned(),
            CBaseType::USize => "usize::default()".to_owned(),
            CBaseType::Named(type_name) => {
                let type_name = self.bitmask_from_value.get(type_name).copied().unwrap_or(type_name);
                if type_name.starts_with(TYPE_PREFIX) {
                    if self
                        .type_by_name
                        .get(type_name)
                        .and_then(|ref ty| ty.category.as_deref())
                        .map(|s| s == "handle")
                        .unwrap_or(false)
                    {
                        "None".to_owned()
                    } else {
                        format!("{}::default()", type_name.skip_prefix(TYPE_PREFIX))
                    }
                } else if type_name.starts_with(PFN_PREFIX) {
                    "None".to_owned()
                } else {
                    "unsafe { mem::zeroed() }".to_owned()
                }
            }
        }
    }

    fn write_constants(&self, w: &mut impl IoWrite) -> WriteResult {
        let mut expr_by_name = HashMap::new();
        for en in &self.constant_enums {
            match en.spec {
                vk::EnumSpec::Value { ref value, .. } => {
                    let expr = c_parse_expr(value.as_str());
                    write!(w, "pub const {}: ", en.name.as_str().skip_prefix(CONST_PREFIX))?;
                    match expr {
                        CExpr::Literal(x) => match en.name.as_str() {
                            "VK_TRUE" | "VK_FALSE" => write!(w, "Bool32 = {};", x)?,
                            _ => writeln!(w, "usize = {};", x)?,
                        },
                        CExpr::Uint32(x) => writeln!(w, "u32 = {:#x};", x)?,
                        CExpr::Uint64(x) => writeln!(w, "u64 = {:#x};", x)?,
                        CExpr::Float(x) => writeln!(w, "f32 = {}_f32;", x)?,
                    }
                    expr_by_name.insert(en.name.as_str(), expr);
                }
                vk::EnumSpec::Alias { ref alias, .. } => {
                    if let Some(expr) = expr_by_name.get(alias.as_str()) {
                        writeln!(
                            w,
                            "pub const {}: {} = {};",
                            en.name.as_str().skip_prefix(CONST_PREFIX),
                            match expr {
                                CExpr::Literal(_) => "usize",
                                CExpr::Uint32(_) => "u32",
                                CExpr::Uint64(_) => "u64",
                                CExpr::Float(_) => "f32",
                            },
                            alias.as_str().skip_prefix(CONST_PREFIX)
                        )?;
                    } else {
                        panic!("failed to find alias {:?}", en);
                    }
                }
                _ => panic!("unexpected constant enum spec {:?}", en),
            }
        }
        Ok(())
    }

    fn write_base_type(&self, w: &mut impl IoWrite, ty: &vk::Type) -> WriteResult {
        if let vk::TypeSpec::Code(ref code) = ty.spec {
            if let Some(decl) = c_try_parse_typedef(code.code.as_str()) {
                writeln!(
                    w,
                    "pub type {} = {}{};",
                    decl.name.skip_prefix(TYPE_PREFIX),
                    decl.ty.decoration,
                    self.get_rust_type_name(decl.ty.base, true, None)
                )?;
            }
        } else {
            panic!("missing code for {:?}", ty);
        }
        Ok(())
    }

    fn is_enum_value_type_used(&self, type_name: &'a str) -> bool {
        // not replaced by a bitmask and used as a parameter or member
        self.bitmask_from_value.get(type_name).is_none() && self.used_type_names.get(type_name).is_some()
    }

    fn get_enum_entry_name(&self, type_name: &str, enum_type: EnumType, enum_name: &str) -> String {
        let uppercase_entry_name = enum_name.to_uppercase();
        // HACK: move numbers in the typename so they can be removed from the name
        let type_name = if let Some(index) = type_name.find("FlagBits2") {
            format!("{}_2_{}", &type_name[..index], &type_name[(index + 9)..])
        } else {
            type_name.to_owned()
        };
        let shouty_type_name = type_name.to_shouty_snake_case();
        let mut name_parts: Vec<&str> = uppercase_entry_name
            .split('_')
            .zip(shouty_type_name.split('_').chain(iter::repeat("")))
            .skip_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();

        let tag_match = name_parts.last().and_then(|tag| self.tag_names.get(tag));
        if tag_match.is_some() {
            name_parts.pop();
        }
        if matches!(enum_type, EnumType::Bitmask(_)) && name_parts.last() == Some(&"BIT") {
            name_parts.pop();
        }
        if let Some(tag) = tag_match {
            if !shouty_type_name.ends_with(tag) {
                name_parts.push(tag)
            }
        }

        let mut short_name = String::new();
        if name_parts.first().unwrap().chars().next().unwrap().is_numeric() {
            short_name.push('N');
        }
        short_name.push_str(&name_parts.join("_"));
        short_name
    }

    fn get_enum_entry_value(&self, type_name: &str, enum_type: EnumType, en: &'a vk::Enum) -> EnumEntryValue {
        match en.spec {
            vk::EnumSpec::Value { ref value, .. } => EnumEntryValue::Number {
                value: c_parse_int(value) as i64,
                comment: en.comment.clone(),
            },
            vk::EnumSpec::Bitpos { ref bitpos, .. } => EnumEntryValue::Number {
                value: 1 << bitpos,
                comment: en.comment.clone(),
            },
            vk::EnumSpec::Offset {
                offset, extnumber, dir, ..
            } => {
                let num = extnumber.unwrap_or_else(|| {
                    let ext = self
                        .extension_by_enum_name
                        .get(en.name.as_str())
                        .expect("missing extension for enum with no extnumber");
                    ext.number.expect("missing number for extension")
                });
                let value = 1_000_000_000 + (num - 1) * 1000 + offset;
                EnumEntryValue::Number {
                    value: if dir { value } else { -value },
                    comment: en.comment.clone(),
                }
            }
            vk::EnumSpec::Alias { ref alias, .. } => {
                EnumEntryValue::Alias(self.get_enum_entry_name(type_name, enum_type, alias.as_str()))
            }
            _ => panic!("unexpected enum spec"),
        }
    }

    fn write_enum_type(&self, w: &mut impl IoWrite, ty: &vk::Type) -> WriteResult {
        let type_name = ty.get_type_name();
        if self.type_name_blacklist.contains(type_name) {
            return Ok(());
        }
        if let Some(ref comment) = ty.comment {
            writeln!(w, "/// {}", comment.as_str().trim_start_matches('/'))?;
        }
        if let Some(alias) = ty.alias.as_deref() {
            if ty.category.as_deref() == Some("enum") && !self.is_enum_value_type_used(alias) {
                return Ok(());
            }
            writeln!(
                w,
                "pub type {} = {};",
                type_name.skip_prefix(TYPE_PREFIX),
                alias.skip_prefix(TYPE_PREFIX),
            )?;
        } else {
            let enum_type = match ty.category.as_deref() {
                Some("enum") => EnumType::Value,
                Some("bitmask") => {
                    let size = match &ty.spec {
                        vk::TypeSpec::Code(vk::TypeCode { code, .. }) => {
                            let def = c_try_parse_typedef(&code).unwrap();
                            match def.ty.base {
                                CBaseType::Named("VkFlags") => BitMaskSize::N32,
                                CBaseType::Named("VkFlags64") => BitMaskSize::N64,
                                _ => panic!("unknown enum type {:?}", def.ty),
                            }
                        }
                        _ => panic!("failed to find bitmask size for {:?}", ty),
                    };
                    EnumType::Bitmask(size)
                }
                _ => panic!("unknown enum category {:?}", ty.category),
            };
            let value_type_name = match enum_type {
                EnumType::Bitmask(_) => ty.get_bitmask_value_name().unwrap_or_else(|| type_name.to_owned()),
                EnumType::Value => {
                    if self.is_enum_value_type_used(type_name) {
                        type_name.to_owned()
                    } else {
                        return Ok(());
                    }
                }
            };

            let entries: Vec<(String, EnumEntryValue, Option<&vk::Extension>)> = self
                .enums_by_name
                .get(value_type_name.as_str())
                .map(|s| s.as_slice())
                .unwrap_or(&[])
                .iter()
                .map(|en| {
                    (
                        self.get_enum_entry_name(&value_type_name, enum_type, en.name.as_str()),
                        self.get_enum_entry_value(&value_type_name, enum_type, en),
                        self.extension_by_enum_name.get(en.name.as_str()).copied(),
                    )
                })
                .collect();

            let (derives, interior_type) = match enum_type {
                EnumType::Bitmask(BitMaskSize::N32) => ("Debug, Copy, Clone, PartialEq, Eq, Hash", "u32"),
                EnumType::Bitmask(BitMaskSize::N64) => ("Debug, Copy, Clone, PartialEq, Eq, Hash", "u64"),
                EnumType::Value => ("Debug, Copy, Clone, PartialOrd, Ord, PartialEq, Eq, Hash", "i32"),
            };
            let enum_name = type_name.skip_prefix(TYPE_PREFIX);
            writeln!(
                w,
                "#[repr(transparent)] #[derive({derives})] pub struct {enum_name}({interior_type});\nimpl {enum_name} {{",
                derives=derives, enum_name=enum_name, interior_type=interior_type
            )?;
            let mut all = 0;
            for (ref name, value, ref ext) in &entries {
                match value {
                    EnumEntryValue::Number { value, ref comment } => {
                        if let Some(comment) = comment {
                            writeln!(w, "/// {}", comment)?;
                        }
                        if let Some(ext) = ext {
                            writeln!(w, "/// Added by extension {}.", ext.name)?;
                        }
                        writeln!(
                            w,
                            "pub const {}: Self = Self({});",
                            name,
                            match enum_type {
                                EnumType::Bitmask(BitMaskSize::N32) => format!("{:#x}", *value as i32),
                                EnumType::Bitmask(BitMaskSize::N64) => format!("{:#x}", value),
                                EnumType::Value => format!("{}", *value as i32),
                            },
                        )?;
                        all |= value;
                    }
                    EnumEntryValue::Alias(ref alias) => {
                        if name != alias {
                            writeln!(w, "pub const {}: Self = Self::{};", name, alias)?;
                        }
                    }
                }
            }
            writeln!(w, "}}")?;
            writeln!(
                w,
                "impl default::Default for {} {{ fn default() -> Self {{ Self(0) }} }}",
                enum_name
            )?;
            match enum_type {
                EnumType::Bitmask(_) => {
                    writeln!(
                        w,
                        "impl {0} {{\
                         pub fn empty() -> Self {{ Self(0) }}\
                         pub fn all() -> Self {{ Self({1:#x}) }}\
                         pub fn is_empty(self) -> bool {{ self.0 == 0 }}\
                         pub fn is_all(self) -> bool {{ self.0 == {1:#x} }}\
                         pub fn intersects(self, other: Self) -> bool {{ (self.0 & other.0) != 0 }}\
                         pub fn contains(self, other: Self) -> bool {{ (self.0 & other.0) == other.0 }}\
                         }}",
                        enum_name, all
                    )?;
                    writeln!(
                        w,
                        "impl ops::BitOr for {} {{ type Output = Self;\
                         fn bitor(self, rhs: Self) -> Self {{ Self(self.0 | rhs.0) }} }}",
                        enum_name
                    )?;
                    writeln!(
                        w,
                        "impl ops::BitOrAssign for {} {{\
                         fn bitor_assign(&mut self, rhs: Self) {{ self.0 |= rhs.0; }} }}",
                        enum_name
                    )?;
                    writeln!(
                        w,
                        "impl ops::BitAnd for {} {{ type Output = Self;\
                         fn bitand(self, rhs: Self) -> Self {{ Self(self.0 & rhs.0) }} }}",
                        enum_name
                    )?;
                    writeln!(
                        w,
                        "impl ops::BitAndAssign for {} {{\
                         fn bitand_assign(&mut self, rhs: Self) {{ self.0 &= rhs.0; }} }}",
                        enum_name
                    )?;
                    writeln!(
                        w,
                        "impl ops::BitXor for {} {{ type Output = Self;\
                         fn bitxor(self, rhs: Self) -> Self {{ Self(self.0 ^ rhs.0) }} }}",
                        enum_name
                    )?;
                    writeln!(
                        w,
                        "impl ops::BitXorAssign for {} {{\
                         fn bitxor_assign(&mut self, rhs: Self) {{ self.0 ^= rhs.0; }} }}",
                        enum_name
                    )?;
                    writeln!(
                        w,
                        "impl fmt::Display for {} {{\
                         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{",
                        enum_name
                    )?;
                    writeln!(w, "display_bitmask(self.0 as _, &[")?;
                    for (ref name, value, _) in &entries {
                        if let EnumEntryValue::Number { value, .. } = value {
                            if *value != 0 {
                                writeln!(w, r#"({:#x}, "{}"),"#, value, name)?;
                            }
                        }
                    }
                    writeln!(w, "], f)")?;
                    writeln!(w, "}} }}")?;
                }
                EnumType::Value => {
                    writeln!(
                        w,
                        "impl fmt::Display for {} {{\
                         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{",
                        enum_name
                    )?;
                    if !entries.is_empty() {
                        writeln!(w, "let name = match self.0 {{")?;
                        for (ref name, value, _) in &entries {
                            if let EnumEntryValue::Number { value, .. } = value {
                                writeln!(w, r#"{} => Some(&"{}"),"#, value, name)?;
                            }
                        }
                        writeln!(w, "_ => None, }};")?;
                        writeln!(w, r#"if let Some(name) = name {{ write!(f, "{{}}", name) }} else {{"#)?;
                    }
                    writeln!(w, r#"write!(f, "{{}}", self.0) }} }}"#)?;
                    if !entries.is_empty() {
                        writeln!(w, "}}")?;
                    }
                }
            }
        }
        Ok(())
    }

    fn write_handle_type(&self, w: &mut impl IoWrite, ty: &'a vk::Type) -> WriteResult {
        if let Some(ref alias) = ty.alias {
            let type_name = ty.name.as_deref().expect("missing handle alias name");
            writeln!(
                w,
                "pub type {} = {};",
                type_name.skip_prefix(TYPE_PREFIX),
                alias.skip_prefix(TYPE_PREFIX)
            )?;
        } else if let vk::TypeSpec::Code(ref code) = ty.spec {
            let mut type_name = None;
            let mut handle_def = None;
            for markup in &code.markup {
                match markup {
                    vk::TypeCodeMarkup::Name(ref s) => {
                        assert!(type_name.is_none());
                        type_name = Some(s.as_str());
                    }
                    vk::TypeCodeMarkup::Type(ref s) => {
                        assert!(handle_def.is_none());
                        handle_def = Some(s.as_str());
                    }
                    _ => {}
                }
            }
            let type_name = type_name.expect("missing handle name");
            let handle_name = type_name.skip_prefix(TYPE_PREFIX);
            match handle_def {
                Some("VK_DEFINE_HANDLE") => {
                    writeln!(
                        w,
                        "#[repr(transparent)] #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] pub struct {0}(num::NonZeroUsize);\
                        impl {0} {{ pub fn from_raw(x: usize) -> Option<Self> {{ num::NonZeroUsize::new(x).map(Self) }} }}",
                        handle_name
                    )?;
                }
                Some("VK_DEFINE_NON_DISPATCHABLE_HANDLE") => {
                    writeln!(
                        w,
                        "#[repr(transparent)] #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] pub struct {0}(num::NonZeroU64);\
                        impl {0} {{ pub fn from_raw(x: u64) -> Option<Self> {{ num::NonZeroU64::new(x).map(Self) }} }}",
                        handle_name
                    )?;
                }
                _ => panic!("missing handle type for {:?}", ty),
            }
        } else {
            panic!("missing handle code for {:?}", ty);
        }
        Ok(())
    }

    fn get_rust_parameter_type(&self, ty: &CType, vk_prefix: Option<&str>) -> String {
        let mut s = String::new();
        write!(
            &mut s,
            "{}{}{}",
            if ty.array_size.is_some() { "[" } else { "" },
            ty.decoration,
            self.get_rust_type_name(ty.base, ty.decoration == CDecoration::None, vk_prefix)
        )
        .unwrap();
        if let Some(array_size) = ty.array_size {
            write!(&mut s, "; {}]", array_size.skip_prefix(CONST_PREFIX)).unwrap();
        }
        s
    }

    fn write_function_pointer_type(&self, w: &mut impl IoWrite, ty: &'a vk::Type) -> WriteResult {
        if let vk::TypeSpec::Code(ref code) = ty.spec {
            let function_decl = c_parse_func_pointer_typedef(code.code.as_str());
            write!(
                w,
                r#"pub type Fn{} = unsafe extern "system" fn("#,
                function_decl.proto.name.skip_prefix(PFN_PREFIX)
            )?;
            for decl in &function_decl.parameters {
                write!(
                    w,
                    "{}: {},",
                    get_rust_variable_name(&decl.name),
                    self.get_rust_parameter_type(&decl.ty.strip_array(), None),
                )?;
            }
            writeln!(w, ")")?;
            if !function_decl.proto.ty.is_base_type(CBaseType::Void) {
                writeln!(w, "-> {}", self.get_rust_parameter_type(&function_decl.proto.ty, None))?;
            }
            writeln!(w, ";")?;
        } else {
            panic!("missing function pointer code for {:?}", ty);
        }
        Ok(())
    }

    fn rewrite_variable_decl(&self, context: &str, mut decl: CVariableDecl<'a>) -> CVariableDecl<'a> {
        match decl.name {
            "apiVersion" | "pApiVersion" => {
                decl.ty.base = NAMED_TYPE_VKVERSION;
            }
            "specVersion" if context == "LayerProperties" => {
                decl.ty.base = NAMED_TYPE_VKVERSION;
            }
            _ => {}
        }
        decl
    }

    fn write_aggregrate_type(&self, w: &mut impl IoWrite, ty: &vk::Type, agg_type: AggregateType) -> WriteResult {
        let type_name = ty.name.as_deref().expect("missing struct name");
        if self.type_name_blacklist.contains(type_name) {
            return Ok(());
        }
        if let Some(ref comment) = ty.comment {
            writeln!(w, "/// {}", comment.as_str().trim_start_matches('/'))?;
        }
        if let Some(ref alias) = ty.alias {
            if !self.type_name_blacklist.contains(alias.as_str()) {
                writeln!(
                    w,
                    "pub type {} = {};",
                    type_name.skip_prefix(TYPE_PREFIX),
                    alias.skip_prefix(TYPE_PREFIX)
                )?;
            }
        } else if let vk::TypeSpec::Members(ref members) = ty.spec {
            let agg_name = type_name.skip_prefix(TYPE_PREFIX);
            writeln!(
                w,
                "#[repr(C)] #[derive({})] pub {} {} {{",
                if self.check_type_derives(CBaseType::Named(type_name)) {
                    "Copy, Clone, PartialEq, Eq, Hash"
                } else {
                    "Copy, Clone"
                },
                match agg_type {
                    AggregateType::Struct => "struct",
                    AggregateType::Union => "union",
                },
                agg_name
            )?;
            let mut members: Vec<_> = members
                .iter()
                .filter_map(|member| match member {
                    vk::TypeMember::Definition(ref member_def) => Some(member_def),
                    _ => None,
                })
                .map(|member_def| {
                    let decl = c_parse_variable_decl(member_def.code.as_str());
                    let decl = self.rewrite_variable_decl(agg_name, decl);
                    StructMember {
                        def: member_def,
                        decl,
                        merged_name: String::new(),
                    }
                })
                .collect();
            // merge bitfield members
            while let Some(index) = members
                .iter()
                .enumerate()
                .find(|(_, member)| member.decl.ty.bit_count.is_some())
                .map(|(i, _)| i)
            {
                let temp = members.remove(index + 1);
                members[index].merge_with(temp);
            }
            for member in members.iter() {
                for comment in member.def.markup.iter().filter_map(|markup| match markup {
                    vk::TypeMemberMarkup::Comment(ref comment) => Some(comment),
                    _ => None,
                }) {
                    writeln!(w, "/// {}", comment)?;
                }
                writeln!(
                    w,
                    "pub {}: {},",
                    get_rust_variable_name(member.name()),
                    self.get_rust_parameter_type(&member.decl.ty, None)
                )?;
            }
            writeln!(w, "}}")?;
            writeln!(w, "impl default::Default for {} {{ fn default() -> Self {{", agg_name)?;
            match agg_type {
                AggregateType::Struct => {
                    write!(w, "Self {{")?;
                    for member in members.iter() {
                        write!(w, "{}: ", get_rust_variable_name(&member.name()))?;
                        if let Some(values) = member.def.values.as_deref() {
                            // assume enum value for now
                            let member_type_name = member.decl.ty.base.try_name().unwrap();
                            let name = self.get_enum_entry_name(member_type_name, EnumType::Value, values);
                            writeln!(
                                w,
                                "{}::{},",
                                self.get_rust_type_name(member.decl.ty.base, true, None),
                                name
                            )?;
                        } else {
                            // get element default
                            let element_value = match member.decl.ty.decoration {
                                CDecoration::Pointer | CDecoration::PointerToPointer => "ptr::null_mut()".to_owned(),
                                CDecoration::PointerToConst | CDecoration::PointerToConstPointerToConst => {
                                    "ptr::null()".to_owned()
                                }
                                CDecoration::None | CDecoration::Const => {
                                    self.get_rust_default(member.decl.ty.base).to_owned()
                                }
                            };

                            // write single or array
                            if let Some(array_size) = member.decl.ty.array_size {
                                writeln!(w, "[{}; {}],", element_value, array_size.skip_prefix(CONST_PREFIX))?;
                            } else {
                                writeln!(w, "{},", element_value)?;
                            }
                        }
                    }
                    writeln!(w, "}}")?;
                }
                AggregateType::Union => {
                    writeln!(w, "unsafe {{ mem::zeroed() }}")?;
                }
            }
            writeln!(w, "}} }}")?;
            {
                writeln!(w, "impl fmt::Debug for {} {{", agg_name)?;
                writeln!(w, "fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {{")?;
                writeln!(w, r#"fmt.debug_struct("{}")"#, agg_name)?;
                for member in members.iter() {
                    let var_name = get_rust_variable_name(&member.name());
                    let category = member
                        .decl
                        .ty
                        .base
                        .try_name()
                        .and_then(|name| self.type_by_name.get(name))
                        .and_then(|ty| ty.category.as_deref());
                    if member.decl.ty.base == CBaseType::Char
                        && member.decl.ty.decoration == CDecoration::None
                        && member.decl.ty.array_size.is_some()
                    {
                        writeln!(
                            w,
                            r#".field("{0}", &unsafe {{ CStr::from_ptr(self.{0}.as_ptr()) }})"#,
                            var_name
                        )?;
                    } else if category == Some("funcpointer")
                        && member.decl.ty.decoration == CDecoration::None
                        && member.decl.ty.array_size.is_none()
                    {
                        writeln!(
                            w,
                            r#".field("{0}", if self.{0}.is_some() {{ &"Some" }} else {{ &"None" }})"#,
                            var_name
                        )?;
                    } else if agg_type == AggregateType::Union {
                        writeln!(w, r#".field("{0}", unsafe {{ &self.{0} }})"#, var_name)?;
                    } else {
                        writeln!(w, r#".field("{0}", &self.{0})"#, var_name)?;
                    }
                }
                writeln!(w, ".finish()")?;
                writeln!(w, "}} }}")?;
            }
        } else {
            panic!("missing struct members for {:?}", ty);
        }
        Ok(())
    }

    fn write_types(&self, w: &mut impl IoWrite) -> WriteResult {
        for ty in self.get_type_iterator() {
            let category = ty.category.as_deref();
            match category {
                Some("basetype") => {
                    self.write_base_type(w, ty)?;
                }
                Some("bitmask") | Some("enum") => {
                    self.write_enum_type(w, ty)?;
                }
                Some("handle") => {
                    self.write_handle_type(w, ty)?;
                }
                Some("funcpointer") => {
                    self.write_function_pointer_type(w, ty)?;
                }
                Some("struct") => {
                    self.write_aggregrate_type(w, ty, AggregateType::Struct)?;
                }
                Some("union") => {
                    self.write_aggregrate_type(w, ty, AggregateType::Union)?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn write_command_types(&self, w: &mut impl IoWrite) -> WriteResult {
        for info in self.cmd_names.iter().filter_map(|name| {
            let info = self.cmd_info_by_name.get(name).expect("missing command info");
            info.category.and(Some(info))
        }) {
            if info.alias.is_none() {
                let mut decl = c_parse_function_decl(info.cmd_def.code.as_str());
                let context = decl.proto.name;
                for param in decl.parameters.iter_mut() {
                    take_mut::take(param, |v| self.rewrite_variable_decl(context, v));
                }
                let name_part = decl.proto.name.skip_prefix(FN_PREFIX);
                write!(w, r#"pub type Fn{} = unsafe extern "system" fn("#, name_part)?;
                for param in &decl.parameters {
                    write!(
                        w,
                        "{}: {},",
                        get_rust_variable_name(param.name.to_snake_case().as_str()),
                        self.get_rust_parameter_type(&param.ty.strip_array(), None),
                    )?;
                }
                writeln!(w, ")")?;
                if !decl.proto.ty.is_base_type(CBaseType::Void) {
                    writeln!(w, "-> {}", self.get_rust_parameter_type(&decl.proto.ty, None))?;
                }
                writeln!(w, ";")?;
            }
        }
        Ok(())
    }

    fn write_vk(&self, path: &Path) -> WriteResult {
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);

        write!(&mut w, "{}", include_str!("vk_prefix.rs"))?;
        self.write_constants(&mut w)?;
        self.write_types(&mut w)?;
        self.write_command_types(&mut w)?;
        Ok(())
    }

    fn write_builder_structs(&self, w: &mut impl IoWrite) -> WriteResult {
        for ty in self
            .get_type_iterator()
            .filter(|ty| ty.category.as_deref() == Some("struct") && ty.alias.is_none())
            .filter(|ty| !self.type_name_blacklist.contains(ty.get_type_name()))
        {
            if let vk::TypeSpec::Members(ref members) = ty.spec {
                let type_name = ty.name.as_deref().expect("missing struct name");
                let agg_name = type_name.skip_prefix(TYPE_PREFIX);
                let member_defs: Vec<&vk::TypeMemberDefinition> = members
                    .iter()
                    .filter_map(|member| match member {
                        vk::TypeMember::Definition(ref member_def) => Some(member_def),
                        _ => None,
                    })
                    .collect();
                let decls: Vec<CVariableDecl> = member_defs
                    .iter()
                    .map(|member_def| c_parse_variable_decl(member_def.code.as_str()))
                    .map(|decl| self.rewrite_variable_decl(agg_name, decl))
                    .collect();
                let mut params: Vec<LibParam> = decls
                    .iter()
                    .map(|decl| LibParam {
                        name: get_rust_variable_name(decl.name.to_snake_case().as_str()),
                        ty: LibParamType::CDecl,
                    })
                    .collect();
                for (i, cparam) in decls.iter().enumerate() {
                    let vparam = &member_defs[i];
                    let inner_type_name = self.get_rust_type_name(
                        cparam.ty.base,
                        cparam.ty.decoration == CDecoration::None,
                        Some("vk::"),
                    );

                    // match bool
                    if cparam.ty.base == NAMED_TYPE_VKBOOL32 && cparam.ty.decoration == CDecoration::None {
                        params[i].ty = LibParamType::Bool;
                        continue;
                    }

                    // match CStr
                    if cparam.ty.base == CBaseType::Char
                        && cparam.ty.decoration == CDecoration::PointerToConst
                        && vparam.len.as_deref() == Some("null-terminated")
                    {
                        let is_optional = vparam.optional.as_deref() == Some("true");
                        params[i].ty = LibParamType::CStr { is_optional };
                        continue;
                    }

                    // remove Option if not optional
                    if cparam.ty.decoration == CDecoration::None
                        && vparam.optional.is_none()
                        && self.is_non_null_type(cparam.ty.base)
                    {
                        params[i].ty = LibParamType::NonOptional {
                            inner_type_name: self.get_rust_type_name(cparam.ty.base, false, Some("vk::")),
                        };
                        continue;
                    }

                    // match slice
                    if let Some(len_name) = vparam.len.as_deref().and_then(|s| s.split(',').next()) {
                        let is_slice_type = cparam.ty.decoration == CDecoration::PointerToConst
                            || cparam.ty.decoration == CDecoration::PointerToConstPointerToConst;
                        if is_slice_type {
                            let is_mutable = false;
                            let is_optional = vparam.optional.as_deref() == Some("true");
                            let is_single = vparam.noautovalidity.as_deref() == Some("true")
                                || vparam.optional.as_deref() == Some("true,false")
                                || vparam.optional.as_deref() == Some("true,false,false");
                            if let Some(len_index) = decls.iter().position(|decl| decl.name == len_name) {
                                let len_cparam = &decls[len_index];
                                let inner_type_name =
                                    if cparam.ty.decoration == CDecoration::PointerToConstPointerToConst {
                                        format!("*const {}", inner_type_name)
                                    } else {
                                        inner_type_name
                                    };
                                let slice_info = SliceInfo {
                                    name: params[i].name.clone(),
                                    type_name: inner_type_name.clone(),
                                    is_mutable,
                                    is_optional,
                                };
                                params[i].ty = LibParamType::Slice {
                                    inner_type_name,
                                    is_mutable,
                                    is_optional,
                                    len_expr: None,
                                };
                                take_mut::take(&mut params[len_index].ty, |ty| match ty {
                                    LibParamType::SliceLenShared { name, mut slice_infos } => {
                                        slice_infos.push(slice_info);
                                        if is_single {
                                            LibParamType::SliceLenSingle { slice_infos }
                                        } else {
                                            LibParamType::SliceLenShared { name, slice_infos }
                                        }
                                    }
                                    LibParamType::SliceLenSingle { mut slice_infos } => {
                                        slice_infos.push(slice_info);
                                        if is_single {
                                            LibParamType::SliceLenSingle { slice_infos }
                                        } else {
                                            panic!("unsupported mix of slices")
                                        }
                                    }
                                    LibParamType::CDecl => {
                                        if is_single {
                                            LibParamType::SliceLenSingle {
                                                slice_infos: vec![slice_info; 1],
                                            }
                                        } else {
                                            LibParamType::SliceLenShared {
                                                name: len_cparam.name.to_snake_case(),
                                                slice_infos: vec![slice_info; 1],
                                            }
                                        }
                                    }
                                    _ => {
                                        panic!("purpose already found for {:?}", len_cparam);
                                    }
                                });
                                continue;
                            }
                        }
                    }

                    // match reference
                    if cparam.ty.base != CBaseType::Void
                        && cparam.ty.decoration == CDecoration::PointerToConst
                        && vparam.len.is_none()
                    {
                        let is_optional = vparam.optional.as_deref() == Some("true");
                        params[i].ty = LibParamType::Ref {
                            inner_type_name,
                            is_optional,
                        };
                        continue;
                    }
                }
                let is_extended = self
                    .get_type_iterator()
                    .filter_map(|ty| ty.structextends.as_deref())
                    .flat_map(|s| s.split(','))
                    .any(|s| s == type_name);
                let needs_lifetime = is_extended
                    || params.iter().any(|rparam| {
                        matches!(
                            rparam.ty,
                            LibParamType::CStr { .. }
                                | LibParamType::SliceLenShared { .. }
                                | LibParamType::SliceLenSingle { .. }
                                | LibParamType::Ref { .. }
                        )
                    });
                let needs_setters =
                    ty.returnedonly.is_none() && decls.iter().any(|decl| decl.ty.decoration != CDecoration::None);
                if is_extended || needs_setters {
                    let generics_decl = if needs_lifetime { "<'a>" } else { "" };

                    // implement trait on vk type
                    writeln!(
                        w,
                        "impl{0} Builder<'{2}> for vk::{1} {{\
                         type Type = {1}Builder{0};\
                         fn builder() -> Self::Type {{ Default::default() }} }}",
                        generics_decl,
                        agg_name,
                        if needs_lifetime { "a" } else { "_" }
                    )?;

                    // declare extension trait if used
                    if is_extended {
                        writeln!(w, "pub trait {}Next {{ }}", agg_name)?;
                    }

                    // declare builder in lib
                    writeln!(w, "#[derive(Default)]")?;
                    writeln!(
                        w,
                        "pub struct {1}Builder{0} {{\
                         inner: vk::{1},",
                        generics_decl, agg_name
                    )?;
                    if needs_lifetime {
                        writeln!(w, "phantom: PhantomData<&'a vk::Never>,")?;
                    }
                    writeln!(w, "}}")?;

                    // setters
                    writeln!(w, "impl{0} {1}Builder{0} {{", generics_decl, agg_name)?;
                    if is_extended {
                        writeln!(
                            w,
                            "pub fn insert_next<T: {}Next>(mut self, next: &'a mut T) -> Self {{",
                            agg_name
                        )?;
                        writeln!(
                            w,
                            "unsafe {{ insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _); }}"
                        )?;
                        writeln!(w, "self }}")?;
                        if ty.returnedonly.is_some() {
                            writeln!(
                                w,
                                "pub fn get_mut(&mut self) -> &mut vk::{} {{ &mut self.inner }}",
                                agg_name
                            )?;
                        }
                    }
                    if needs_setters {
                        for (cparam, rparam) in decls.iter().zip(params.iter()) {
                            match rparam.ty {
                                LibParamType::CDecl => {
                                    if cparam.ty.array_size.is_none() {
                                        writeln!(
                                            w,
                                            "pub fn {0}(mut self, {0}: {1}) -> Self {{\
                                            self.inner.{0} = {0}; self }}",
                                            rparam.name,
                                            self.get_rust_parameter_type(&cparam.ty, Some("vk::"))
                                        )?;
                                    }
                                }
                                LibParamType::Bool => {
                                    writeln!(
                                        w,
                                        "pub fn {0}(mut self, {0}: bool) -> Self {{\
                                        self.inner.{0} = if {0} {{ vk::TRUE }} else {{ vk::FALSE }}; self }}",
                                        rparam.name
                                    )?;
                                }
                                LibParamType::CStr { is_optional } => {
                                    if is_optional {
                                        writeln!(
                                            w,
                                            "pub fn {0}(mut self, {0}: Option<&'a CStr>) -> Self {{\
                                            self.inner.{0} = {0}.map_or(ptr::null(), |s| s.as_ptr()); self }}",
                                            rparam.name
                                        )?;
                                    } else {
                                        writeln!(
                                            w,
                                            "pub fn {0}(mut self, {0}: &'a CStr) -> Self {{\
                                            self.inner.{0} = {0}.as_ptr(); self }}",
                                            rparam.name
                                        )?;
                                    }
                                }
                                LibParamType::NonOptional { ref inner_type_name } => {
                                    writeln!(
                                        w,
                                        "pub fn {0}(mut self, {0}: {1}) -> Self {{\
                                        self.inner.{0} = Some({0}); self }}",
                                        rparam.name, inner_type_name,
                                    )?;
                                }
                                LibParamType::Slice { .. } => {}
                                LibParamType::SliceLenShared { ref slice_infos, .. } => {
                                    let len_type_name = self.get_rust_parameter_type(&cparam.ty, Some("vk::"));
                                    let any_generic = slice_infos.iter().any(SliceInfo::is_generic);
                                    write!(w, "pub fn {}", slice_infos[0].name)?;
                                    if any_generic {
                                        write!(w, "<T>")?;
                                    }
                                    write!(w, "(mut self ")?;
                                    let has_multiple_slices = slice_infos.len() > 1;
                                    for slice_info in slice_infos {
                                        let type_name = slice_info.get_type_name();
                                        if slice_info.is_optional && has_multiple_slices {
                                            write!(w, ", {}: Option<&'a [{}]>", slice_info.name, type_name)?;
                                        } else {
                                            write!(w, ", {}: &'a [{}]", slice_info.name, type_name)?;
                                        }
                                    }
                                    writeln!(w, ") -> Self {{")?;
                                    writeln!(
                                        w,
                                        "self.inner.{} = {} as {};",
                                        rparam.name,
                                        slice_infos[0].get_len(),
                                        len_type_name
                                    )?;
                                    for slice_info in slice_infos.iter().skip(1) {
                                        if slice_info.is_optional && has_multiple_slices {
                                            writeln!(
                                                w,
                                                "if let Some(s) = {} {{ assert_eq!(self.inner.{}, {} as {}); }}",
                                                slice_info.name,
                                                rparam.name,
                                                slice_len("s", &slice_info.type_name),
                                                len_type_name
                                            )?;
                                        } else {
                                            writeln!(
                                                w,
                                                "assert_eq!(self.inner.{}, {} as {});",
                                                rparam.name,
                                                slice_info.get_len(),
                                                len_type_name
                                            )?;
                                        }
                                    }
                                    for slice_info in slice_infos {
                                        writeln!(
                                            w,
                                            "self.inner.{} = {};",
                                            slice_info.name,
                                            slice_info.get_as_ptr(slice_info.is_optional && has_multiple_slices)
                                        )?;
                                    }
                                    writeln!(w, "self }}",)?;
                                }
                                LibParamType::SliceLenSingle { ref slice_infos } => {
                                    let len_type_name = self.get_rust_parameter_type(&cparam.ty, Some("vk::"));
                                    if slice_infos.iter().all(|s| s.is_optional) {
                                        writeln!(
                                            w,
                                            "pub fn {0}(mut self, {0}: {1}) -> Self {{\
                                            self.inner.{0} = {0}; self }}",
                                            rparam.name,
                                            self.get_rust_parameter_type(&cparam.ty, Some("vk::"))
                                        )?;
                                    }
                                    for slice_info in slice_infos {
                                        assert!(!slice_info.is_generic()); // TODO
                                        write!(
                                            w,
                                            "pub fn {0}(mut self, {0}: &'a [{1}]) -> Self {{",
                                            slice_info.name, slice_info.type_name
                                        )?;
                                        write!(
                                            w,
                                            "self.inner.{} = {}.len() as {};",
                                            rparam.name, slice_info.name, len_type_name
                                        )?;
                                        writeln!(
                                            w,
                                            "self.inner.{0} = {0}.first().map_or(ptr::null(), |s| s as *const _); self }}",
                                            slice_info.name
                                        )?;
                                    }
                                }
                                LibParamType::Ref {
                                    ref inner_type_name,
                                    is_optional,
                                } => {
                                    if is_optional {
                                        writeln!(
                                            w,
                                            "pub fn {0}(mut self, {0}: Option<&'a {1}>) -> Self {{\
                                            self.inner.{0} = {0}.map_or(ptr::null(), |p| p); self }}",
                                            rparam.name, inner_type_name,
                                        )?;
                                    } else {
                                        writeln!(
                                            w,
                                            "pub fn {0}(mut self, {0}: &'a {1}) -> Self {{\
                                            self.inner.{0} = {0}; self }}",
                                            rparam.name, inner_type_name,
                                        )?;
                                    }
                                }
                                _ => panic!("unhandled struct member {:?}", rparam),
                            }
                        }
                    }
                    writeln!(w, "}}")?;

                    // allow deref to vk type
                    writeln!(
                        w,
                        "impl{0} Deref for {1}Builder{0} {{\
                         type Target = vk::{1};\
                         fn deref(&self) -> &Self::Target {{ &self.inner }} }}",
                        generics_decl, agg_name
                    )?;

                    // implement next marker traits for builder
                    if let Some(structextends) = ty.structextends.as_deref() {
                        for base_type_name in structextends
                            .split(',')
                            .filter(|base_type_name| !self.type_name_blacklist.contains(base_type_name))
                        {
                            writeln!(
                                w,
                                "impl{} {}Next for {}Builder{0} {{ }}",
                                generics_decl,
                                base_type_name.skip_prefix("Vk"),
                                agg_name
                            )?;
                        }
                    }
                }

                // implement next marker traits for base type
                if let Some(structextends) = ty.structextends.as_deref() {
                    for base_type_name in structextends
                        .split(',')
                        .filter(|base_type_name| !self.type_name_blacklist.contains(base_type_name))
                    {
                        writeln!(
                            w,
                            "impl {}Next for vk::{} {{ }}",
                            base_type_name.skip_prefix("Vk"),
                            agg_name
                        )?;
                    }
                }
            } else {
                panic!("missing type members for {:?}", ty);
            }
        }
        Ok(())
    }

    fn write_builder(&self, path: &Path) -> WriteResult {
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);

        write!(&mut w, "{}", include_str!("builder_prefix.rs"))?;
        self.write_builder_structs(&mut w)?;

        Ok(())
    }

    fn wrap_command_arguments(
        &self,
        category: Category,
        cmd_def: &vk::CommandDefinition,
        cmd_return_value: CommandReturnValue,
        decl: &CFunctionDecl,
        params: &mut [LibParam],
    ) -> (LibReturnType, LibReturnTransform, String) {
        let mut return_type = LibReturnType::CDecl;
        let mut return_transform = LibReturnTransform::None;
        let mut return_type_name = self.get_rust_parameter_type(&decl.proto.ty, Some("vk::"));
        for (i, cparam) in decl.parameters.iter().enumerate() {
            let vparam = &cmd_def.params[i];
            let inner_type_name =
                self.get_rust_type_name(cparam.ty.base, cparam.ty.decoration == CDecoration::None, Some("vk::"));

            // match member handle (first parameter only)
            if i == 0 {
                if let Some(type_name) = match category {
                    Category::Instance => Some("VkInstance"),
                    Category::Device => Some("VkDevice"),
                    _ => None,
                } {
                    if cparam.ty.base == CBaseType::Named(type_name) && cparam.ty.decoration == CDecoration::None {
                        params[i].ty = LibParamType::MemberHandle;
                        continue;
                    }
                }
            }

            // match bool
            if cparam.ty.base == NAMED_TYPE_VKBOOL32 && cparam.ty.decoration == CDecoration::None {
                params[i].ty = LibParamType::Bool;
                continue;
            }

            // match CStr
            if cparam.ty.base == CBaseType::Char
                && cparam.ty.decoration == CDecoration::PointerToConst
                && vparam.len.as_deref() == Some("null-terminated")
            {
                let is_optional = vparam.optional.as_deref() == Some("true");
                params[i].ty = LibParamType::CStr { is_optional };
                continue;
            }

            // remove Option if not optional
            if cparam.ty.decoration == CDecoration::None
                && vparam.optional.is_none()
                && self.is_non_null_type(cparam.ty.base)
            {
                params[i].ty = LibParamType::NonOptional {
                    inner_type_name: self.get_rust_type_name(cparam.ty.base, false, Some("vk::")),
                };
                continue;
            }

            // match slice (parameter or return)
            if let Some(len_name) = vparam.len.as_deref() {
                if cparam.ty.decoration == CDecoration::PointerToConst
                    || cparam.ty.decoration == CDecoration::PointerToConstPointerToConst
                    || (cparam.ty.base == CBaseType::Void
                        && vparam.optional.is_none()
                        && cparam.ty.decoration == CDecoration::Pointer)
                {
                    let is_mutable = matches!(cparam.ty.decoration, CDecoration::Pointer);
                    let is_optional = vparam.optional.as_deref() == Some("true");
                    let inner_type_name = if cparam.ty.decoration == CDecoration::PointerToConstPointerToConst {
                        format!("*const {}", inner_type_name)
                    } else {
                        inner_type_name
                    };
                    let len_expr =
                        if let Some(len_index) = decl.parameters.iter().position(|cparam| cparam.name == len_name) {
                            let len_cparam = &decl.parameters[len_index];
                            let slice_info = SliceInfo {
                                name: params[i].name.clone(),
                                type_name: inner_type_name.clone(),
                                is_mutable,
                                is_optional,
                            };
                            take_mut::take(&mut params[len_index].ty, |ty| match ty {
                                LibParamType::SliceLenShared { name, mut slice_infos } => {
                                    slice_infos.push(slice_info);
                                    LibParamType::SliceLenShared { name, slice_infos }
                                }
                                LibParamType::CDecl => LibParamType::SliceLenShared {
                                    name: len_cparam.name.to_snake_case(),
                                    slice_infos: vec![slice_info; 1],
                                },
                                _ => {
                                    panic!("purpose already found for {:?}", len_cparam);
                                }
                            });
                            None
                        } else {
                            let len_names: Vec<&str> = len_name.split("::").flat_map(|s| s.split("->")).collect();
                            let len_names: Vec<String> = len_names
                                .iter()
                                .map(|s| get_rust_variable_name(s.to_snake_case().as_str()))
                                .collect();
                            Some(len_names.join("."))
                        };
                    params[i].ty = LibParamType::Slice {
                        inner_type_name: inner_type_name.clone(),
                        is_mutable,
                        is_optional,
                        len_expr,
                    };
                    continue;
                }
                if cparam.ty.base != CBaseType::Void
                    && cparam.ty.decoration == CDecoration::Pointer
                    && vparam.optional.as_deref() == Some("true")
                    && (cmd_return_value == CommandReturnValue::Void
                        || cmd_def.successcodes.as_deref() == Some("VK_SUCCESS,VK_INCOMPLETE"))
                {
                    let len_index = decl
                        .parameters
                        .iter()
                        .position(|cparam| cparam.name == len_name)
                        .expect("missing len variable");
                    let len_cparam = &decl.parameters[len_index];
                    let len_vparam = &cmd_def.params[len_index];
                    if len_cparam.ty.decoration == CDecoration::Pointer
                        && len_vparam.optional.as_deref() == Some("false,true")
                    {
                        params[i].ty = LibParamType::ReturnVec {
                            inner_type_name: inner_type_name.clone(),
                        };
                        let slice_name = params[i].name.clone();
                        take_mut::take(&mut params[len_index].ty, |ty| match ty {
                            LibParamType::CDecl => LibParamType::ReturnVecLenSingle { slice_name },
                            LibParamType::ReturnVecLenSingle { .. } => LibParamType::ReturnVecLenShared,
                            _ => panic!("purpose already found for {:?}", len_cparam),
                        });
                        take_mut::take(&mut return_type, |ty| match ty {
                            LibReturnType::CDecl => match cmd_return_value {
                                CommandReturnValue::Result => LibReturnType::ResultVecUnknownLen,
                                CommandReturnValue::Void => LibReturnType::VecUnknownLen,
                                CommandReturnValue::Other => panic!("cannot handle return type {:?}", cmd_def.proto),
                            },
                            LibReturnType::ResultVecUnknownLen => LibReturnType::ResultMultiVecUnknownLen,
                            _ => panic!("already have return type of {:?}", ty),
                        });
                        return_type_name = match return_type {
                            LibReturnType::ResultMultiVecUnknownLen => "vk::Result".to_string(),
                            _ => inner_type_name,
                        };
                        continue;
                    }
                }
                if cparam.ty.base != CBaseType::Void
                    && cparam.ty.decoration == CDecoration::Pointer
                    && vparam.optional.is_none()
                    && vparam.len.is_some()
                {
                    let len_names: Vec<&str> = len_name.split("::").flat_map(|s| s.split("->")).collect();
                    let len_expr = if len_names.len() == 1 {
                        let len_index = decl
                            .parameters
                            .iter()
                            .position(|cparam| cparam.name == len_names.first().copied().unwrap())
                            .expect("missing len variable");
                        let len_cparam = &decl.parameters[len_index];
                        let len_expr = len_cparam.name.to_snake_case();
                        take_mut::take(&mut params[len_index].ty, |ty| match ty {
                            LibParamType::SliceLenShared { name, slice_infos } => {
                                LibParamType::SliceLenShared { name, slice_infos }
                            }
                            LibParamType::CDecl => LibParamType::SliceLenShared {
                                name: len_expr.clone(),
                                slice_infos: Vec::new(),
                            },
                            _ => {
                                panic!("purpose already found for {:?}", len_cparam);
                            }
                        });
                        params[i].ty = LibParamType::ReturnVec {
                            inner_type_name: inner_type_name.clone(),
                        };
                        len_expr
                    } else {
                        let len_names: Vec<String> = len_names
                            .iter()
                            .map(|s| get_rust_variable_name(s.to_snake_case().as_str()))
                            .collect();
                        params[i].ty = LibParamType::ReturnVec {
                            inner_type_name: inner_type_name.clone(),
                        };
                        len_names.join(".")
                    };
                    take_mut::take(&mut return_type, |ty| match ty {
                        LibReturnType::CDecl => match cmd_return_value {
                            CommandReturnValue::Result => LibReturnType::ResultVecKnownLen { len_expr },
                            CommandReturnValue::Other | CommandReturnValue::Void => {
                                panic!("cannot handle return type {:?}", cmd_def.proto)
                            }
                        },
                        _ => panic!("already have return type of {:?}", ty),
                    });
                    return_type_name = inner_type_name;
                    continue;
                }
            }

            // match single return type (last parameter only)
            // TODO: add to return type (as tuple?) when one already exists from previous parameter?
            if i == decl.parameters.len() - 1
                && (cparam.ty.decoration == CDecoration::Pointer
                    || cparam.ty.decoration == CDecoration::PointerToPointer)
                && (vparam.optional.is_none() || vparam.optional.as_deref() == Some("false,true"))
                && vparam.len.is_none()
                && cmd_return_value != CommandReturnValue::Other
                && return_type == LibReturnType::CDecl
            {
                let has_member_values = cparam
                    .ty
                    .base
                    .try_name()
                    .and_then(|name| self.type_by_name.get(name))
                    .and_then(|vtype| {
                        if let Some(alias) = vtype.alias.as_deref() {
                            self.type_by_name.get(alias)
                        } else {
                            Some(vtype)
                        }
                    })
                    .and_then(|vtype| match vtype.spec {
                        vk::TypeSpec::Members(ref members) => Some(members),
                        _ => None,
                    })
                    .map(|members| {
                        members
                            .iter()
                            .filter_map(|member| match member {
                                vk::TypeMember::Definition(ref def) => Some(def),
                                _ => None,
                            })
                            .any(|def| def.values.is_some())
                    })
                    .unwrap_or(false);
                if !has_member_values {
                    let mut inner_type_name = if cparam.ty.decoration == CDecoration::PointerToPointer {
                        format!("*mut {}", inner_type_name)
                    } else {
                        inner_type_name.clone()
                    };
                    if cparam.ty.decoration == CDecoration::Pointer {
                        match cparam.ty.base {
                            NAMED_TYPE_VKBOOL32 => {
                                inner_type_name = "bool".to_owned();
                                return_transform = LibReturnTransform::ToBool;
                            }
                            NAMED_TYPE_VKINSTANCE => {
                                inner_type_name = "Instance".to_owned();
                                return_transform = LibReturnTransform::ToInstance;
                            }
                            NAMED_TYPE_VKDEVICE => {
                                inner_type_name = "Device".to_owned();
                                return_transform = LibReturnTransform::ToDevice;
                            }
                            _ => {}
                        }
                    }
                    params[i].ty = LibParamType::ReturnObject {
                        inner_type_name: inner_type_name.clone(),
                    };
                    take_mut::take(&mut return_type, |ty| match ty {
                        LibReturnType::CDecl => match cmd_return_value {
                            CommandReturnValue::Result => match cmd_def.successcodes.as_deref() {
                                None | Some("VK_SUCCESS") => LibReturnType::ResultObject,
                                _ => LibReturnType::ResultEnumAndObject,
                            },
                            CommandReturnValue::Void => LibReturnType::Object,
                            CommandReturnValue::Other => panic!("cannot handle return type {:?}", cmd_def.proto),
                        },
                        _ => panic!("already have return type of {:?} for {:?}", ty, cmd_def.proto),
                    });
                    return_type_name = inner_type_name;
                    continue;
                }
            }

            // match reference
            if cparam.ty.base != CBaseType::Void && vparam.len.is_none() && !self.is_non_null_type(cparam.ty.base) {
                if cparam.ty.decoration == CDecoration::PointerToConst {
                    let is_optional = vparam.optional.as_deref() == Some("true");
                    params[i].ty = LibParamType::Ref {
                        inner_type_name,
                        is_optional,
                    };
                    continue;
                }
                if cparam.ty.decoration == CDecoration::Pointer && vparam.optional.is_none() {
                    params[i].ty = LibParamType::MutRef { inner_type_name };
                    continue;
                }
            }
        }

        if return_type == LibReturnType::CDecl && cmd_return_value == CommandReturnValue::Result {
            return_type = if cmd_def.successcodes.as_deref() == Some("VK_SUCCESS") {
                return_type_name = "()".to_owned();
                LibReturnType::ResultEmpty
            } else {
                LibReturnType::ResultEnum
            };
        }
        if return_type == LibReturnType::CDecl && cmd_return_value == CommandReturnValue::Void {
            return_type = LibReturnType::None;
            return_type_name = String::new();
        }

        (return_type, return_transform, return_type_name)
    }

    fn write_command(&self, w: &mut impl IoWrite, category: Category, cmd_name: &str) -> WriteResult {
        let cmd_info = self.cmd_info_by_name.get(cmd_name).expect("missing command info");
        let cmd_def = cmd_info.cmd_def;
        let decl = {
            let mut decl = c_parse_function_decl(cmd_def.code.as_str());
            let context = decl.proto.name;
            for param in decl.parameters.iter_mut() {
                take_mut::take(param, |v| self.rewrite_variable_decl(context, v));
            }
            decl
        };
        let mut params: Vec<LibParam> = decl
            .parameters
            .iter()
            .map(|cparam| LibParam {
                name: get_rust_variable_name(cparam.name.to_snake_case().as_str()),
                ty: LibParamType::CDecl,
            })
            .collect();
        let cmd_return_value = if decl.proto.ty.is_base_type(CBaseType::Named("VkResult")) {
            CommandReturnValue::Result
        } else if decl.proto.ty.is_base_type(CBaseType::Void) {
            CommandReturnValue::Void
        } else {
            CommandReturnValue::Other
        };

        let (return_type, return_transform, return_type_name) =
            self.wrap_command_arguments(category, &cmd_def, cmd_return_value, &decl, &mut params);

        let fn_name = cmd_name.skip_prefix(FN_PREFIX).to_snake_case();
        let fp_name = cmd_info
            .alias
            .unwrap_or(cmd_name)
            .skip_prefix(FN_PREFIX)
            .to_snake_case();

        let styles: &[LibCommandStyle] = match return_type {
            LibReturnType::VecUnknownLen | LibReturnType::ResultVecUnknownLen => &[LibCommandStyle::ToVecUnknownLen],
            LibReturnType::ResultVecKnownLen { .. } => &[
                LibCommandStyle::Default,
                LibCommandStyle::ToVecKnownLen,
                LibCommandStyle::Array,
                LibCommandStyle::Single,
            ],
            _ => &[LibCommandStyle::Default],
        };

        let any_generic = params
            .iter()
            .filter_map(|rparam| match rparam.ty {
                LibParamType::SliceLenSingle { ref slice_infos }
                | LibParamType::SliceLenShared { ref slice_infos, .. } => Some(slice_infos.iter()),
                _ => None,
            })
            .flatten()
            .any(SliceInfo::is_generic);

        for style in styles {
            write!(w, "pub unsafe fn {}", fn_name)?;
            match style {
                LibCommandStyle::Default => {}
                LibCommandStyle::ToVecUnknownLen | LibCommandStyle::ToVecKnownLen => {
                    write!(w, "_to_vec")?;
                }
                LibCommandStyle::Array => {
                    write!(w, "_array<const N: usize>")?;
                }
                LibCommandStyle::Single => {
                    write!(w, "_single")?;
                }
            }
            if any_generic {
                write!(w, "<T>")?;
            }
            write!(w, "(&self, ")?;

            for (cparam, rparam) in decl.parameters.iter().zip(params.iter()) {
                match rparam.ty {
                    LibParamType::CDecl => {
                        write!(
                            w,
                            "{}: {},",
                            rparam.name,
                            self.get_rust_parameter_type(&cparam.ty, Some("vk::")),
                        )?;
                    }
                    LibParamType::MemberHandle => {}
                    LibParamType::Bool => {
                        write!(w, "{}: bool,", rparam.name)?;
                    }
                    LibParamType::CStr { is_optional } => {
                        if is_optional {
                            write!(w, "{}: Option<&CStr>,", rparam.name)?;
                        } else {
                            write!(w, "{}: &CStr,", rparam.name)?;
                        }
                    }
                    LibParamType::NonOptional { ref inner_type_name } => {
                        write!(w, "{}: {},", rparam.name, inner_type_name)?;
                    }
                    LibParamType::SliceLenShared { ref slice_infos, .. } => {
                        let all_optional = slice_infos.iter().all(|slice_info| slice_info.is_optional);
                        if (*style == LibCommandStyle::Default && slice_infos.is_empty()) || all_optional {
                            write!(
                                w,
                                "{}: {},",
                                rparam.name,
                                self.get_rust_parameter_type(&cparam.ty, Some("vk::"))
                            )?;
                        }
                    }
                    LibParamType::SliceLenSingle { .. } => {}
                    LibParamType::Slice {
                        ref inner_type_name,
                        is_mutable,
                        is_optional,
                        ..
                    } => {
                        let type_name = slice_type_name(inner_type_name);
                        let mutability = if is_mutable { "mut" } else { "" };
                        if let LibCommandStyle::Single = style {
                            // bit of a hack: assume all slices share this length, so can be references
                            if is_optional {
                                write!(w, "{}: Option<&{}{}>,", rparam.name, mutability, type_name)?;
                            } else {
                                write!(w, "{}: &{}{},", rparam.name, mutability, type_name)?;
                            }
                        } else {
                            if is_optional {
                                write!(w, "{}: Option<&{}[{}]>,", rparam.name, mutability, type_name)?;
                            } else {
                                write!(w, "{}: &{}[{}],", rparam.name, mutability, type_name)?;
                            }
                        }
                    }
                    LibParamType::Ref {
                        ref inner_type_name,
                        is_optional,
                    } => {
                        if is_optional {
                            write!(w, "{}: Option<&{}>,", rparam.name, inner_type_name,)?;
                        } else {
                            write!(w, "{}: &{},", rparam.name, inner_type_name,)?;
                        }
                    }
                    LibParamType::MutRef { ref inner_type_name } => {
                        write!(w, "{}: &mut {},", rparam.name, inner_type_name,)?;
                    }
                    LibParamType::ReturnObject { .. } => {}
                    LibParamType::ReturnVecLenShared => {
                        write!(
                            w,
                            "{}: &mut {},",
                            rparam.name,
                            self.get_rust_type_name(cparam.ty.base, true, Some("vk::")),
                        )?;
                    }
                    LibParamType::ReturnVecLenSingle { .. } => {}
                    LibParamType::ReturnVec { ref inner_type_name } => {
                        if *style == LibCommandStyle::Default {
                            write!(w, "{}: *mut {},", rparam.name, inner_type_name)?;
                        }
                    }
                }
            }
            if return_transform == LibReturnTransform::ToDevice {
                writeln!(w, "version: vk::Version,")?;
            }
            write!(w, ")")?;
            match return_type {
                LibReturnType::None => {}
                LibReturnType::CDecl | LibReturnType::Object => {
                    write!(w, "-> {}", return_type_name,)?;
                }
                LibReturnType::VecUnknownLen => {
                    write!(w, "-> Vec<{}>", return_type_name)?;
                }
                LibReturnType::ResultEmpty
                | LibReturnType::ResultObject
                | LibReturnType::ResultEnum
                | LibReturnType::ResultMultiVecUnknownLen => match return_transform {
                    LibReturnTransform::ToInstance | LibReturnTransform::ToDevice => {
                        write!(w, "-> result::Result<{}, LoaderError>", return_type_name)?;
                    }
                    _ => {
                        write!(w, "-> Result<{}>", return_type_name)?;
                    }
                },
                LibReturnType::ResultVecUnknownLen | LibReturnType::ResultVecKnownLen { .. } => match style {
                    LibCommandStyle::Default => {
                        write!(w, "-> Result<()>")?;
                    }
                    LibCommandStyle::ToVecKnownLen | LibCommandStyle::ToVecUnknownLen => {
                        write!(w, "-> Result<Vec<{}>>", return_type_name)?;
                    }
                    LibCommandStyle::Array => {
                        write!(w, "-> Result<[{}; N]>", return_type_name)?;
                    }
                    LibCommandStyle::Single => {
                        write!(w, "-> Result<{}>", return_type_name)?;
                    }
                },
                LibReturnType::ResultEnumAndObject => {
                    write!(w, "-> Result<(vk::Result, {})>", return_type_name)?;
                }
            }
            writeln!(w, "{{")?;

            if cmd_name == "vkEnumerateInstanceVersion" {
                writeln!(w, r#"if let Some(fp) = self.fp_{} {{"#, fp_name)?;
            } else {
                writeln!(
                    w,
                    r#"let fp = self.fp_{}.expect("{} is not loaded");"#,
                    fp_name, cmd_name
                )?;
            }

            for (cparam, rparam) in decl.parameters.iter().zip(params.iter()) {
                match rparam.ty {
                    LibParamType::SliceLenShared {
                        ref name,
                        ref slice_infos,
                    } => {
                        if let LibCommandStyle::Single = style {
                            // bit of a hack: assume this is the result vector length, so set to 1
                            writeln!(w, "let {} = 1;", name)?;
                        } else {
                            let type_name = self.get_rust_parameter_type(&cparam.ty, Some("vk::"));
                            let first_non_optional =
                                slice_infos.iter().find(|slice_info| !slice_info.is_optional).unwrap();
                            writeln!(w, "let {} = {} as {};", name, first_non_optional.get_len(), type_name)?;
                            for slice_info in slice_infos {
                                if slice_info.name == first_non_optional.name {
                                    continue;
                                }
                                if slice_info.is_optional {
                                    writeln!(
                                        w,
                                        "if let Some(s) = {} {{ assert_eq!({}, {} as {}); }}",
                                        slice_info.name,
                                        name,
                                        slice_len("s", &slice_info.type_name),
                                        type_name
                                    )?;
                                } else {
                                    writeln!(w, "assert_eq!({}, {} as {});", name, slice_info.get_len(), type_name)?;
                                }
                            }
                        }
                    }
                    LibParamType::Slice {
                        len_expr: Some(ref len_expr),
                        is_optional,
                        ..
                    } => {
                        // bit of a hack, assume expression result is u32
                        if is_optional {
                            writeln!(
                                w,
                                "if let Some(s) = {} {{ assert_eq!(s.len() as u32, {}); }}",
                                rparam.name, len_expr
                            )?;
                        } else {
                            writeln!(w, "assert_eq!({}.len() as u32, {});", rparam.name, len_expr)?;
                        }
                    }
                    _ => {}
                }
            }

            let pass_start = match style {
                LibCommandStyle::ToVecUnknownLen => 0,
                _ => 1,
            };

            for pass_index in pass_start..2 {
                match return_type {
                    LibReturnType::None | LibReturnType::CDecl => {}
                    LibReturnType::ResultEmpty
                    | LibReturnType::ResultEnum
                    | LibReturnType::ResultMultiVecUnknownLen => {
                        write!(w, "let err = ")?;
                    }
                    LibReturnType::ResultObject | LibReturnType::ResultEnumAndObject => {
                        write!(w, "let mut res = MaybeUninit::<_>::uninit(); let err = ")?;
                    }
                    LibReturnType::Object => {
                        write!(w, "let mut res = MaybeUninit::<_>::uninit();")?;
                    }
                    LibReturnType::ResultVecUnknownLen => {
                        if pass_index == 0 {
                            write!(w, "let mut len = MaybeUninit::<_>::uninit(); let len_err = ")?;
                        } else {
                            write!(w, "let mut v = Vec::with_capacity(len as usize); let v_err = ")?;
                        }
                    }
                    LibReturnType::VecUnknownLen => {
                        if pass_index == 0 {
                            write!(w, "let mut len = MaybeUninit::<_>::uninit();")?;
                        } else {
                            write!(w, "let mut v = Vec::with_capacity(len as usize);")?;
                        }
                    }
                    LibReturnType::ResultVecKnownLen { ref len_expr } => match style {
                        LibCommandStyle::Default => {
                            write!(w, "let v_err = ")?;
                        }
                        LibCommandStyle::ToVecKnownLen | LibCommandStyle::ToVecUnknownLen => {
                            write!(
                                w,
                                "let mut v = Vec::with_capacity({0} as usize); v.set_len({0} as usize); let v_err = ",
                                len_expr
                            )?;
                        }
                        LibCommandStyle::Array => {
                            write!(
                                w,
                                "assert_eq!({}, N as u32); let mut v = MaybeUninit::<_>::uninit(); let v_err = ",
                                len_expr
                            )?;
                        }
                        LibCommandStyle::Single => {
                            if !params
                                .iter()
                                .any(|rparam| matches!(rparam.ty, LibParamType::SliceLenShared { .. }))
                            {
                                write!(w, "assert_eq!({}, 1);", len_expr)?;
                            }
                            write!(w, "let mut v = MaybeUninit::<_>::uninit(); let v_err = ")?;
                        }
                    },
                }

                write!(w, "(fp)(")?;
                for (cparam, rparam) in decl.parameters.iter().zip(params.iter()) {
                    match rparam.ty {
                        LibParamType::CDecl => {
                            if cparam.ty.array_size.is_some() {
                                write!(w, "{}.as_ptr()", rparam.name)?;
                            } else {
                                write!(w, "{}", rparam.name)?;
                            }
                        }
                        LibParamType::MutRef { .. } | LibParamType::ReturnVecLenShared => {
                            write!(w, "{}", rparam.name)?;
                        }
                        LibParamType::MemberHandle => {
                            write!(w, "Some(self.handle)")?;
                        }
                        LibParamType::Bool => {
                            write!(w, "if {} {{ vk::TRUE }} else {{ vk::FALSE }}", rparam.name)?;
                        }
                        LibParamType::CStr { is_optional } => {
                            if is_optional {
                                write!(w, "{}.map_or(ptr::null(), |s| s.as_ptr())", rparam.name)?;
                            } else {
                                write!(w, "{}.as_ptr()", rparam.name)?;
                            }
                        }
                        LibParamType::NonOptional { .. } => {
                            write!(w, "Some({})", rparam.name)?;
                        }
                        LibParamType::SliceLenShared { ref name, .. } => {
                            write!(w, "{}", name)?;
                        }
                        LibParamType::SliceLenSingle { ref slice_infos } => {
                            let type_name = self.get_rust_parameter_type(&cparam.ty, Some("vk::"));
                            write!(w, "{} as {}", slice_infos.first().unwrap().get_len(), type_name)?;
                        }
                        LibParamType::Slice {
                            is_mutable,
                            is_optional,
                            ref inner_type_name,
                            ..
                        } => {
                            if let LibCommandStyle::Single = style {
                                if is_optional {
                                    write!(w, "{}.map_or(ptr::null(), |r| r)", rparam.name)?;
                                } else {
                                    write!(w, "{}", rparam.name)?;
                                }
                            } else {
                                write!(
                                    w,
                                    "{}",
                                    slice_as_ptr(&rparam.name, inner_type_name, is_mutable, is_optional)
                                )?;
                            }
                        }
                        LibParamType::Ref { is_optional, .. } => {
                            if is_optional {
                                write!(w, "{}.map_or(ptr::null(), |r| r)", rparam.name)?;
                            } else {
                                write!(w, "{}", rparam.name)?;
                            }
                        }
                        LibParamType::ReturnObject { .. } => {
                            write!(w, "res.as_mut_ptr()")?;
                        }
                        LibParamType::ReturnVecLenSingle { .. } => {
                            if pass_index == 0 {
                                write!(w, "len.as_mut_ptr()")?;
                            } else {
                                write!(w, "&mut len")?;
                            }
                        }
                        LibParamType::ReturnVec { .. } => match style {
                            LibCommandStyle::Default => {
                                write!(w, "{}", rparam.name)?;
                            }
                            LibCommandStyle::ToVecUnknownLen | LibCommandStyle::ToVecKnownLen => {
                                if pass_index == 0 {
                                    write!(w, "ptr::null_mut()")?;
                                } else {
                                    write!(w, "v.as_mut_ptr()")?;
                                }
                            }
                            LibCommandStyle::Array => {
                                write!(w, "v.as_mut_ptr() as *mut _")?;
                            }
                            LibCommandStyle::Single => {
                                write!(w, "v.as_mut_ptr()")?;
                            }
                        },
                    }
                    write!(w, ",")?;
                }
                match return_type {
                    LibReturnType::CDecl => writeln!(w, ")")?,
                    _ => writeln!(w, ");")?,
                }

                match return_type {
                    LibReturnType::CDecl => {}
                    LibReturnType::None => {}
                    LibReturnType::ResultEmpty => {
                        write!(w, "match err {{ vk::Result::SUCCESS => Ok(()), _ => Err(err) }}")?;
                    }
                    LibReturnType::ResultEnum | LibReturnType::ResultMultiVecUnknownLen => {
                        let ok_matches = if let Some(successcodes) = cmd_def.successcodes.as_deref() {
                            let matches: Vec<String> = successcodes
                                .split(',')
                                .map(|s| format!("vk::Result::{}", s.skip_prefix("VK_")))
                                .collect();
                            matches.join("|")
                        } else {
                            "vk::Result::SUCCESS".to_owned()
                        };
                        write!(w, "match err {{ {} => Ok(err), _ => Err(err) }}", ok_matches)?;
                    }
                    LibReturnType::ResultObject => {
                        write!(
                            w,
                            "match err {{ vk::Result::SUCCESS => Ok(res.assume_init()), _ => Err(err) }}",
                        )?;
                    }
                    LibReturnType::ResultEnumAndObject => {
                        let matches: Vec<String> = cmd_def
                            .successcodes
                            .as_deref()
                            .unwrap()
                            .split(',')
                            .map(|s| format!("vk::Result::{}", s.skip_prefix("VK_")))
                            .collect();
                        write!(
                            w,
                            "match err {{ {} => Ok((err, res.assume_init())), _ => Err(err) }}",
                            matches.join("|"),
                        )?;
                    }
                    LibReturnType::Object => {
                        write!(w, "res.assume_init()")?;
                    }
                    LibReturnType::ResultVecUnknownLen => {
                        if pass_index == 0 {
                            write!(w, "if len_err != vk::Result::SUCCESS {{ return Err(len_err) }} let mut len = len.assume_init();")?;
                        } else {
                            write!(w, "v.set_len(len as usize); match v_err {{ vk::Result::SUCCESS => Ok(v), _ => Err(v_err) }}")?;
                        }
                    }
                    LibReturnType::VecUnknownLen => {
                        if pass_index == 0 {
                            write!(w, "let mut len = len.assume_init();")?;
                        } else {
                            write!(w, "v.set_len(len as usize); v")?;
                        }
                    }
                    LibReturnType::ResultVecKnownLen { .. } => match style {
                        LibCommandStyle::Default => {
                            write!(w, "match v_err {{ vk::Result::SUCCESS => Ok(()), _ => Err(v_err) }}")?;
                        }
                        LibCommandStyle::ToVecUnknownLen | LibCommandStyle::ToVecKnownLen => {
                            write!(w, "match v_err {{ vk::Result::SUCCESS => Ok(v), _ => Err(v_err) }}")?;
                        }
                        LibCommandStyle::Array | LibCommandStyle::Single => {
                            write!(
                                w,
                                "match v_err {{ vk::Result::SUCCESS => Ok(v.assume_init()), _ => Err(v_err) }}"
                            )?;
                        }
                    },
                }

                if return_type != LibReturnType::None && pass_index != 0 {
                    match return_transform {
                        LibReturnTransform::None => {}
                        LibReturnTransform::ToBool => writeln!(w, ".map(|r| r != vk::FALSE)")?,
                        LibReturnTransform::ToInstance => writeln!(
                            w,
                            ".map_err(LoaderError::Vulkan).and_then(|r| Instance::load(&self, r, p_create_info))"
                        )?,
                        LibReturnTransform::ToDevice => writeln!(
                            w,
                            ".map_err(LoaderError::Vulkan).and_then(|r| Device::load(&self, r, p_create_info, version))"
                        )?,
                    }
                }
            }

            if cmd_name == "vkEnumerateInstanceVersion" {
                writeln!(w, "}} else {{ Ok(vk::Version::default()) }}")?;
            }

            writeln!(w, " }}")?;
        }

        Ok(())
    }

    fn write_command_ref_condition(
        &self,
        category: Category,
        cmd_ref: CommandRef,
        w: &mut impl IoWrite,
    ) -> WriteResult {
        match cmd_ref {
            CommandRef::Feature(version) => write!(
                w,
                "version >= vk::Version::from_raw_parts({}, {}, 0)",
                version.major, version.minor
            )?,
            CommandRef::Extension(name) => {
                let ext = self.extension_by_name.get(name).expect("missing extension");
                if category == Category::Device && ext.get_category() == Category::Instance {
                    write!(
                        w,
                        "instance.extensions.{}",
                        name.skip_prefix(CONST_PREFIX).to_snake_case()
                    )?;
                } else {
                    write!(w, "extensions.{}", name.skip_prefix(CONST_PREFIX).to_snake_case())?;
                }
            }
        }
        Ok(())
    }

    fn write_command_ref_condition_array(&self, info: &CommandInfo, w: &mut impl IoWrite) -> WriteResult {
        let category = info.category.unwrap();
        let mut is_first = true;
        for cmd_ref_pair in info.refs.iter() {
            let want_brackets = (info.refs.len() > 1) && cmd_ref_pair.secondary.is_some();
            if is_first {
                is_first = false;
            } else {
                write!(w, " || ")?;
            }
            if want_brackets {
                write!(w, "(")?;
            }
            self.write_command_ref_condition(category, cmd_ref_pair.primary, w)?;
            if let Some(secondary) = cmd_ref_pair.secondary {
                write!(w, " && ")?;
                self.write_command_ref_condition(category, secondary, w)?;
            }
            if want_brackets {
                write!(w, ")")?;
            }
        }
        Ok(())
    }

    fn write_struct(&self, category: Category, w: &mut impl IoWrite) -> WriteResult {
        let all_supported_extensions: Vec<&vk::Extension> = self
            .registry
            .0
            .iter()
            .filter_map(|ext_child| match ext_child {
                vk::RegistryChild::Extensions(extensions) => Some(extensions),
                _ => None,
            })
            .flat_map(|extensions| extensions.children.iter())
            .filter(|ext| ext.is_supported() && !ext.is_blacklisted())
            .collect();

        let extensions: Vec<&vk::Extension> = all_supported_extensions
            .iter()
            .copied()
            .filter(|ext| ext.get_category() == category)
            .collect();

        if !extensions.is_empty() {
            writeln!(w, "#[derive(Debug, Copy, Clone)]")?;
            writeln!(w, "pub struct {}Extensions {{", category)?;
            writeln!(w, "pub core_version: vk::Version,")?;
            for ext in extensions.iter() {
                let var_name = ext.name.skip_prefix(CONST_PREFIX).to_snake_case();
                writeln!(w, "pub {}: bool,", var_name)?;
            }
            writeln!(w, "}}")?;

            writeln!(w, "impl {}Extensions {{", category)?;

            writeln!(w, "fn enable_by_name(&mut self, name: &CStr) {{")?;
            writeln!(w, "match name.to_bytes() {{")?;
            for ext in extensions.iter() {
                let var_name = ext.name.skip_prefix(CONST_PREFIX).to_snake_case();
                writeln!(w, r#"b"{}" => self.{} = true,"#, ext.name, var_name)?;
            }
            writeln!(w, "_ => {{}}, }} }}")?;

            writeln!(
                w,
                "pub fn new(core_version: vk::Version) -> Self {{ Self {{ core_version,"
            )?;
            for ext in extensions.iter() {
                let var_name = ext.name.skip_prefix(CONST_PREFIX).to_snake_case();
                writeln!(w, "{}: false,", var_name)?;
            }
            writeln!(w, "}} }}")?;

            writeln!(
                w,
                "pub fn from_properties(core_version: vk::Version, properties: &[vk::ExtensionProperties]) -> Self {{\
                 let mut ext = Self::new(core_version);\
                 for ep in properties.iter() {{\
                    if ep.extension_name.iter().any(|&c| c == 0) {{\
                        let name = unsafe {{ CStr::from_ptr(ep.extension_name.as_ptr()) }};\
                        ext.enable_by_name(name);\
                    }}\
                 }}\
                 ext }}"
            )?;

            for ext in all_supported_extensions.iter() {
                let mut queue: VecDeque<&'a vk::Extension> = VecDeque::new();
                let mut queued_names: HashSet<&'a str> = HashSet::new();
                queue.push_back(ext);
                queued_names.insert(ext.name.as_str());

                let mut dependencies: Vec<&'a vk::Extension> = Vec::new();
                while let Some(ext) = queue.pop_front() {
                    for req in ext
                        .requires
                        .as_deref()
                        .iter()
                        .flat_map(|s| s.split(','))
                        .filter_map(|s| self.extension_by_name.get(s))
                    {
                        if queued_names.insert(req.name.as_str()) {
                            dependencies.push(req);
                            queue.push_back(req);
                        }
                    }
                }
                dependencies
                    .retain(|ext| ext.is_supported() && !ext.is_blacklisted() && ext.get_category() == category);

                if ext.get_category() == category || !dependencies.is_empty() {
                    let var_name = ext.name.skip_prefix(CONST_PREFIX).to_snake_case();
                    let promoted_to_version = ext.promotedto.as_deref().and_then(|s| c_try_parse_version(s));
                    let min_version = ext.requires_core.as_deref().map(|s| c_parse_version_decimal(s));

                    let mut support_checks = Vec::new();
                    if let Some(version) = min_version {
                        support_checks.push(format!(
                            "self.core_version >= vk::Version::from_raw_parts({}, {}, 0)",
                            version.0, version.1
                        ))
                    }
                    if ext.get_category() == category {
                        support_checks.push(if let Some(version) = promoted_to_version {
                            format!(
                                "self.{} || self.core_version >= vk::Version::from_raw_parts({}, {}, 0)",
                                var_name, version.0, version.1
                            )
                        } else {
                            format!("self.{}", var_name)
                        });
                    }
                    support_checks.extend(dependencies.iter().map(|ext| {
                        format!(
                            "self.supports_{}()",
                            ext.name.as_str().skip_prefix(CONST_PREFIX).to_snake_case()
                        )
                    }));
                    if support_checks.len() > 1 {
                        for check in support_checks.iter_mut() {
                            if check.contains("||") {
                                *check = format!("({})", check);
                            }
                        }
                    }
                    writeln!(
                        w,
                        "pub fn supports_{}(&self) -> bool {{ {} }}",
                        var_name,
                        support_checks.join("&&")
                    )?;

                    writeln!(w, "pub fn enable_{}(&mut self) {{", var_name,)?;
                    if ext.get_category() == category {
                        if let Some(version) = promoted_to_version {
                            writeln!(
                                w,
                                "if self.core_version < vk::Version::from_raw_parts({}, {}, 0) {{ self.{} = true; }}",
                                version.0, version.1, var_name
                            )?;
                        } else {
                            writeln!(w, "self.{} = true;", var_name)?;
                        }
                    }
                    for ext in dependencies {
                        writeln!(
                            w,
                            "self.enable_{}();",
                            ext.name.as_str().skip_prefix(CONST_PREFIX).to_snake_case()
                        )?;
                    }
                    writeln!(w, "}}")?;
                }
            }

            writeln!(
                w,
                "pub fn to_name_vec(&self) -> Vec<&'static CStr> {{ let mut v = Vec::new();"
            )?;
            for ext in extensions.iter() {
                let var_name = ext.name.skip_prefix(CONST_PREFIX).to_snake_case();
                writeln!(
                    w,
                    r#"if self.{} {{ v.push(unsafe {{ CStr::from_bytes_with_nul_unchecked(b"{}\0") }}) }}"#,
                    var_name, ext.name
                )?;
            }
            writeln!(w, "v }}")?;

            writeln!(w, "}}")?;
        }

        writeln!(w, "#[derive(Copy, Clone)]")?;
        writeln!(w, "pub struct {} {{", category)?;
        match category {
            Category::Loader => {}
            Category::Instance => writeln!(w, "pub handle: vk::Instance,")?,
            Category::Device => writeln!(w, "pub handle: vk::Device,")?,
        }
        if !extensions.is_empty() {
            writeln!(w, "pub extensions: {}Extensions,", category)?;
        }
        for name in self.cmd_names.iter().filter(|&name| {
            let info = self.cmd_info_by_name.get(name).expect("missing command info");
            info.alias.is_none() && info.category == Some(category)
        }) {
            let name_part = name.skip_prefix(FN_PREFIX);
            let fn_name = name_part.to_snake_case();
            writeln!(w, "pub fp_{}: Option<vk::Fn{}>,", fn_name, name_part)?;
        }
        writeln!(w, "}}")?;

        writeln!(w, "impl {} {{", category)?;
        match category {
            Category::Loader => {
                writeln!(
                    w,
                    "pub fn new() -> LoaderResult<Self> {{\
                     let lib = LIB.as_ref().map_err(|e| e.clone())?;\
                     unsafe {{\
                     let f = |name: &CStr| lib.get_instance_proc_addr(name);\
                     Ok(Self {{"
                )?;
            }
            Category::Instance => {
                writeln!(w, "#[allow(clippy::cognitive_complexity, clippy::nonminimal_bool)]")?;
                writeln!(
                    w,
                    "pub unsafe fn load(loader: &Loader, instance: vk::Instance, create_info: &vk::InstanceCreateInfo) -> LoaderResult<Self> {{\
                     let version = create_info.p_application_info.as_ref().map(|app_info| app_info.api_version).unwrap_or_default();\
                     let mut extensions = {}Extensions::new(version);", category)?;
                writeln!(w,
                    "if create_info.enabled_extension_count != 0 {{\
                     for &name_ptr in slice::from_raw_parts(create_info.pp_enabled_extension_names, create_info.enabled_extension_count as usize) {{\
                     extensions.enable_by_name(&CStr::from_ptr(name_ptr)); }} }}")?;
                writeln!(
                    w,
                    "let f = |name: &CStr| loader.get_instance_proc_addr(Some(instance), name);\
                     Ok(Self {{ handle: instance, extensions,"
                )?;
            }
            Category::Device => {
                writeln!(w, "#[allow(clippy::cognitive_complexity, clippy::nonminimal_bool)]")?;
                writeln!(
                    w,
                    "pub unsafe fn load(instance: &Instance, device: vk::Device, create_info: &vk::DeviceCreateInfo, version: vk::Version) -> LoaderResult<Self> {{\
                     let mut extensions = {}Extensions::new(version);", category)?;
                writeln!(w,
                    "if create_info.enabled_extension_count != 0 {{\
                     for &name_ptr in slice::from_raw_parts(create_info.pp_enabled_extension_names, create_info.enabled_extension_count as usize) {{\
                     extensions.enable_by_name(&CStr::from_ptr(name_ptr)); }} }}")?;
                writeln!(
                    w,
                    "let f = |name: &CStr| instance.get_device_proc_addr(device, name);\
                     Ok(Self {{ handle: device, extensions,"
                )?;
            }
        }

        for (name, info) in self.cmd_names.iter().filter_map(|&name| {
            let info = self.cmd_info_by_name.get(name).expect("missing command info");
            if info.alias.is_none() && info.category == Some(category) {
                Some((name, info))
            } else {
                None
            }
        }) {
            let fn_name = name.skip_prefix(FN_PREFIX).to_snake_case();
            writeln!(w, "fp_{}:", fn_name)?;
            let always_load = info.is_core_vulkan_1_0() || category == Category::Loader;
            if name == "vkGetInstanceProcAddr" {
                writeln!(w, "Some(lib.fp_{})", fn_name)?;
            } else {
                if !always_load {
                    writeln!(w, "if ")?;
                    self.write_command_ref_condition_array(info, w)?;
                }
                writeln!(
                    w,
                    r#"{{ let fp = f(CStr::from_bytes_with_nul_unchecked(b"{}\0"));"#,
                    name
                )?;
                let is_core = info
                    .refs
                    .iter()
                    .all(|cmd_ref_pair| matches!(cmd_ref_pair.primary, CommandRef::Feature(_)));
                if is_core && category != Category::Loader {
                    writeln!(
                        w,
                        r#"if fp.is_none() {{ return Err(LoaderError::MissingSymbol("{}".to_string())); }}"#,
                        name
                    )?;
                }
                writeln!(w, "fp.map(|f| mem::transmute(f)) }}")?;
                if !always_load {
                    for (other_name, other_info) in self.cmd_names.iter().filter_map(|&other_name| {
                        let other_info = self.cmd_info_by_name.get(other_name).expect("missing command info");
                        if other_info.alias == Some(name) {
                            Some((other_name, other_info))
                        } else {
                            None
                        }
                    }) {
                        writeln!(w, "else if ")?;
                        self.write_command_ref_condition_array(other_info, w)?;
                        writeln!(
                            w,
                            r#"{{ let fp = f(CStr::from_bytes_with_nul_unchecked(b"{}\0"));"#,
                            other_name
                        )?;
                        writeln!(w, "fp.map(|f| mem::transmute(f)) }}")?;
                    }
                    writeln!(w, "else {{ None }}")?;
                }
            }
            writeln!(w, ",")?;
        }
        match category {
            Category::Loader => {
                writeln!(w, "}}) }} }}")?;
            }
            Category::Instance | Category::Device => {
                writeln!(w, "}}) }}")?;
            }
        }
        for name in self.cmd_names.iter().filter(|&name| {
            let info = self.cmd_info_by_name.get(name).expect("missing command info");
            info.category == Some(category)
        }) {
            self.write_command(w, category, name)?;
        }
        writeln!(w, "}}")?;

        Ok(())
    }

    fn write_lib(&self, path: &Path) -> WriteResult {
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);

        let mut header_version = None;
        for ty in self.get_type_iterator() {
            if Some("define") == ty.category.as_deref() {
                if let vk::TypeSpec::Code(ref type_code) = ty.spec {
                    let prefix = "#define VK_HEADER_VERSION";
                    if let Some(offset) = type_code.code.find(&prefix) {
                        header_version =
                            Some(type_code.code[(offset + prefix.len())..].trim_matches(char::is_whitespace));
                        break;
                    }
                }
            }
        }
        if let Some(v) = header_version {
            writeln!(&mut w, "//! Generated from vk.xml with `VK_HEADER_VERSION` {}", v)?;
        }

        write!(&mut w, "{}", include_str!("lib_prefix.rs"))?;
        self.write_struct(Category::Loader, &mut w)?;
        self.write_struct(Category::Instance, &mut w)?;
        self.write_struct(Category::Device, &mut w)?;
        write!(&mut w, "{}", include_str!("lib_postfix.rs"))?;

        Ok(())
    }
}

fn main() -> WriteResult {
    let args: Vec<String> = env::args().collect();
    let xml_file_name = &args.get(1).expect("missing XML filename as argument").as_str();
    let (registry, errors) = vk::parse_file(Path::new(xml_file_name))?;
    for error in &errors {
        println!("Parser error: {:?}", error);
    }

    let generator = Generator::new(&registry);
    generator.write_vk(Path::new("spark/src/vk.rs"))?;
    generator.write_builder(Path::new("spark/src/builder.rs"))?;
    generator.write_lib(Path::new("spark/src/lib.rs"))?;

    Spawn::new("cargo").arg("fmt").current_dir("spark").output()?;

    Ok(())
}
