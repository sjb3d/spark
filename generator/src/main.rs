mod c_parse;

use crate::c_parse::*;
use heck::{CamelCase, ShoutySnakeCase, SnakeCase};
use std::collections::{HashMap, HashSet};
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
const VERSION_PREFIX: &str = "VK_VERSION_";

trait SkipPrefix {
    fn skip_prefix(&self, prefix: &str) -> &str;
}

impl SkipPrefix for str {
    fn skip_prefix(&self, prefix: &str) -> &str {
        let len = prefix.len();
        if &self[0..len] != prefix {
            panic!("cannot remove prefix {} from {}", prefix, self);
        }
        &self[len..]
    }
}

trait AsRefStr {
    fn as_ref_str(&self) -> Option<&str>;
}

impl AsRefStr for Option<String> {
    fn as_ref_str(&self) -> Option<&str> {
        self.as_ref().map(|s| s.as_str())
    }
}

trait GetTypeName {
    fn get_type_name(&self) -> &str;
}

impl GetTypeName for vk::Type {
    fn get_type_name(&self) -> &str {
        if self.alias.is_some() {
            self.name.as_ref().expect("missing bitmask or enum alias type name")
        } else {
            match self.category.as_ref_str() {
                Some("bitmask") | Some("handle") | Some("funcpointer") => {
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
                Some("enum") | Some("struct") | Some("union") => self.name.as_ref().expect("missing struct name"),
                _ => panic!("cannot get type name for {:?}", self),
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum CommandReturnValue {
    Void,
    Result,
    Other,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum EnumType {
    Bitmask,
    Value,
}

enum EnumEntryValue {
    Number { value: i32, comment: Option<String> },
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Group<'a> {
    Loader,
    Instance,
    Device,
    InstanceExtension(&'a str),
    DeviceExtension(&'a str),
}

impl<'a> fmt::Display for Group<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Group::Loader => write!(f, "Loader"),
            Group::Instance => write!(f, "Instance"),
            Group::Device => write!(f, "Device"),
            Group::InstanceExtension(s) => write!(f, "{}", s.skip_prefix(CONST_PREFIX).to_camel_case()),
            Group::DeviceExtension(s) => write!(f, "{}", s.skip_prefix(CONST_PREFIX).to_camel_case()),
        }
    }
}

struct VersionNames<'a> {
    pub version: &'a str,
    pub names: Vec<&'a str>,
}

struct GroupNames<'a> {
    pub group: Group<'a>,
    pub versions: Vec<VersionNames<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SliceInfo {
    name: String,
    type_name: String,
    is_optional: bool,
}

#[derive(Debug, PartialEq, Eq)]
enum LibParamType {
    CDecl,
    Bool,
    MemberHandle,
    CStr,
    NonOptional { inner_type_name: String },
    SharedSliceLen { name: String, slice_infos: Vec<SliceInfo> },
    SingleSliceLen { slice_infos: Vec<SliceInfo> },
    Slice { inner_type_name: String, is_optional: bool },
    Ref { inner_type_name: String, is_optional: bool },
    MutRef { inner_type_name: String },
    ReturnObject { inner_type_name: String },
    ReturnVecLen { slice_name: String },
    ReturnVec { inner_type_name: String },
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
}

#[derive(Debug, PartialEq, Eq)]
enum LibReturnTransform {
    None,
    ToInstance,
    ToDevice,
    ToBool,
}

#[derive(Debug, PartialEq, Eq)]
enum LibCommandStyle {
    Default,
    ToVecUnknownLen,
    ToVecKnownLen,
    Array,
    Single,
}

struct Generator<'a> {
    registry: &'a vk::Registry,
    type_by_name: HashMap<&'a str, &'a vk::Type>,
    tag_names: HashSet<&'a str>,
    bitmask_from_value: HashMap<&'a str, &'a str>,
    enums_by_name: HashMap<&'a str, Vec<&'a vk::Enum>>,
    constant_enums: Vec<&'a vk::Enum>,
    extension_by_enum_name: HashMap<&'a str, &'a vk::Extension>,
    cmd_def_by_name: HashMap<&'a str, &'a vk::CommandDefinition>,
    group_names: Vec<GroupNames<'a>>,
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

    fn collect_types(&mut self) {
        for ty in self.get_type_iterator() {
            let category = ty.category.as_ref_str();
            match category {
                Some("bitmask") | Some("enum") | Some("handle") | Some("funcpointer") | Some("struct")
                | Some("union") => {
                    let name = ty.get_type_name();
                    if self.type_by_name.insert(name, ty).is_some() {
                        panic!("duplicate type name from {:?}", ty)
                    }
                    if category == Some("bitmask") {
                        if let Some(requires) = ty.requires.as_ref_str() {
                            if self.bitmask_from_value.insert(requires, name).is_some() {
                                panic!("duplicate value for bitmask {}", requires);
                            }
                        }
                    }
                }
                _ => {}
            }
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
            vk::EnumSpec::Alias { ref extends, .. } => extends.as_ref_str(),
            vk::EnumSpec::Offset { ref extends, .. } => Some(extends.as_str()),
            vk::EnumSpec::Bitpos { ref extends, .. } => extends.as_ref_str(),
            vk::EnumSpec::Value { ref extends, .. } => extends.as_ref_str(),
            vk::EnumSpec::None => None,
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
                vk::RegistryChild::Enums(enums) => match enums.kind.as_ref_str() {
                    Some("enum") | Some("bitmask") => {
                        let name = enums.name.as_ref_str().expect("missing enum name");
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
                    for en in feature
                        .children
                        .iter()
                        .filter_map(|ext_child| match ext_child {
                            vk::ExtensionChild::Require { items, .. } => Some(items),
                            _ => None,
                        })
                        .flat_map(|items| items.iter())
                        .filter_map(|item| match item {
                            vk::InterfaceItem::Enum(en) => Some(en),
                            _ => None,
                        })
                    {
                        self.collect_extension_enum(en);
                    }
                }
                vk::RegistryChild::Extensions(extensions) => {
                    for ext in &extensions.children {
                        for en in ext
                            .children
                            .iter()
                            .filter_map(|ext_child| match ext_child {
                                vk::ExtensionChild::Require { items, .. } => Some(items),
                                _ => None,
                            })
                            .flat_map(|items| items.iter())
                            .filter_map(|item| match item {
                                vk::InterfaceItem::Enum(en) => Some(en),
                                _ => None,
                            })
                        {
                            self.collect_extension_enum(en);
                            self.extension_by_enum_name.insert(en.name.as_str(), ext);
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
                            cmd_aliases.push((name.as_str(), alias.as_str()));
                        }
                        vk::Command::Definition(cmd_def) => {
                            self.cmd_def_by_name.insert(cmd_def.proto.name.as_str(), cmd_def);
                        }
                    }
                }
            }
        }
        for (name, alias) in &cmd_aliases {
            let cmd_def = self
                .cmd_def_by_name
                .get(alias)
                .cloned()
                .expect("command alias not found");
            self.cmd_def_by_name.insert(name, cmd_def);
        }
    }

    fn get_command_group(&self, name: &str) -> Option<Group<'a>> {
        match name {
            "vkGetInstanceProcAddr" => None,
            "vkCreateInstance"
            | "vkEnumerateInstanceLayerProperties"
            | "vkEnumerateInstanceExtensionProperties"
            | "vkEnumerateInstanceVersion" => Some(Group::Loader),
            "vkGetDeviceProcAddr" => Some(Group::Instance),
            _ => {
                let cmd_def = self.cmd_def_by_name.get(name).cloned().expect("command not found");
                let is_first_param_from_device = cmd_def
                    .params
                    .get(0)
                    .and_then(|param| param.definition.type_name.as_ref())
                    .map(|type_name| match type_name.as_str() {
                        "VkDevice" | "VkCommandBuffer" | "VkQueue" => true,
                        _ => false,
                    })
                    .unwrap_or(false);
                if is_first_param_from_device {
                    Some(Group::Device)
                } else {
                    Some(Group::Instance)
                }
            }
        }
    }

    fn collect_group(&self, group: Group<'a>) -> GroupNames<'a> {
        let mut group_names = GroupNames {
            group,
            versions: Vec::new(),
        };
        for feature in self
            .registry
            .0
            .iter()
            .filter_map(|registry_child| match registry_child {
                vk::RegistryChild::Feature(feature) => Some(feature),
                _ => None,
            })
        {
            let mut names = Vec::new();
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
                if Some(group) == self.get_command_group(name) {
                    names.push(name);
                }
            }
            if !names.is_empty() {
                let version = feature.name.as_str();
                group_names.versions.push(VersionNames { version, names });
            }
        }
        group_names
    }

    fn collect_groups(&mut self) {
        let entry_group = self.collect_group(Group::Loader);
        let instance_group = self.collect_group(Group::Instance);
        let device_group = self.collect_group(Group::Device);
        self.group_names.push(entry_group);
        self.group_names.push(instance_group);
        self.group_names.push(device_group);

        let mut feature_names = Vec::new();
        for registry_child in &self.registry.0 {
            match registry_child {
                vk::RegistryChild::Feature(feature) => {
                    feature_names.push(feature.name.as_str());
                }
                vk::RegistryChild::Extensions(extensions) => {
                    for extension in &extensions.children {
                        if extension.supported.as_ref_str() == Some("vulkan") {
                            let mut feature_commands: Vec<Vec<_>> = vec![Vec::new(); feature_names.len()];
                            for ext_child in &extension.children {
                                if let vk::ExtensionChild::Require { feature, items, .. } = ext_child {
                                    let feature_index: usize = feature
                                        .as_ref()
                                        .and_then(|s| feature_names.iter().position(|&n| s == n))
                                        .unwrap_or(0);
                                    for item in items {
                                        if let vk::InterfaceItem::Command { name, .. } = item {
                                            feature_commands[feature_index].push(name.as_str());
                                        }
                                    }
                                }
                            }

                            let ext_type = extension.ext_type.as_ref_str().expect("missing ext_type");
                            let group = match ext_type {
                                "instance" => Group::InstanceExtension(extension.name.as_str()),
                                "device" => Group::DeviceExtension(extension.name.as_str()),
                                _ => panic!("unknown extension type {:?}", extension),
                            };

                            let mut group_names = GroupNames {
                                group,
                                versions: Vec::new(),
                            };
                            for (version, names) in feature_names.iter().zip(feature_commands.drain(..)) {
                                if !names.is_empty() {
                                    group_names.versions.push(VersionNames { version, names });
                                }
                            }
                            self.group_names.push(group_names);
                        }
                    }
                }
                _ => {}
            }
        }
    }

    pub fn new(registry: &'a vk::Registry) -> Self {
        let mut gen = Self {
            registry,
            type_by_name: HashMap::new(),
            tag_names: HashSet::new(),
            bitmask_from_value: HashMap::new(),
            enums_by_name: HashMap::new(),
            constant_enums: Vec::new(),
            extension_by_enum_name: HashMap::new(),
            cmd_def_by_name: HashMap::new(),
            group_names: Vec::new(),
        };
        gen.collect_types();
        gen.collect_tags();
        gen.collect_enums();
        gen.collect_commands();
        gen.collect_groups();
        gen
    }

    fn is_non_null_type(&self, type_name: &str) -> bool {
        self.type_by_name
            .get(type_name)
            .and_then(|ref ty| ty.category.as_ref_str())
            .map(|s| s == "funcpointer" || s == "handle")
            .unwrap_or(false)
    }

    fn get_rust_type_name(&self, type_name: &str, use_option: bool, vk_prefix: Option<&str>) -> String {
        match type_name {
            "void" => "c_void".to_owned(),
            "char" => "c_char".to_owned(),
            "int" => "c_int".to_owned(),
            "float" => "f32".to_owned(),
            "uint8_t" => "u8".to_owned(),
            "uint16_t" => "u16".to_owned(),
            "uint32_t" => "u32".to_owned(),
            "uint64_t" => "u64".to_owned(),
            "int32_t" => "i32".to_owned(),
            "size_t" => "usize".to_owned(),
            _ => {
                let type_name = self.bitmask_from_value.get(type_name).cloned().unwrap_or(type_name);
                if type_name.starts_with(TYPE_PREFIX) {
                    if self.is_non_null_type(type_name) && use_option {
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

    fn get_rust_default(&self, type_name: &str) -> String {
        match type_name {
            "char" => "c_char::default()".to_owned(),
            "int" => "c_int::default()".to_owned(),
            "float" => "f32::default()".to_owned(),
            "uint8_t" => "u8::default()".to_owned(),
            "uint16_t" => "u16::default()".to_owned(),
            "uint32_t" => "u32::default()".to_owned(),
            "uint64_t" => "u64::default()".to_owned(),
            "int32_t" => "i32::default()".to_owned(),
            "size_t" => "usize::default()".to_owned(),
            _ => {
                let type_name = self.bitmask_from_value.get(type_name).cloned().unwrap_or(type_name);
                if type_name.starts_with(TYPE_PREFIX) {
                    if self
                        .type_by_name
                        .get(type_name)
                        .and_then(|ref ty| ty.category.as_ref_str())
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
                        CExpr::Float(x) => writeln!(w, "f32 = {} as f32;", x)?,
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
            let decl = c_parse_typedef(code.code.as_str());
            writeln!(
                w,
                "pub type {} = {};",
                decl.name.skip_prefix(TYPE_PREFIX),
                self.get_rust_type_name(decl.ty.name, true, None)
            )?;
        } else {
            panic!("missing code for {:?}", ty);
        }
        Ok(())
    }

    fn is_enum_value_type_used(&self, type_name: &'a str) -> bool {
        // TODO: if we are not replaced by a bitmask, check usage in structs/functions (and skip if not used) to avoid unused FlagBits
        self.bitmask_from_value.get(type_name).is_none()
    }

    fn get_enum_entry_name(&self, type_name: &str, enum_type: EnumType, enum_name: &str) -> String {
        let uppercase_entry_name = enum_name.to_uppercase();
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
        if enum_type == EnumType::Bitmask && name_parts.last() == Some(&"BIT") {
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
                value: c_parse_int(value).expect("failed to parse enum value"),
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
                    value: if dir { value as i32 } else { -value as i32 },
                    comment: en.comment.clone(),
                }
            }
            vk::EnumSpec::Alias { ref alias, .. } => {
                EnumEntryValue::Alias(self.get_enum_entry_name(type_name, enum_type, alias.as_str()))
            }
            _ => panic!("unexpected enum spec"),
        }
    }

    fn write_enum_type(&self, w: &mut impl IoWrite, ty: &vk::Type, enum_type: EnumType) -> WriteResult {
        if let Some(ref comment) = ty.comment {
            writeln!(w, "/// {}", comment.as_str().trim_left_matches('/'))?;
        }
        let type_name = ty.get_type_name();
        if let Some(alias) = ty.alias.as_ref_str() {
            if enum_type == EnumType::Value && !self.is_enum_value_type_used(alias) {
                return Ok(());
            }
            writeln!(
                w,
                "pub type {} = {};",
                self.get_rust_type_name(type_name, true, None),
                self.get_rust_type_name(alias, true, None)
            )?;
        } else {
            let requires = ty.requires.as_ref_str();
            let value_type_name = match enum_type {
                EnumType::Bitmask => requires.unwrap_or(type_name),
                EnumType::Value => {
                    if self.is_enum_value_type_used(type_name) {
                        type_name
                    } else {
                        return Ok(());
                    }
                }
            };

            let entries: Vec<(String, EnumEntryValue, Option<&vk::Extension>)> = self
                .enums_by_name
                .get(value_type_name)
                .map(|s| s.as_slice())
                .unwrap_or(&[])
                .iter()
                .map(|en| {
                    (
                        self.get_enum_entry_name(value_type_name, enum_type, en.name.as_str()),
                        self.get_enum_entry_value(value_type_name, enum_type, en),
                        self.extension_by_enum_name.get(en.name.as_str()).cloned(),
                    )
                })
                .collect();

            let (derives, interior_type) = match enum_type {
                EnumType::Bitmask => ("Debug, Copy, Clone, PartialEq, Eq, Hash", "u32"),
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
                            "pub const {}: Self = {}({});",
                            name,
                            enum_name,
                            match enum_type {
                                EnumType::Bitmask => format!("{:#x}", value),
                                EnumType::Value => format!("{}", value),
                            },
                        )?;
                        all |= value;
                    }
                    EnumEntryValue::Alias(ref alias) => {
                        writeln!(w, "pub const {}: Self = Self::{};", name, alias)?;
                    }
                }
            }
            writeln!(w, "}}")?;
            writeln!(
                w,
                "impl default::Default for {} {{ fn default() -> Self {{ {0}(0) }} }}",
                enum_name
            )?;
            match enum_type {
                EnumType::Bitmask => {
                    writeln!(
                        w,
                        "impl {0} {{\
                         pub fn empty() -> Self {{ {0}(0) }}\
                         pub fn all() -> Self {{ {0}({1:#x}) }}\
                         pub fn is_empty(&self) -> bool {{ self.0 == 0 }}\
                         pub fn is_all(&self) -> bool {{ self.0 == {1:#x} }}\
                         pub fn intersects(&self, other: Self) -> bool {{ (self.0 & other.0) != 0 }}\
                         pub fn contains(&self, other: Self) -> bool {{ (self.0 & other.0) == other.0 }}\
                         }}",
                        enum_name, all
                    )?;
                    writeln!(
                        w,
                        "impl ops::BitOr for {} {{ type Output = Self;\
                         fn bitor(self, rhs: Self) -> Self {{ {0}(self.0 | rhs.0) }} }}",
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
                         fn bitand(self, rhs: Self) -> Self {{ {0}(self.0 & rhs.0) }} }}",
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
                         fn bitxor(self, rhs: Self) -> Self {{ {0}(self.0 ^ rhs.0) }} }}",
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
                    if entries.is_empty() {
                        writeln!(w, r#"f.write_str("0")"#)?;
                    } else {
                        writeln!(w, "display_bitmask(self.0, &[")?;
                        for (ref name, value, _) in &entries {
                            if let EnumEntryValue::Number { value, .. } = value {
                                writeln!(w, r#"({:#x}, "{}"),"#, value, name)?;
                            }
                        }
                        writeln!(w, "], f)")?;
                    }
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

    pub fn write_handle_type(&self, w: &mut impl IoWrite, ty: &'a vk::Type) -> WriteResult {
        if let Some(ref alias) = ty.alias {
            let type_name = ty.name.as_ref_str().expect("missing handle alias name");
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
                        "#[repr(transparent)] #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] pub struct {}(ptr::NonNull<c_void>);",
                        handle_name
                    )?;
                }
                Some("VK_DEFINE_NON_DISPATCHABLE_HANDLE") => {
                    writeln!(
                        w,
                        "#[repr(transparent)] #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)] pub struct {}(num::NonZeroU64);",
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
            match ty.decoration {
                CDecoration::None => "",
                CDecoration::Pointer => "* mut ",
                CDecoration::PointerToConst => "* const ",
                CDecoration::PointerToPointer => "* mut *mut ",
                CDecoration::PointerToConstPointerToConst => "*const *const ",
            },
            self.get_rust_type_name(&ty.name, ty.decoration == CDecoration::None, vk_prefix)
        )
        .unwrap();
        if let Some(mut array_size) = ty.array_size {
            if array_size.starts_with(CONST_PREFIX) {
                array_size = &array_size[3..];
            }
            write!(&mut s, "; {}]", array_size).unwrap();
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
                    self.get_rust_parameter_type(&decl.ty, None),
                )?;
            }
            writeln!(
                w,
                ") -> {};",
                self.get_rust_parameter_type(&function_decl.proto.ty, None)
            )?;
        } else {
            panic!("missing function pointer code for {:?}", ty);
        }
        Ok(())
    }

    fn rewrite_variable_decl(&self, context: &str, mut decl: CVariableDecl<'a>) -> CVariableDecl<'a> {
        match decl.name {
            "apiVersion" | "pApiVersion" => {
                decl.ty.name = "VkVersion";
            }
            "specVersion" if context == "LayerProperties" => {
                decl.ty.name = "VkVersion";
            }
            _ => {}
        }
        decl
    }

    fn write_aggregrate_type(&self, w: &mut impl IoWrite, ty: &vk::Type, agg_type: AggregateType) -> WriteResult {
        if let Some(ref comment) = ty.comment {
            writeln!(w, "/// {}", comment.as_str().trim_left_matches('/'))?;
        }
        let type_name = ty.name.as_ref_str().expect("missing struct name");
        if let Some(ref alias) = ty.alias {
            writeln!(
                w,
                "pub type {} = {};",
                type_name.skip_prefix(TYPE_PREFIX),
                alias.skip_prefix(TYPE_PREFIX)
            )?;
        } else if let vk::TypeSpec::Members(ref members) = ty.spec {
            let agg_name = type_name.skip_prefix(TYPE_PREFIX);
            writeln!(
                w,
                "#[repr(C)] #[derive({})] pub {} {} {{",
                match agg_type {
                    AggregateType::Struct => match type_name {
                        "VkOffset2D" | "VkOffset3D" | "VkExtent2D" | "VkExtent3D" | "VkRect2D" => {
                            "Copy, Clone, PartialEq, Eq, Hash"
                        }
                        _ => "Copy, Clone",
                    },
                    AggregateType::Union => "Copy, Clone",
                },
                match agg_type {
                    AggregateType::Struct => "struct",
                    AggregateType::Union => "union",
                },
                agg_name
            )?;
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
            for (member_def, decl) in member_defs.iter().zip(decls.iter()) {
                for comment in member_def.markup.iter().filter_map(|markup| match markup {
                    vk::TypeMemberMarkup::Comment(ref comment) => Some(comment),
                    _ => None,
                }) {
                    writeln!(w, "/// {}", comment)?;
                }
                writeln!(
                    w,
                    "pub {}: {},",
                    get_rust_variable_name(&decl.name),
                    self.get_rust_parameter_type(&decl.ty, None)
                )?;
            }
            writeln!(w, "}}")?;
            writeln!(w, "impl default::Default for {} {{ fn default() -> Self {{", agg_name)?;
            match agg_type {
                AggregateType::Struct => {
                    write!(w, "{} {{", agg_name)?;
                    for (member_def, decl) in member_defs.iter().zip(decls.iter()) {
                        write!(w, "{}: ", get_rust_variable_name(&decl.name))?;
                        if let Some(values) = member_def.values.as_ref_str() {
                            // assume enum value for now
                            let name = self.get_enum_entry_name(&decl.ty.name, EnumType::Value, values);
                            writeln!(w, "{}::{},", self.get_rust_type_name(decl.ty.name, true, None), name)?;
                        } else {
                            // get element default
                            let element_value = match decl.ty.decoration {
                                CDecoration::Pointer | CDecoration::PointerToPointer => "ptr::null_mut()".to_owned(),
                                CDecoration::PointerToConst | CDecoration::PointerToConstPointerToConst => {
                                    "ptr::null()".to_owned()
                                }
                                CDecoration::None => self.get_rust_default(decl.ty.name).to_owned(),
                            };

                            // write single or array
                            if let Some(mut array_size) = decl.ty.array_size {
                                if array_size.starts_with(CONST_PREFIX) {
                                    array_size = &array_size[3..];
                                }
                                writeln!(w, "[{}; {}],", element_value, array_size)?;
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
                for decl in &decls {
                    let var_name = get_rust_variable_name(&decl.name);
                    let category = self
                        .type_by_name
                        .get(decl.ty.name)
                        .and_then(|ty| ty.category.as_ref_str());
                    if decl.ty.name == "char" && decl.ty.decoration == CDecoration::None && decl.ty.array_size.is_some()
                    {
                        writeln!(
                            w,
                            r#".field("{0}", &unsafe {{ CStr::from_ptr(self.{0}.as_ptr()) }})"#,
                            var_name
                        )?;
                    } else if category == Some("funcpointer")
                        && decl.ty.decoration == CDecoration::None
                        && decl.ty.array_size.is_none()
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
            let category = ty.category.as_ref_str();
            match category {
                Some("basetype") => {
                    self.write_base_type(w, ty)?;
                }
                Some("bitmask") => {
                    self.write_enum_type(w, ty, EnumType::Bitmask)?;
                }
                Some("enum") => {
                    self.write_enum_type(w, ty, EnumType::Value)?;
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

    fn write_blocks(&self, w: &mut impl IoWrite) -> WriteResult {
        let mut cmd_names = HashSet::new();
        for group_names in &self.group_names {
            for version_names in &group_names.versions {
                let version = version_names.version.skip_prefix(VERSION_PREFIX);
                let struct_name = format!("{}Fn{}", group_names.group, version);
                let decls: Vec<CFunctionDecl> = version_names
                    .names
                    .iter()
                    .map(|name| {
                        let cmd_def = self.cmd_def_by_name.get(name).expect("missing cmd def");
                        let mut decl = c_parse_function_decl(cmd_def.code.as_str());
                        let context = decl.proto.name;
                        for param in decl.parameters.iter_mut() {
                            take_mut::take(param, |v| self.rewrite_variable_decl(context, v));
                        }
                        decl
                    })
                    .collect();
                for function_decl in &decls {
                    let name_part = function_decl.proto.name.skip_prefix(FN_PREFIX);
                    if cmd_names.insert(name_part) {
                        write!(w, r#"type Fn{} = unsafe extern "system" fn("#, name_part)?;
                        for param in &function_decl.parameters {
                            write!(
                                w,
                                "{}: {},",
                                get_rust_variable_name(param.name.to_snake_case().as_str()),
                                self.get_rust_parameter_type(&param.ty, None),
                            )?;
                        }
                        writeln!(
                            w,
                            ") -> {};",
                            self.get_rust_parameter_type(&function_decl.proto.ty, None)
                        )?;
                    }
                }
                writeln!(w, "pub struct {} {{", struct_name)?;
                for function_decl in &decls {
                    let name_part = function_decl.proto.name.skip_prefix(FN_PREFIX);
                    writeln!(w, "pub {}: Fn{},", name_part.to_snake_case(), name_part)?;
                }
                writeln!(w, "}}")?;
                writeln!(w, "impl {} {{", struct_name)?;
                writeln!(
                    w,
                    "pub fn load<F>(mut f: F) -> (Self, bool) where F: FnMut(&CStr) -> Option<FnVoidFunction> {{\
                     let mut all_loaded = true;\
                     let block = {} {{",
                    struct_name
                )?;
                for function_decl in &decls {
                    let fn_name = function_decl.proto.name.skip_prefix(FN_PREFIX).to_snake_case();
                    write!(w, "{}: unsafe {{", fn_name)?;
                    write!(w, r#"extern "system" fn {}_fallback("#, fn_name)?;
                    for param in &function_decl.parameters {
                        write!(w, "_: {},", self.get_rust_parameter_type(&param.ty, None),)?;
                    }
                    writeln!(
                        w,
                        r#") -> {} {{ panic!("fn {} not loaded"); }}"#,
                        self.get_rust_parameter_type(&function_decl.proto.ty, None),
                        fn_name,
                    )?;
                    writeln!(
                        w,
                        r#"let name = CStr::from_bytes_with_nul_unchecked(b"{}\0");"#,
                        function_decl.proto.name
                    )?;
                    writeln!(w, "f(name).map_or_else(|| {{ all_loaded = false; mem::transmute({}_fallback as *const c_void) }}, |f| mem::transmute(f)) }},", fn_name)?;
                }
                writeln!(w, "}}; (block, all_loaded) }}")?;
                writeln!(w, "}}")?;
            }
        }
        Ok(())
    }

    pub fn write_vk(&self, path: &Path) -> WriteResult {
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);

        write!(&mut w, "{}", include_str!("vk_prefix.rs"))?;
        self.write_constants(&mut w)?;
        self.write_types(&mut w)?;
        self.write_blocks(&mut w)?;
        Ok(())
    }

    fn write_builder_structs(&self, w: &mut impl IoWrite) -> WriteResult {
        for ty in self
            .get_type_iterator()
            .filter(|ty| ty.category.as_ref_str() == Some("struct") && ty.alias.is_none() && ty.returnedonly.is_none())
        {
            if let vk::TypeSpec::Members(ref members) = ty.spec {
                let type_name = ty.name.as_ref_str().expect("missing struct name");
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
                        cparam.ty.name,
                        cparam.ty.decoration == CDecoration::None,
                        Some("vk::"),
                    );

                    // match bool
                    if cparam.ty.name == "VkBool32" && cparam.ty.decoration == CDecoration::None {
                        params[i].ty = LibParamType::Bool;
                        continue;
                    }

                    // match CStr
                    if cparam.ty.name == "char"
                        && cparam.ty.decoration == CDecoration::PointerToConst
                        && vparam.len.as_ref_str() == Some("null-terminated")
                    {
                        params[i].ty = LibParamType::CStr;
                        continue;
                    }

                    // remove Option if not optional
                    if cparam.ty.decoration == CDecoration::None
                        && vparam.optional.is_none()
                        && self.is_non_null_type(cparam.ty.name)
                    {
                        params[i].ty = LibParamType::NonOptional {
                            inner_type_name: self.get_rust_type_name(cparam.ty.name, false, Some("vk::")),
                        };
                        continue;
                    }

                    // match slice
                    if let Some(len_name) = vparam.len.as_ref_str().and_then(|s| s.split(',').next()) {
                        let is_slice_type = (cparam.ty.name != "void"
                            && cparam.ty.decoration == CDecoration::PointerToConst)
                            || cparam.ty.decoration == CDecoration::PointerToConstPointerToConst;
                        if is_slice_type && !len_name.starts_with("latexmath:") {
                            let inner_type_name = if cparam.ty.decoration == CDecoration::PointerToConstPointerToConst {
                                format!("*const {}", inner_type_name)
                            } else {
                                inner_type_name
                            };
                            let is_optional = vparam.optional.as_ref_str() == Some("true");
                            let is_single = vparam.noautovalidity.as_ref_str() == Some("true");
                            let len_index = decls
                                .iter()
                                .position(|decl| decl.name == len_name)
                                .expect("missing len variable");
                            let len_cparam = &decls[len_index];
                            let slice_info = SliceInfo {
                                name: params[i].name.clone(),
                                type_name: inner_type_name.clone(),
                                is_optional,
                            };
                            params[i].ty = LibParamType::Slice {
                                inner_type_name,
                                is_optional,
                            };
                            take_mut::take(&mut params[len_index].ty, |ty| match ty {
                                LibParamType::SharedSliceLen { name, mut slice_infos } => {
                                    if is_single {
                                        panic!("unsupported mix of slices")
                                    } else {
                                        slice_infos.push(slice_info);
                                        LibParamType::SharedSliceLen { name, slice_infos }
                                    }
                                }
                                LibParamType::SingleSliceLen { mut slice_infos } => {
                                    if is_single {
                                        slice_infos.push(slice_info);
                                        LibParamType::SingleSliceLen { slice_infos }
                                    } else {
                                        panic!("unsupported mix of slices")
                                    }
                                }
                                LibParamType::CDecl => {
                                    if is_single {
                                        LibParamType::SingleSliceLen {
                                            slice_infos: vec![slice_info; 1],
                                        }
                                    } else {
                                        LibParamType::SharedSliceLen {
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

                    // match reference
                    if cparam.ty.name != "void"
                        && cparam.ty.decoration == CDecoration::PointerToConst
                        && vparam.len.is_none()
                    {
                        let is_optional = vparam.optional.as_ref_str() == Some("true");
                        params[i].ty = LibParamType::Ref {
                            inner_type_name,
                            is_optional,
                        };
                        continue;
                    }
                }
                if let Some(phantom_decl) = decls.iter().find(|decl| decl.ty.decoration != CDecoration::None) {
                    // implement trait on vk type
                    writeln!(
                        w,
                        "impl<'a> Builder<'a> for vk::{0} {{\
                         type Type = {0}Builder<'a>;\
                         fn builder() -> Self::Type {{ {0}Builder::new() }} }}",
                        agg_name
                    )?;

                    // declare builder in lib
                    let phantom_type_name = self.get_rust_type_name(
                        phantom_decl.ty.name,
                        phantom_decl.ty.decoration != CDecoration::None,
                        Some("vk::"),
                    );
                    writeln!(
                        w,
                        "pub struct {0}Builder<'a> {{\
                         inner: vk::{0}, phantom: PhantomData<&'a {1}> }}",
                        agg_name, phantom_type_name,
                    )?;

                    // setters
                    writeln!(w, "impl<'a> {}Builder<'a> {{", agg_name)?;
                    writeln!(
                        w,
                        "pub fn new() -> Self {{\
                         Self {{ inner: Default::default(), phantom: PhantomData, }} }}"
                    )?;
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
                            LibParamType::CStr => {
                                writeln!(
                                    w,
                                    "pub fn {0}(mut self, {0}: &'a CStr) -> Self {{\
                                     self.inner.{0} = {0}.as_ptr(); self }}",
                                    rparam.name
                                )?;
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
                            LibParamType::SharedSliceLen { ref slice_infos, .. } => {
                                write!(w, "pub fn {0}(mut self ", slice_infos[0].name)?;
                                let has_multiple_slices = slice_infos.len() > 1;
                                for slice_info in slice_infos {
                                    if slice_info.is_optional && has_multiple_slices {
                                        write!(w, ", {}: Option<&'a [{}]>", slice_info.name, slice_info.type_name)?;
                                    } else {
                                        write!(w, ", {}: &'a [{}]", slice_info.name, slice_info.type_name)?;
                                    }
                                }
                                writeln!(w, ") -> Self {{")?;
                                writeln!(
                                    w,
                                    "self.inner.{0} = {1}.len() as u32;",
                                    rparam.name, slice_infos[0].name
                                )?;
                                for slice_info in slice_infos.iter().skip(1) {
                                    if slice_info.is_optional && has_multiple_slices {
                                        writeln!(
                                            w,
                                            "if let Some(s) = {} {{ assert_eq!(self.inner.{}, s.len() as u32); }}",
                                            slice_info.name, rparam.name
                                        )?;
                                    } else {
                                        writeln!(
                                            w,
                                            "assert_eq!(self.inner.{}, {}.len() as u32);",
                                            rparam.name, slice_info.name
                                        )?;
                                    }
                                }
                                for slice_info in slice_infos {
                                    if slice_info.is_optional && has_multiple_slices {
                                        writeln!(
                                            w,
                                            "self.inner.{0} = {0}.map_or(ptr::null(), |s| s.as_ptr());",
                                            slice_info.name
                                        )?;
                                    } else {
                                        writeln!(w, "self.inner.{0} = {0}.as_ptr();", slice_info.name)?;
                                    }
                                }
                                writeln!(w, "self }}",)?;
                            }
                            LibParamType::SingleSliceLen { ref slice_infos } => {
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
                                    write!(
                                        w,
                                        "pub fn {0}(mut self, {0}: &'a [{1}]) -> Self {{",
                                        slice_info.name, slice_info.type_name
                                    )?;
                                    write!(w, "self.inner.{} = {}.len() as u32;", rparam.name, slice_info.name)?;
                                    writeln!(w, "self.inner.{0} = {0}.as_ptr(); self }}", slice_info.name)?;
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
                    writeln!(w, "}}")?;

                    // allow deref to vk type
                    writeln!(
                        w,
                        "impl<'a> Deref for {0}Builder<'a> {{\
                         type Target = vk::{0};\
                         fn deref(&self) -> &Self::Target {{ &self.inner }} }}",
                        agg_name
                    )?;
                }
            } else {
                panic!("missing type members for {:?}", ty);
            }
        }
        Ok(())
    }

    pub fn write_builder(&self, path: &Path) -> WriteResult {
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);

        write!(&mut w, "{}", include_str!("builder_prefix.rs"))?;
        self.write_builder_structs(&mut w)?;

        Ok(())
    }

    fn wrap_group_command_arguments(
        &self,
        group: Group,
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
                self.get_rust_type_name(cparam.ty.name, cparam.ty.decoration == CDecoration::None, Some("vk::"));

            // match member handle (first parameter only)
            if i == 0 {
                if let Some(type_name) = match group {
                    Group::Instance | Group::InstanceExtension(_) => Some("VkInstance"),
                    Group::Device | Group::DeviceExtension(_) => Some("VkDevice"),
                    _ => None,
                } {
                    if cparam.ty.name == type_name && cparam.ty.decoration == CDecoration::None {
                        params[i].ty = LibParamType::MemberHandle;
                        continue;
                    }
                }
            }

            // match bool
            if cparam.ty.name == "VkBool32" && cparam.ty.decoration == CDecoration::None {
                params[i].ty = LibParamType::Bool;
                continue;
            }

            // match CStr
            if cparam.ty.name == "char"
                && cparam.ty.decoration == CDecoration::PointerToConst
                && vparam.len.as_ref_str() == Some("null-terminated")
            {
                params[i].ty = LibParamType::CStr;
                continue;
            }

            // remove Option if not optional
            if cparam.ty.decoration == CDecoration::None
                && vparam.optional.is_none()
                && self.is_non_null_type(cparam.ty.name)
            {
                params[i].ty = LibParamType::NonOptional {
                    inner_type_name: self.get_rust_type_name(cparam.ty.name, false, Some("vk::")),
                };
                continue;
            }

            // match slice (parameter or return)
            if let Some(len_name) = vparam.len.as_ref_str() {
                if cparam.ty.name != "void" && cparam.ty.decoration == CDecoration::PointerToConst {
                    let len_index = decl
                        .parameters
                        .iter()
                        .position(|cparam| cparam.name == len_name)
                        .expect("missing len variable");
                    let len_cparam = &decl.parameters[len_index];
                    let is_optional = vparam.optional.as_ref_str() == Some("true");
                    let slice_info = SliceInfo {
                        name: params[i].name.clone(),
                        type_name: inner_type_name.clone(),
                        is_optional,
                    };
                    params[i].ty = LibParamType::Slice {
                        inner_type_name,
                        is_optional,
                    };
                    take_mut::take(&mut params[len_index].ty, |ty| match ty {
                        LibParamType::SharedSliceLen { name, mut slice_infos } => {
                            slice_infos.push(slice_info);
                            LibParamType::SharedSliceLen { name, slice_infos }
                        }
                        LibParamType::CDecl => LibParamType::SharedSliceLen {
                            name: len_cparam.name.to_snake_case(),
                            slice_infos: vec![slice_info; 1],
                        },
                        _ => {
                            panic!("purpose already found for {:?}", len_cparam);
                        }
                    });
                    continue;
                }
                if cparam.ty.name != "void"
                    && cparam.ty.decoration == CDecoration::Pointer
                    && vparam.optional.as_ref_str() == Some("true")
                    && (cmd_return_value == CommandReturnValue::Void
                        || cmd_def.successcodes.as_ref_str() == Some("VK_SUCCESS,VK_INCOMPLETE"))
                {
                    let len_index = decl
                        .parameters
                        .iter()
                        .position(|cparam| cparam.name == len_name)
                        .expect("missing len variable");
                    let len_cparam = &decl.parameters[len_index];
                    let len_vparam = &cmd_def.params[len_index];
                    if len_cparam.ty.decoration == CDecoration::Pointer
                        && len_vparam.optional.as_ref_str() == Some("false,true")
                    {
                        params[i].ty = LibParamType::ReturnVec {
                            inner_type_name: inner_type_name.clone(),
                        };
                        let slice_name = params[i].name.clone();
                        params[len_index].ty = match params[len_index].ty {
                            LibParamType::CDecl => LibParamType::ReturnVecLen { slice_name },
                            _ => panic!("purpose already found for {:?}", len_cparam),
                        };
                        take_mut::take(&mut return_type, |ty| match ty {
                            LibReturnType::CDecl => match cmd_return_value {
                                CommandReturnValue::Result => LibReturnType::ResultVecUnknownLen,
                                CommandReturnValue::Void => LibReturnType::VecUnknownLen,
                                CommandReturnValue::Other => panic!("cannot handle return type {:?}", cmd_def.proto),
                            },
                            _ => panic!("already have return type of {:?}", ty),
                        });
                        return_type_name = inner_type_name;
                        continue;
                    }
                }
                if cparam.ty.name != "void"
                    && cparam.ty.decoration == CDecoration::Pointer
                    && vparam.optional.is_none()
                    && vparam.len.is_some()
                {
                    let len_names: Vec<&str> = len_name.split("::").collect();
                    let len_expr = if len_names.len() == 1 {
                        let len_index = decl
                            .parameters
                            .iter()
                            .position(|cparam| cparam.name == len_names.first().cloned().unwrap())
                            .expect("missing len variable");
                        let len_cparam = &decl.parameters[len_index];
                        let len_expr = len_cparam.name.to_snake_case();
                        take_mut::take(&mut params[len_index].ty, |ty| match ty {
                            LibParamType::SharedSliceLen { name, slice_infos } => {
                                LibParamType::SharedSliceLen { name, slice_infos }
                            }
                            LibParamType::CDecl => LibParamType::SharedSliceLen {
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
                && (vparam.optional.is_none() || vparam.optional.as_ref_str() == Some("false,true"))
                && vparam.len.is_none()
                && cmd_return_value != CommandReturnValue::Other
                && return_type == LibReturnType::CDecl
            {
                let has_member_values = self
                    .type_by_name
                    .get(cparam.ty.name)
                    .and_then(|vtype| {
                        if let Some(alias) = vtype.alias.as_ref_str() {
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
                        match cparam.ty.name {
                            "VkBool32" => {
                                inner_type_name = "bool".to_owned();
                                return_transform = LibReturnTransform::ToBool;
                            }
                            "VkInstance" => {
                                inner_type_name = "Instance".to_owned();
                                return_transform = LibReturnTransform::ToInstance;
                            }
                            "VkDevice" => {
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
                            CommandReturnValue::Result => match cmd_def.successcodes.as_ref_str() {
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
            if cparam.ty.name != "void" && vparam.len.is_none() && !self.is_non_null_type(cparam.ty.name) {
                if cparam.ty.decoration == CDecoration::PointerToConst {
                    let is_optional = vparam.optional.as_ref_str() == Some("true");
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
            return_type = if cmd_def.successcodes.as_ref_str() == Some("VK_SUCCESS") {
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

    fn write_group_command(&self, w: &mut impl IoWrite, group: Group, version: &str, cmd_name: &str) -> WriteResult {
        let cmd_def = self.cmd_def_by_name.get(cmd_name).expect("missing cmd def");
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
        let cmd_return_value = match cmd_def.proto.type_name.as_ref_str() {
            Some("VkResult") => CommandReturnValue::Result,
            Some("void") => CommandReturnValue::Void,
            _ => CommandReturnValue::Other,
        };

        let (return_type, return_transform, return_type_name) =
            self.wrap_group_command_arguments(group, &cmd_def, cmd_return_value, &decl, &mut params);

        let fn_name = decl.proto.name.skip_prefix(FN_PREFIX).to_snake_case();

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

        for style in styles {
            write!(w, "pub unsafe fn {}", fn_name)?;
            match style {
                LibCommandStyle::Default => {}
                LibCommandStyle::ToVecUnknownLen | LibCommandStyle::ToVecKnownLen => {
                    write!(w, "_to_vec")?;
                }
                LibCommandStyle::Array => {
                    write!(w, "_array<A: Array<Item = {}>>", return_type_name)?;
                }
                LibCommandStyle::Single => {
                    write!(w, "_single")?;
                }
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
                    LibParamType::CStr => {
                        write!(w, "{}: &CStr,", rparam.name)?;
                    }
                    LibParamType::NonOptional { ref inner_type_name } => {
                        write!(w, "{}: {},", rparam.name, inner_type_name)?;
                    }
                    LibParamType::SharedSliceLen { ref slice_infos, .. } => {
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
                    LibParamType::SingleSliceLen { .. } => {}
                    LibParamType::Slice {
                        ref inner_type_name,
                        is_optional,
                    } => {
                        if is_optional {
                            write!(w, "{}: Option<&[{}]>,", rparam.name, inner_type_name,)?;
                        } else {
                            write!(w, "{}: &[{}],", rparam.name, inner_type_name,)?;
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
                    LibParamType::ReturnVecLen { .. } => {}
                    LibParamType::ReturnVec { ref inner_type_name } => {
                        if *style == LibCommandStyle::Default {
                            write!(w, "{}: *mut {},", rparam.name, inner_type_name)?;
                        }
                    }
                }
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
                LibReturnType::ResultEmpty | LibReturnType::ResultObject | LibReturnType::ResultEnum => {
                    match return_transform {
                        LibReturnTransform::ToInstance | LibReturnTransform::ToDevice => {
                            write!(w, "-> result::Result<{}, LoaderError>", return_type_name)?;
                        }
                        _ => {
                            write!(w, "-> Result<{}>", return_type_name)?;
                        }
                    }
                }
                LibReturnType::ResultVecUnknownLen | LibReturnType::ResultVecKnownLen { .. } => match style {
                    LibCommandStyle::Default => {
                        write!(w, "-> Result<()>")?;
                    }
                    LibCommandStyle::ToVecKnownLen | LibCommandStyle::ToVecUnknownLen => {
                        write!(w, "-> Result<Vec<{}>>", return_type_name)?;
                    }
                    LibCommandStyle::Array => {
                        write!(w, "-> Result<A>")?;
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

            for rparam in &params {
                if let LibParamType::SharedSliceLen {
                    ref name,
                    ref slice_infos,
                } = rparam.ty
                {
                    let first_non_optional = slice_infos.iter().find(|slice_info| !slice_info.is_optional);
                    if let Some(first_non_optional) = first_non_optional {
                        writeln!(w, "let {} = {}.len() as u32;", name, first_non_optional.name)?;
                    }
                    for slice_info in slice_infos {
                        if let Some(first_non_optional) = first_non_optional {
                            if slice_info.name == first_non_optional.name {
                                continue;
                            }
                        }
                        if slice_info.is_optional {
                            writeln!(
                                w,
                                "if let Some(s) = {} {{ assert_eq!({}, s.len() as u32); }}",
                                slice_info.name, name
                            )?;
                        } else {
                            writeln!(w, "assert_eq!({}, {}.len() as u32);", name, slice_info.name)?;
                        }
                    }
                }
            }

            let pass_start = match style {
                LibCommandStyle::ToVecUnknownLen => 0,
                _ => 1,
            };

            for pass_index in pass_start..2 {
                match return_type {
                    LibReturnType::CDecl => {
                        write!(w, "let res = ")?;
                    }
                    LibReturnType::None => {}
                    LibReturnType::ResultEmpty | LibReturnType::ResultEnum => {
                        write!(w, "let err = ")?;
                    }
                    LibReturnType::ResultObject | LibReturnType::ResultEnumAndObject => {
                        write!(w, "let mut res = mem::uninitialized(); let err = ")?;
                    }
                    LibReturnType::Object => {
                        write!(w, "let mut res = mem::uninitialized();")?;
                    }
                    LibReturnType::ResultVecUnknownLen => {
                        if pass_index == 0 {
                            write!(w, "let mut len = mem::uninitialized(); let len_err = ")?;
                        } else {
                            write!(w, "let mut v = Vec::with_capacity(len as usize); let v_err = ")?;
                        }
                    }
                    LibReturnType::VecUnknownLen => {
                        if pass_index == 0 {
                            write!(w, "let mut len = mem::uninitialized();")?;
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
                                "assert_eq!({}, A::len() as u32); let mut v: A = mem::uninitialized(); let v_err = ",
                                len_expr
                            )?;
                        }
                        LibCommandStyle::Single => {
                            write!(
                                w,
                                "assert_eq!({}, 1); let mut v = mem::uninitialized(); let v_err = ",
                                len_expr
                            )?;
                        }
                    },
                }

                write!(w, r#"(self.fp{}.{})("#, version, fn_name)?;
                for rparam in &params {
                    match rparam.ty {
                        LibParamType::CDecl | LibParamType::MutRef { .. } => {
                            write!(w, "{}", rparam.name)?;
                        }
                        LibParamType::MemberHandle => {
                            write!(w, "Some(self.handle)")?;
                        }
                        LibParamType::Bool => {
                            write!(w, "if {} {{ vk::TRUE }} else {{ vk::FALSE }}", rparam.name)?;
                        }
                        LibParamType::CStr => {
                            write!(w, "{}.as_ptr()", rparam.name)?;
                        }
                        LibParamType::NonOptional { .. } => {
                            write!(w, "Some({})", rparam.name)?;
                        }
                        LibParamType::SharedSliceLen { ref name, .. } => {
                            write!(w, "{}", name)?;
                        }
                        LibParamType::SingleSliceLen { ref slice_infos } => {
                            write!(w, "{}.len() as u32", slice_infos.first().unwrap().name)?;
                        }
                        LibParamType::Slice { is_optional, .. } => {
                            if is_optional {
                                write!(w, "{}.map_or(ptr::null(), |r| r.as_ptr())", rparam.name)?;
                            } else {
                                write!(w, "{}.as_ptr()", rparam.name)?;
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
                            write!(w, "&mut res")?;
                        }
                        LibParamType::ReturnVecLen { .. } => {
                            write!(w, "&mut len")?;
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
                                write!(w, "v.as_mut_ptr()")?;
                            }
                            LibCommandStyle::Single => {
                                write!(w, "&mut v")?;
                            }
                        },
                    }
                    write!(w, ",")?;
                }
                writeln!(w, ");")?;

                match return_type {
                    LibReturnType::CDecl => {}
                    LibReturnType::None => {}
                    LibReturnType::ResultEmpty => {
                        write!(
                            w,
                            "let res = match err {{ vk::Result::SUCCESS => Ok(()), _ => Err(err) }};"
                        )?;
                    }
                    LibReturnType::ResultEnum => {
                        let ok_matches = if let Some(successcodes) = cmd_def.successcodes.as_ref_str() {
                            let matches: Vec<String> = successcodes
                                .split(',')
                                .map(|s| format!("vk::Result::{}", s.skip_prefix("VK_")))
                                .collect();
                            matches.join("|")
                        } else {
                            "vk::Result::SUCCESS".to_owned()
                        };
                        write!(w, "let res = match err {{ {} => Ok(err), _ => Err(err) }};", ok_matches)?;
                    }
                    LibReturnType::ResultObject => {
                        write!(
                            w,
                            "let res = match err {{ vk::Result::SUCCESS => Ok(res), _ => Err(err) }};",
                        )?;
                    }
                    LibReturnType::ResultEnumAndObject => {
                        let matches: Vec<String> = cmd_def
                            .successcodes
                            .as_ref_str()
                            .unwrap()
                            .split(',')
                            .map(|s| format!("vk::Result::{}", s.skip_prefix("VK_")))
                            .collect();
                        write!(
                            w,
                            "let res = match err {{ {} => Ok((err, res)), _ => Err(err) }};",
                            matches.join("|"),
                        )?;
                    }
                    LibReturnType::Object => {}
                    LibReturnType::ResultVecUnknownLen => {
                        if pass_index == 0 {
                            write!(w, "if len_err != vk::Result::SUCCESS {{ return Err(len_err) }}")?;
                        } else {
                            write!(w, "v.set_len(len as usize); let res = match v_err {{ vk::Result::SUCCESS => Ok(v), _ => Err(v_err) }};")?;
                        }
                    }
                    LibReturnType::VecUnknownLen => {
                        if pass_index != 0 {
                            write!(w, "v.set_len(len as usize); let res = v;")?;
                        }
                    }
                    LibReturnType::ResultVecKnownLen { .. } => match style {
                        LibCommandStyle::Default => {
                            write!(
                                w,
                                "let res = match v_err {{ vk::Result::SUCCESS => Ok(()), _ => Err(v_err) }};"
                            )?;
                        }
                        LibCommandStyle::ToVecUnknownLen
                        | LibCommandStyle::ToVecKnownLen
                        | LibCommandStyle::Array
                        | LibCommandStyle::Single => {
                            write!(
                                w,
                                "let res = match v_err {{ vk::Result::SUCCESS => Ok(v), _ => Err(v_err) }};"
                            )?;
                        }
                    },
                }

                if return_type != LibReturnType::None && pass_index != 0 {
                    writeln!(
                        w,
                        "{}",
                        match return_transform {
                            LibReturnTransform::None => "res",
                            LibReturnTransform::ToBool => "res.map(|r| r != vk::FALSE)",
                            LibReturnTransform::ToInstance => {
                                "res.map_err(|e| LoaderError::Vulkan(e)).and_then(|r| Instance::load(r))"
                            }
                            LibReturnTransform::ToDevice => {
                                "res.map_err(|e| LoaderError::Vulkan(e)).and_then(|r| Device::load(&self, r))"
                            }
                        }
                    )?;
                }
            }

            writeln!(w, " }}")?;
        }

        Ok(())
    }

    fn write_group_structs(&self, w: &mut impl IoWrite) -> WriteResult {
        for group_names in self.group_names.iter().filter(|g| !g.versions.is_empty()) {
            match group_names.group {
                Group::Loader => {
                    writeln!(w, "/// Core library loader")?;
                }
                Group::Instance => {
                    writeln!(w, "/// Core instance loader")?;
                }
                Group::InstanceExtension(ref name) => {
                    writeln!(w, "/// Loader for the `{}` instance extension", name)?;
                }
                Group::Device => {
                    writeln!(w, "/// Core device loader")?;
                }
                Group::DeviceExtension(ref name) => {
                    writeln!(w, "/// Loader for the `{}` device extension", name)?;
                }
            }
            write!(w, "pub struct {} {{", group_names.group)?;
            write!(w, "pub version: vk::Version,")?;
            match group_names.group {
                Group::Loader => {}
                Group::Instance | Group::InstanceExtension(_) => {
                    write!(w, "pub handle: vk::Instance,")?;
                }
                Group::Device | Group::DeviceExtension(_) => {
                    write!(w, "pub handle: vk::Device,")?;
                }
            }
            for version_names in &group_names.versions {
                let version = version_names.version.skip_prefix(VERSION_PREFIX);
                write!(w, "pub fp{0}: vk::{1}Fn{0},", version, group_names.group)?;
            }
            writeln!(w, "}}")?;
            writeln!(w, "impl {} {{", group_names.group)?;
            match group_names.group {
                Group::Loader => {
                    write!(
                        w,
                        "pub fn new() -> result::Result<Self, LoaderError> {{\
                         let lib = LIB.as_ref().map_err(|e| (*e).clone())?;\
                         let f = |name: &CStr| unsafe {{\
                         lib.get_instance_proc_addr(None, name).map(|p| mem::transmute(p)) }};"
                    )?;
                }
                Group::Instance => {
                    writeln!(
                        w,
                        "unsafe fn load(instance: vk::Instance) -> result::Result<Self, LoaderError> {{\
                         let lib = LIB.as_ref().map_err(|e| (*e).clone())?;\
                         let f = |name: &CStr| lib.get_instance_proc_addr(Some(instance), name).map(|p| mem::transmute(p));"
                    )?;
                }
                Group::Device => {
                    writeln!(
                        w,
                        "unsafe fn load(instance: &Instance, device: vk::Device) -> result::Result<Self, LoaderError> {{\
                         let f = |name: &CStr| instance.get_device_proc_addr(device, name).map(|p| mem::transmute(p));"
                    )?;
                }
                Group::InstanceExtension(_) => {
                    writeln!(
                        w,
                        "pub unsafe fn new(instance: &Instance) -> result::Result<Self, LoaderError> {{\
                         let lib = LIB.as_ref().map_err(|e| (*e).clone())?;\
                         let f = |name: &CStr| lib.get_instance_proc_addr(Some(instance.handle), name).map(|p| mem::transmute(p));"
                    )?;
                }
                Group::DeviceExtension(_) => {
                    writeln!(
                        w,
                        "pub unsafe fn new(instance: &Instance, device: &Device) -> result::Result<Self, LoaderError> {{\
                         let f = |name: &CStr| instance.get_device_proc_addr(device.handle, name).map(|p| mem::transmute(p));"
                    )?;
                }
            }
            writeln!(w, "let mut version = vk::Version::from_raw(0); let mut ok = true;")?;
            for version_names in &group_names.versions {
                let version = version_names.version.skip_prefix(VERSION_PREFIX);
                let version_parts: Vec<&str> = version.split('_').collect();
                writeln!(
                    w,
                    "let (fp{0}, ok{0}) = vk::{1}Fn{0}::load(f);\
                     ok = ok && ok{0};\
                     if ok {{ version = vk::Version::from_raw_parts({2}, {3}, 0); }}",
                    version, group_names.group, version_parts[0], version_parts[1]
                )?;
            }
            writeln!(w, "Ok(Self {{ version,")?;
            match group_names.group {
                Group::Loader => {}
                Group::Instance => {
                    write!(w, "handle: instance,")?;
                }
                Group::Device => {
                    write!(w, "handle: device,")?;
                }
                Group::InstanceExtension(_) => {
                    write!(w, "handle: instance.handle,")?;
                }
                Group::DeviceExtension(_) => {
                    write!(w, "handle: device.handle,")?;
                }
            }
            for version_names in &group_names.versions {
                let version = version_names.version.skip_prefix(VERSION_PREFIX);
                write!(w, "fp{},", version)?;
            }
            writeln!(w, "}}) }}")?;
            match group_names.group {
                Group::InstanceExtension(ref name) | Group::DeviceExtension(ref name) => {
                    write!(
                        w,
                        r#"pub fn name() -> &'static CStr {{ CStr::from_bytes_with_nul(b"{}\0").unwrap() }}"#,
                        name
                    )?;
                }
                _ => {}
            }
            for version_names in &group_names.versions {
                let version = version_names.version.skip_prefix(VERSION_PREFIX);
                for name in &version_names.names {
                    self.write_group_command(w, group_names.group, version, name)?;
                }
            }
            writeln!(w, "}}")?;
        }
        Ok(())
    }

    pub fn write_lib(&self, path: &Path) -> WriteResult {
        let file = File::create(path)?;
        let mut w = io::BufWriter::new(file);

        let mut header_version = None;
        for ty in self.get_type_iterator() {
            if Some("define") == ty.category.as_ref_str() {
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
        self.write_group_structs(&mut w)?;
        write!(&mut w, "{}", include_str!("lib_postfix.rs"))?;

        Ok(())
    }
}

fn main() -> WriteResult {
    let args: Vec<String> = env::args().collect();
    let xml_file_name = &args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("/usr/share/vulkan/registry/vk.xml");
    let registry = vk::parse_file(Path::new(xml_file_name));

    let generator = Generator::new(&registry);
    generator.write_vk(Path::new("../vkr/src/vk.rs"))?;
    generator.write_builder(Path::new("../vkr/src/builder.rs"))?;
    generator.write_lib(Path::new("../vkr/src/lib.rs"))?;

    Spawn::new("cargo")
        .arg("fmt")
        .current_dir("../vkr")
        .output()
        .expect("failed to run cargo fmt");

    Ok(())
}
