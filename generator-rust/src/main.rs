use heck::{AsShoutySnakeCase, AsSnakeCase, ToUpperCamelCase};
use std::{
    collections::HashSet,
    fmt::{self, Write},
    fs::File,
    io::{self, Write as IoWrite},
    iter, process,
};
use vk_oracle::*;

#[derive(Debug)]
pub enum Error {
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

pub type Res = Result<(), Error>;

struct AsIdent<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsIdent<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.0.as_ref() {
            "type" => f.write_str("ty"),
            _ => f.write_fmt(format_args!("{}", AsSnakeCase(self.0.as_ref()))),
        }
    }
}

struct AsConstantName<'a>(&'a Constant);

impl<'a> fmt::Display for AsConstantName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.0.short_name.starts_with(char::is_numeric) {
            f.write_str("N")?;
        }
        let name = AsShoutySnakeCase(&self.0.short_name);
        f.write_fmt(format_args!("{name}"))
    }
}

struct AsDimAwareUpperCamelCase<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsDimAwareUpperCamelCase<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.0.as_ref();
        for word in s.split('_') {
            let mut chars = word.chars();
            if let Some(c) = chars.next() {
                if matches!(c, '1'..='4') {
                    f.write_char(c)?;
                    if let Some(d) = chars.next() {
                        f.write_char(d.to_ascii_uppercase())?;
                    }
                } else {
                    f.write_char(c.to_ascii_uppercase())?;
                }
            }
            f.write_str(chars.as_str())?;
        }
        Ok(())
    }
}

struct AsTypeName<'a> {
    ty: &'a Type,
    in_option: bool,
    in_namespace: bool,
}

impl<'a> fmt::Display for AsTypeName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.in_option {
            f.write_str("Option<")?;
        }
        if self.in_namespace {
            f.write_str("vk::")?;
        }
        if matches!(&self.ty.detail, TypeDetail::External(_)) {
            f.write_str(&self.ty.spec_name)?;
        } else {
            let name = AsDimAwareUpperCamelCase(&self.ty.short_name);
            if matches!(&self.ty.detail, TypeDetail::FunctionPointer(_)) {
                f.write_fmt(format_args!("Fn{name}"))?;
            } else {
                f.write_fmt(format_args!("{name}"))?;
            }
        }
        if self.in_option {
            f.write_str(">")?;
        }
        Ok(())
    }
}

struct AsNumber(Literal);

impl AsNumber {
    fn is_zero(&self) -> bool {
        match self.0 {
            Literal::Int(n) => n == 0,
            Literal::U32(n) => n == 0,
            Literal::U64(n) => n == 0,
            Literal::F32(n) => n == 0.0,
        }
    }
}

impl fmt::Display for AsNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Literal::Int(n) => f.write_fmt(format_args!("{n}")),
            Literal::U32(n) => f.write_fmt(format_args!("0x{n:x}")),
            Literal::U64(n) => f.write_fmt(format_args!("0x{n:x}")),
            Literal::F32(n) => f.write_fmt(format_args!("{n}_f32")),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct TypeContext {
    is_optional_parameter: Option<bool>,
    is_pointed_to: bool,
    in_namespace: bool,
}

impl TypeContext {
    fn parameter(is_optional: bool) -> Self {
        Self {
            is_optional_parameter: Some(is_optional),
            ..Default::default()
        }
    }

    fn member() -> Self {
        Self::parameter(true)
    }

    fn in_namespace(self) -> Self {
        Self {
            in_namespace: true,
            ..self
        }
    }

    fn array_element(self) -> Self {
        Self {
            is_optional_parameter: None,
            is_pointed_to: false,
            ..self
        }
    }

    fn pointee(self) -> Self {
        Self {
            is_optional_parameter: None,
            is_pointed_to: true,
            ..self
        }
    }
}

fn type_decl_is_non_null(oracle: &Oracle, type_decl: &TypeDecl) -> bool {
    match type_decl {
        TypeDecl::Type(type_index) => match &oracle.types[*type_index].detail {
            TypeDetail::Alias(inner_decl) => type_decl_is_non_null(oracle, inner_decl),
            TypeDetail::FunctionPointer(_) => true,
            _ => false,
        },
        _ => false,
    }
}

fn write_type_decl(w: &mut impl IoWrite, oracle: &Oracle, type_decl: &TypeDecl, context: TypeContext) -> Res {
    match type_decl {
        TypeDecl::BuiltIn(built_in) => {
            write!(
                w,
                "{}",
                match built_in {
                    BuiltInDecl::Void =>
                        if context.is_pointed_to {
                            "c_void"
                        } else {
                            "void"
                        },
                    BuiltInDecl::Char => "c_char",
                    BuiltInDecl::Int => "c_int",
                    BuiltInDecl::F32 => "f32",
                    BuiltInDecl::F64 => "f64",
                    BuiltInDecl::U8 => "u8",
                    BuiltInDecl::U16 => "u16",
                    BuiltInDecl::U32 => "u32",
                    BuiltInDecl::U64 => "u64",
                    BuiltInDecl::I8 => "i8",
                    BuiltInDecl::I16 => "i16",
                    BuiltInDecl::I32 => "i32",
                    BuiltInDecl::I64 => "i64",
                    BuiltInDecl::USize => "usize",
                }
            )?;
        }
        TypeDecl::Type(type_index) => {
            let type_name = AsTypeName {
                ty: &oracle.types[*type_index],
                in_option: context.is_optional_parameter.unwrap_or(true) && type_decl_is_non_null(oracle, type_decl),
                in_namespace: context.in_namespace,
            };
            write!(w, "{type_name}")?;
        }
        TypeDecl::Array(array_decl) => {
            write!(w, "[")?;
            write_type_decl(w, oracle, &array_decl.element_type, context.array_element())?;
            write!(w, "; ")?;
            match &array_decl.array.size {
                ArraySize::Unknown | ArraySize::Named(_) => {
                    panic!("cannot declare array of unknown length");
                }
                ArraySize::Literal(value) => {
                    let number = AsNumber(*value);
                    write!(w, "{number}")?;
                }
                ArraySize::Constant(constant_index) => {
                    let name = AsConstantName(&oracle.constants[*constant_index]);
                    if context.in_namespace {
                        write!(w, "vk::")?;
                    }
                    write!(w, "{name}")?;
                }
            }
            write!(w, "]")?;
        }
        TypeDecl::Pointer(pointer_decl) => {
            if pointer_decl.is_const {
                write!(w, "*const ")?;
            } else {
                write!(w, "*mut ")?;
            }
            write_type_decl(w, oracle, &pointer_decl.element_type, context.pointee())?;
        }
        TypeDecl::External(opaque_decl) => match opaque_decl {
            ExternalDecl::Opaque => {
                if context.is_pointed_to {
                    write!(w, "c_void")?
                } else {
                    write!(w, "Never")?
                }
            }
            ExternalDecl::CULong => write!(w, "c_ulong")?,
        },
    }
    Ok(())
}

fn write_version_comment(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    writeln!(
        w,
        "//! Generated from vk.xml version {}.{}.{}",
        oracle.header_version.0, oracle.header_version.1, oracle.header_version.2
    )?;
    Ok(())
}

fn write_global_constants(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    for constant in oracle
        .constants
        .iter()
        .filter(|constant| constant.enum_type_index.is_none())
    {
        let name = AsConstantName(constant);
        match &constant.value {
            ConstantValue::Literal(literal) => {
                let number = AsNumber(*literal);
                write!(w, "pub const {name}: ")?;
                if let Some(type_decl) = literal.type_decl() {
                    write_type_decl(w, oracle, &type_decl, TypeContext::default())?;
                } else {
                    write!(
                        w,
                        "{}",
                        match constant.spec_name.as_str() {
                            "VK_TRUE" | "VK_FALSE" => "Bool32",
                            _ => "usize",
                        }
                    )?;
                }
                writeln!(w, " = {number};")?;
            }
            ConstantValue::Alias(_) => {}
        }
    }
    Ok(())
}

fn write_type_decl_default(w: &mut impl IoWrite, oracle: &Oracle, ty: &TypeDecl) -> Res {
    match ty {
        TypeDecl::Pointer(pointer_decl) => {
            if pointer_decl.is_const {
                write!(w, "ptr::null()")?;
            } else {
                write!(w, "ptr::null_mut()")?;
            }
        }
        TypeDecl::Array(array_decl) => {
            write!(w, "[")?;
            write_type_decl_default(w, oracle, &array_decl.element_type)?;
            write!(w, ";")?;
            match &array_decl.array.size {
                ArraySize::Unknown | ArraySize::Named(_) => {
                    panic!("cannot set default for array of unknown length")
                }
                ArraySize::Literal(value) => {
                    let number = AsNumber(*value);
                    write!(w, "{number}")?;
                }
                ArraySize::Constant(constant_index) => {
                    let ident = AsConstantName(&oracle.constants[*constant_index]);
                    write!(w, "{ident}")?;
                }
            }
            write!(w, "]")?;
        }
        _ => write!(w, "Default::default()")?,
    }
    Ok(())
}

fn can_derive_hash(oracle: &Oracle, ty: &TypeDecl) -> bool {
    match ty {
        TypeDecl::BuiltIn(built_in) => match built_in {
            BuiltInDecl::Void | BuiltInDecl::F32 | BuiltInDecl::F64 => false,
            BuiltInDecl::Char
            | BuiltInDecl::Int
            | BuiltInDecl::U8
            | BuiltInDecl::U16
            | BuiltInDecl::U32
            | BuiltInDecl::U64
            | BuiltInDecl::I8
            | BuiltInDecl::I16
            | BuiltInDecl::I32
            | BuiltInDecl::I64
            | BuiltInDecl::USize => true,
        },
        TypeDecl::Type(type_index) => {
            let type_info = &oracle.types[*type_index];
            match &type_info.detail {
                TypeDetail::Alias(alias_decl) => can_derive_hash(oracle, alias_decl),
                TypeDetail::Aggregate(agg_type) => {
                    !agg_type.is_union
                        && agg_type
                            .members
                            .iter()
                            .all(|member| can_derive_hash(oracle, &member.ty))
                }
                TypeDetail::Enum(_)
                | TypeDetail::Handle(_)
                | TypeDetail::FunctionPointer(_)
                | TypeDetail::External(_) => true,
            }
        }
        TypeDecl::Array(array_decl) => can_derive_hash(oracle, &array_decl.element_type),
        TypeDecl::Pointer(_) => false,
        TypeDecl::External(_) => true,
    }
}

fn write_types(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    for ty in oracle
        .types
        .iter()
        .filter(|ty| !matches!(ty.spec_name.as_str(), "Version"))
    {
        let type_name = AsTypeName {
            ty,
            in_option: false,
            in_namespace: false,
        };
        match &ty.detail {
            TypeDetail::Alias(type_decl) => {
                write!(w, "pub type {type_name} = ")?;
                write_type_decl(w, oracle, type_decl, TypeContext::default())?;
                writeln!(w, ";")?;
            }
            TypeDetail::External(type_decl) => {
                if ty.spec_name.to_upper_camel_case() != ty.spec_name.as_str() {
                    writeln!(w, "#[allow(non_camel_case_types)]")?;
                }
                write!(w, "pub type {type_name} = ")?;
                write_type_decl(w, oracle, type_decl, TypeContext::default())?;
                writeln!(w, ";")?;
            }
            TypeDetail::Enum(enum_type) => {
                if let Some(bitmask_width) = enum_type.bitmask_width {
                    writeln!(w, "\n#[repr(transparent)]")?;
                    writeln!(w, "#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]")?;
                    writeln!(
                        w,
                        "pub struct {type_name}(pub(crate) {});",
                        match bitmask_width {
                            BitWidth::U32 => "u32",
                            BitWidth::U64 => "u64",
                        }
                    )?;

                    writeln!(w, "impl {type_name} {{")?;
                    let mut seen_short_names = HashSet::new();
                    for constant in enum_type.values.iter().map(|&index| &oracle.constants[index]) {
                        if seen_short_names.insert(constant.short_name.as_str()) {
                            let name = AsConstantName(constant);
                            match constant.value {
                                ConstantValue::Literal(literal) => {
                                    let value = AsNumber(literal);
                                    writeln!(w, "pub const {name}: Self = Self({value});")?;
                                }
                                ConstantValue::Alias(other_index) => {
                                    let other_name = AsConstantName(&oracle.constants[other_index]);
                                    writeln!(w, "pub const {name}: Self = Self::{other_name};")?;
                                }
                            }
                        }
                    }
                    writeln!(w, "}}")?;

                    writeln!(w, "impl_bitmask!({type_name});")?;
                    writeln!(w, "impl fmt::Display for {type_name} {{")?;
                    writeln!(w, "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
                    writeln!(w, "display_bitmask(self.0 as _, &[")?;
                    for constant in enum_type.values.iter().map(|&index| &oracle.constants[index]) {
                        if let ConstantValue::Literal(literal) = constant.value {
                            let name = AsConstantName(constant);
                            let value = AsNumber(literal);
                            if !value.is_zero() {
                                writeln!(w, "({value}, \"{name}\"),")?;
                            }
                        }
                    }
                    writeln!(w, "], f) }} }}")?;
                } else {
                    writeln!(w, "\n#[repr(transparent)]")?;
                    writeln!(
                        w,
                        "#[derive(Debug, Copy, Clone, Default, PartialOrd, Ord, PartialEq, Eq, Hash)]"
                    )?;
                    writeln!(w, "pub struct {type_name}(pub(crate) i32);",)?;

                    writeln!(w, "impl {type_name} {{")?;
                    let mut seen_short_names = HashSet::new();
                    for constant in enum_type.values.iter().map(|&index| &oracle.constants[index]) {
                        if seen_short_names.insert(constant.short_name.as_str()) {
                            let name = AsConstantName(constant);
                            match constant.value {
                                ConstantValue::Literal(literal) => {
                                    let value = AsNumber(literal);
                                    writeln!(w, "pub const {name}: Self = Self({value});")?;
                                }
                                ConstantValue::Alias(other_index) => {
                                    let other_name = AsConstantName(&oracle.constants[other_index]);
                                    writeln!(w, "pub const {name}: Self = Self::{other_name};")?;
                                }
                            }
                        }
                    }
                    writeln!(w, "}}")?;

                    writeln!(w, "impl fmt::Display for {type_name} {{")?;
                    writeln!(w, "fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{")?;
                    writeln!(w, "let name = match self.0 {{")?;
                    for constant in enum_type.values.iter().map(|&index| &oracle.constants[index]) {
                        let name = AsConstantName(constant);
                        if let ConstantValue::Literal(literal) = constant.value {
                            let value = AsNumber(literal);
                            writeln!(w, "{value} => Some(&\"{name}\"),")?;
                        }
                    }
                    writeln!(w, "_ => None, }};")?;
                    writeln!(
                        w,
                        "if let Some(name) = name {{ f.write_str(name) }} else {{ write!(f, \"{{}}\", self.0) }}"
                    )?;
                    writeln!(w, "}} }}")?;
                }
            }
            TypeDetail::Aggregate(aggregate_type) => {
                let agg_type = if aggregate_type.is_union { "union" } else { "struct" };
                writeln!(w, "\n#[repr(C)]")?;
                let can_derive_debug = !aggregate_type.is_union
                    && aggregate_type.members.iter().all(|member| {
                        !matches!(
                            &member.ty,
                            TypeDecl::Pointer(_) | TypeDecl::External(_) | TypeDecl::Array(_)
                        )
                    });
                let can_derive_default = !aggregate_type.is_union
                    && aggregate_type.members.iter().all(|member| {
                        member.default.is_none()
                            && !matches!(
                                &member.ty,
                                TypeDecl::Pointer(_) | TypeDecl::External(_) | TypeDecl::Array(_)
                            )
                    });
                let mut derives = vec!["Copy", "Clone"];
                if can_derive_debug {
                    derives.push("Debug");
                }
                if can_derive_default {
                    derives.push("Default");
                }
                if can_derive_hash(oracle, &TypeDecl::Type(ty.index)) {
                    derives.push("PartialEq");
                    derives.push("Eq");
                    derives.push("Hash");
                }
                write!(w, "#[derive({})]", derives.join(","))?;
                writeln!(w, "pub {agg_type} {type_name} {{")?;
                for member in &aggregate_type.members {
                    let ident = AsIdent(&member.short_name);
                    write!(w, "pub {ident}: ")?;
                    write_type_decl(w, oracle, &member.ty, TypeContext::member())?;
                    writeln!(w, ",")?;
                }
                writeln!(w, "}}")?;
                if aggregate_type
                    .members
                    .iter()
                    .any(|member| matches!(&member.ty, TypeDecl::Pointer(_)))
                {
                    writeln!(w, "unsafe impl Send for {type_name} {{ }}")?;
                    writeln!(w, "unsafe impl Sync for {type_name} {{ }}")?;
                }
                if !can_derive_default {
                    writeln!(w, "impl Default for {type_name} {{ fn default() -> Self {{")?;
                    if aggregate_type.is_union {
                        write!(w, "unsafe {{ mem::zeroed() }}")?;
                    } else {
                        write!(w, "Self {{")?;
                        for member in &aggregate_type.members {
                            let ident = AsIdent(&member.short_name);
                            write!(w, "{ident}: ")?;
                            if let Some(constant_index) = member.default {
                                let constant = &oracle.constants[constant_index];
                                if let Some(enum_type_index) = constant.enum_type_index {
                                    let enum_type_name = AsTypeName {
                                        ty: &oracle.types[enum_type_index],
                                        in_option: false,
                                        in_namespace: false,
                                    };
                                    write!(w, "{enum_type_name}::")?;
                                }
                                let constant_name = AsConstantName(constant);
                                write!(w, "{constant_name}")?;
                            } else {
                                write_type_decl_default(w, oracle, &member.ty)?;
                            }
                            writeln!(w, ",")?;
                        }
                        write!(w, "}}")?;
                    }
                    writeln!(w, "}} }}")?;
                }
                if !can_derive_debug {
                    writeln!(
                        w,
                        "impl fmt::Debug for {type_name} {{ fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {{"
                    )?;
                    writeln!(w, "fmt.debug_struct(\"{type_name}\")")?;
                    for member in &aggregate_type.members {
                        let ident = AsIdent(&member.short_name);
                        if aggregate_type.is_union {
                            writeln!(w, ".field(\"{ident}\", unsafe {{ &self.{ident} }})")?;
                        } else {
                            writeln!(w, ".field(\"{ident}\", &self.{ident})")?;
                        }
                    }
                    writeln!(w, ".finish() }} }}")?;
                }
            }
            TypeDetail::Handle(handle_type) => {
                let interior_type = match handle_type {
                    HandleType::USize => "usize",
                    HandleType::U64 => "u64",
                };
                writeln!(w, "\n#[repr(transparent)]")?;
                writeln!(w, "#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]")?;
                writeln!(w, "pub struct {type_name}({interior_type});")?;
                writeln!(w, "impl_handle!({type_name});")?;
            }
            TypeDetail::FunctionPointer(function_pointer_type) => {
                writeln!(w, "pub type {type_name} = unsafe extern \"system\" fn(")?;
                for parameter in &function_pointer_type.parameters {
                    let ident = AsIdent(&parameter.name);
                    write!(w, "{ident}: ")?;
                    write_type_decl(w, oracle, &parameter.ty, TypeContext::default())?;
                    write!(w, ",")?;
                }
                write!(w, ")")?;
                if !matches!(function_pointer_type.return_type, TypeDecl::BuiltIn(BuiltInDecl::Void)) {
                    write!(w, " -> ")?;
                    write_type_decl(w, oracle, &function_pointer_type.return_type, TypeContext::default())?;
                }
                writeln!(w, ";")?;
            }
        }
    }
    Ok(())
}

enum PointerTransform {
    None,
    Ref,
    CStr,
}

impl PointerTransform {
    fn from_type_decl(type_decl: &TypeDecl, is_parameter: bool) -> Self {
        if let TypeDecl::Pointer(pointer_decl) = &type_decl {
            if let Some(array_info) = &pointer_decl.array_hint {
                if matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::Char))
                    && pointer_decl.is_const
                    && array_info.is_null_terminated
                {
                    return Self::CStr;
                }
            } else if (is_parameter || pointer_decl.is_const)
                && !matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::Void))
            {
                return Self::Ref;
            }
        }
        Self::None
    }
}

fn write_command_param_declaration(
    w: &mut impl IoWrite,
    oracle: &Oracle,
    param: &ParameterDecl,
    single_element_slices: bool,
) -> Res {
    let ident = AsIdent(&param.name);
    match &param.transform {
        ParameterTransform::None => {
            write!(w, "{ident}: ")?;
            match PointerTransform::from_type_decl(&param.ty, true) {
                PointerTransform::CStr => {
                    if param.is_optional {
                        write!(w, "Option<&CStr>")?;
                    } else {
                        write!(w, "&CStr")?;
                    }
                }
                PointerTransform::Ref => {
                    if param.is_optional {
                        write!(w, "Option<")?;
                    }
                    let TypeDecl::Pointer(pointer_decl) = &param.ty else {
                        panic!("unexpected type for pointer");
                    };
                    write!(w, "&")?;
                    if !pointer_decl.is_const {
                        write!(w, "mut ")?;
                    }
                    write_type_decl(
                        w,
                        oracle,
                        &pointer_decl.element_type,
                        TypeContext::default().in_namespace(),
                    )?;
                    if param.is_optional {
                        write!(w, ">")?;
                    }
                }
                PointerTransform::None => write_type_decl(
                    w,
                    oracle,
                    &param.ty,
                    TypeContext::parameter(param.is_optional).in_namespace(),
                )?,
            }
            writeln!(w, ",")?;
        }
        ParameterTransform::FromBool => {
            write!(w, "{ident}: bool,")?;
        }
        ParameterTransform::FromMemberHandle => {}
        ParameterTransform::FromSlice(_) => {
            write!(w, "{ident}: ")?;
            if param.is_optional {
                write!(w, "Option<")?;
            }
            write!(w, "&")?;
            let TypeDecl::Pointer(pointer_decl) = &param.ty else {
                panic!("unexpected type for slice");
            };
            if !pointer_decl.is_const {
                write!(w, "mut ")?;
            }
            if !single_element_slices {
                write!(w, "[")?;
            }
            if matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::Void)) {
                write!(w, "u8")?;
            } else {
                write_type_decl(
                    w,
                    oracle,
                    &pointer_decl.element_type,
                    TypeContext::default().in_namespace(),
                )?;
            }
            if !single_element_slices {
                write!(w, "]")?;
            }
            if param.is_optional {
                write!(w, ">")?;
            }
            writeln!(w, ",")?;
        }
        ParameterTransform::FromSliceLength { .. } | ParameterTransform::FromOutput => {}
    }
    Ok(())
}

fn write_command_param_body(
    w: &mut impl IoWrite,
    oracle: &Oracle,
    param: &ParameterDecl,
    fp_type: &FunctionPointerType,
) -> Res {
    match &param.transform {
        ParameterTransform::FromSlice(transform) => {
            if let Some(length_check) = transform.length_check.as_deref() {
                let ident = AsIdent(&param.name);
                if param.is_optional {
                    writeln!(
                        w,
                        "if let Some(s) = {ident} {{ assert_eq!({length_check} as usize, s.len()); }}"
                    )?;
                } else {
                    writeln!(w, "assert_eq!({length_check} as usize, {ident}.len());")?;
                }
            }
        }
        ParameterTransform::FromSliceLength(transform) => {
            let ident = AsIdent(&param.name);
            let as_dest = match &param.ty {
                TypeDecl::BuiltIn(BuiltInDecl::U32) => "as u32",
                TypeDecl::BuiltIn(BuiltInDecl::USize) => "",
                TypeDecl::Type(type_index) => {
                    let spec_name = oracle.types[*type_index].spec_name.as_str();
                    match spec_name {
                        "VkDeviceSize" => "as vk::DeviceSize",
                        _ => panic!("unexpected destination type {spec_name} for slice length"),
                    }
                }
                _ => panic!("unexpected destination type {:?} for slice length", param.ty),
            };
            let mut is_first = true;
            for &param_index in &transform.required_param {
                let slice_ident = AsIdent(&fp_type.parameters[param_index].name);
                if is_first {
                    write!(w, "let {ident} = {slice_ident}.len() {as_dest};")?;
                    is_first = false;
                } else {
                    writeln!(w, "assert_eq!({ident}, {slice_ident}.len() {as_dest});")?;
                }
            }
            for &param_index in &transform.optional_param {
                let slice_ident = AsIdent(&fp_type.parameters[param_index].name);
                writeln!(
                    w,
                    "if let Some(s) = {slice_ident} {{ assert_eq!({ident}, s.len() {as_dest}); }}"
                )?;
            }
        }
        ParameterTransform::FromOutput => {
            let ident = AsIdent(&param.name);
            write!(w, "let mut {ident} = MaybeUninit::<_>::uninit();")?;
        }
        _ => {}
    }
    Ok(())
}

fn write_command_param_forward(w: &mut impl IoWrite, oracle: &Oracle, param: &ParameterDecl) -> Res {
    let ident = AsIdent(&param.name);
    match &param.transform {
        ParameterTransform::None => {
            if type_decl_is_non_null(oracle, &param.ty) {
                if param.is_optional {
                    write!(w, "{ident}")?;
                } else {
                    write!(w, "Some({ident})")?;
                }
            } else {
                match PointerTransform::from_type_decl(&param.ty, true) {
                    PointerTransform::None => write!(w, "{ident}")?,
                    PointerTransform::Ref => {
                        if param.is_optional {
                            let TypeDecl::Pointer(pointer_decl) = &param.ty else {
                                panic!("expected pointer type for slice parameter");
                            };
                            if pointer_decl.is_const {
                                write!(w, "{ident}.map_or(ptr::null(), |r| r)")?;
                            } else {
                                write!(w, "{ident}.map_or(ptr::null_mut(), |r| r)")?;
                            }
                        } else {
                            write!(w, "{ident}")?;
                        }
                    }
                    PointerTransform::CStr => {
                        if param.is_optional {
                            write!(w, "{ident}.map_or(ptr::null(), |s| s.as_ptr())")?;
                        } else {
                            write!(w, "{ident}.as_ptr()")?;
                        }
                    }
                }
            }
        }
        ParameterTransform::FromBool => {
            write!(w, "if {ident} {{ vk::TRUE }} else {{ vk::FALSE }}")?;
        }
        ParameterTransform::FromMemberHandle => {
            write!(w, "self.handle")?;
        }
        ParameterTransform::FromSlice(_) => {
            let TypeDecl::Pointer(pointer_decl) = &param.ty else {
                panic!("expected pointer type for slice parameter");
            };
            let as_ptr = if pointer_decl.is_const { "as_ptr" } else { "as_mut_ptr" };
            if param.is_optional {
                let null = if pointer_decl.is_const { "null" } else { "null_mut" };
                write!(w, "{ident}.map_or(ptr::{null}(), |r| r.{as_ptr}())")?;
            } else {
                write!(w, "{ident}.{as_ptr}()")?;
            }
            if matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::Void)) {
                if pointer_decl.is_const {
                    write!(w, " as *const _")?;
                } else {
                    write!(w, " as *mut _")?;
                }
            }
        }
        ParameterTransform::FromSliceLength { .. } => {
            write!(w, "{ident}")?;
        }
        ParameterTransform::FromOutput => {
            write!(w, "{ident}.as_mut_ptr()")?;
        }
    }
    Ok(())
}

fn write_commands(w: &mut impl IoWrite, oracle: &Oracle, category: CommandCategory) -> Res {
    let struct_name = match category {
        CommandCategory::Global => "Globals",
        CommandCategory::Instance => "Instance",
        CommandCategory::Device => "Device",
    };
    writeln!(w, "\n#[derive(Copy, Clone)]")?;
    writeln!(w, "pub struct {struct_name} {{")?;
    match category {
        CommandCategory::Global => {}
        CommandCategory::Instance => {
            writeln!(w, "pub handle: vk::Instance, pub extensions: InstanceExtensions,")?;
        }
        CommandCategory::Device => {
            writeln!(w, "pub handle: vk::Device, pub extensions: DeviceExtensions,")?;
        }
    }
    for cmd in oracle.commands.iter().filter(|cmd| cmd.category == category) {
        if let CommandDetail::Function(cmd_func) = &cmd.detail {
            let function_name = AsIdent(&cmd.short_name);
            let type_name = AsTypeName {
                ty: &oracle.types[cmd_func.function_type_index],
                in_option: !cmd.dependencies.is_always(),
                in_namespace: true,
            };
            writeln!(w, "pub fp_{function_name}: {type_name},")?;
        }
    }
    writeln!(w, "}}")?;

    writeln!(w, "impl {struct_name} {{")?;

    if let Some(impl_file_name) = match category {
        CommandCategory::Global => Some("input/global_impl.rs"),
        CommandCategory::Instance => Some("input/instance_impl.rs"),
        CommandCategory::Device => None,
    } {
        io::copy(&mut io::BufReader::new(File::open(impl_file_name)?), w)?;
    }

    match category {
        CommandCategory::Global => {
            writeln!(
                w,
                "pub fn new() -> LoadResult<Self> {{\
                let lib = LIB.as_ref().map_err(|e| e.clone())?;
                unsafe {{ Self::load(lib.fp_get_instance_proc_addr) }} }}"
            )?;
            writeln!(
                w,
                "pub unsafe fn load(get_instance_proc_addr: vk::FnGetInstanceProcAddr) -> LoadResult<Self> {{\
                Ok(Self {{"
            )?;
        }
        CommandCategory::Instance => {
            writeln!(w, "#[allow(clippy::cognitive_complexity, clippy::nonminimal_bool)]")?;
            writeln!(
                w,
                "pub unsafe fn load(globals: &Globals, instance: vk::Instance, create_info: &vk::InstanceCreateInfo) -> LoadResult<Self> {{\
                let version = create_info.p_application_info.as_ref().map(|app_info| app_info.api_version).unwrap_or_default();\
                let mut extensions = InstanceExtensions::new(version);")?;
            writeln!(w,
                "if create_info.enabled_extension_count != 0 {{\
                for &name_ptr in slice::from_raw_parts(create_info.pp_enabled_extension_names, create_info.enabled_extension_count as usize) {{\
                extensions.enable_by_name(CStr::from_ptr(name_ptr)); }} }}")?;
            writeln!(w, "Ok(Self {{ handle: instance, extensions,")?;
        }
        CommandCategory::Device => {
            writeln!(w, "#[allow(clippy::cognitive_complexity, clippy::nonminimal_bool)]")?;
            writeln!(
                w,
                "pub unsafe fn load(globals: &Globals, instance: &Instance, device: vk::Device, create_info: &vk::DeviceCreateInfo) -> LoadResult<Self> {{\
                let mut extensions = DeviceExtensions::new(instance.extensions.core_version);")?;
            writeln!(w,
                "if create_info.enabled_extension_count != 0 {{\
                for &name_ptr in slice::from_raw_parts(create_info.pp_enabled_extension_names, create_info.enabled_extension_count as usize) {{\
                extensions.enable_by_name(CStr::from_ptr(name_ptr)); }} }}")?;
            writeln!(w, "Ok(Self {{ handle: device, extensions,")?;
        }
    }
    for (cmd, cmd_func) in oracle.commands.iter().filter_map(|cmd| match &cmd.detail {
        CommandDetail::Function(cmd_func) if cmd.category == category => Some((cmd, cmd_func)),
        _ => None,
    }) {
        let function_name = AsIdent(&cmd.short_name);
        write!(w, "fp_{function_name}: ")?;

        let loader = match category {
            CommandCategory::Global => {
                match cmd.spec_name.as_str() {
                    "vkGetInstanceProcAddr" => {
                        // special case: copy loader entry point
                        writeln!(w, "get_instance_proc_addr,")?;
                        continue;
                    }
                    "vkEnumerateInstanceVersion" => {
                        // special case: always try to load, allow None if not present
                        writeln!(
                            w,
                            "get_instance_proc_addr(vk::Instance::null(), c\"vkEnumerateInstanceVersion\".as_ptr()).map(|f| mem::transmute(f)),"
                        )?;
                        continue;
                    }
                    _ => "get_instance_proc_addr(vk::Instance::null(), ",
                }
            }
            CommandCategory::Instance => "globals.get_instance_proc_addr(instance, ",
            CommandCategory::Device => {
                if cmd_func.load_on_instance {
                    "globals.get_instance_proc_addr(instance.handle, "
                } else {
                    "instance.get_device_proc_addr(device, "
                }
            }
        };
        let name_postfix = match category {
            CommandCategory::Global => ".as_ptr()",
            CommandCategory::Instance | CommandCategory::Device => "",
        };

        if cmd.dependencies.is_always() {
            let spec_name = &cmd.spec_name;
            writeln!(w, "mem::transmute({loader}c\"{spec_name}\"{name_postfix})")?;
            writeln!(w, ".ok_or(LoadError::MissingSymbol(c\"{spec_name}\"))?),")?;
        } else {
            for alt_cmd in iter::once(cmd).chain(cmd_func.aliases.iter().map(|&index| &oracle.commands[index])) {
                let spec_name = &alt_cmd.spec_name;
                write!(w, "if ")?;
                write_check_dependency(w, oracle, category, &alt_cmd.dependencies)?;
                writeln!(w, " {{ {loader}c\"{spec_name}\").map(|f| mem::transmute(f)) }} else ")?;
            }
            writeln!(w, "{{ None }},")?;
        }
    }
    writeln!(w, "}}) }}")?;

    for cmd in oracle.commands.iter().filter(|cmd| cmd.category == category) {
        let is_special_case = matches!(cmd.spec_name.as_str(), "vkEnumerateInstanceVersion");
        if is_special_case {
            continue;
        }

        let cmd_impl = match &cmd.detail {
            CommandDetail::Function(_) => cmd,
            CommandDetail::Alias(cmd_index) => &oracle.commands[*cmd_index],
        };
        let CommandDetail::Function(cmd_func) = &cmd_impl.detail else {
            panic!("expected command function definition");
        };

        let function_name = AsIdent(&cmd.short_name);
        let TypeDetail::FunctionPointer(fp_type) = &oracle.types[cmd_func.function_type_index].detail else {
            panic!("expected function pointer type");
        };

        let has_error_set = !cmd_func.error_codes.is_empty() || !cmd_func.success_codes.is_empty();

        writeln!(w, "pub unsafe fn {function_name}(&self, ")?;
        for param in &fp_type.parameters {
            write_command_param_declaration(w, oracle, param, false)?;
        }
        write!(w, ")")?;

        match &cmd_func.output_transform {
            CommandOutputTransform::None => {
                if cmd_func.success_codes.is_empty() {
                    let is_void = matches!(&fp_type.return_type, TypeDecl::BuiltIn(BuiltInDecl::Void));
                    if !is_void {
                        write!(w, " -> ")?;
                        write_type_decl(w, oracle, &fp_type.return_type, TypeContext::default().in_namespace())?;
                    }
                } else if cmd_func.success_codes.len() == 1 {
                    write!(w, " -> Result<()>")?;
                } else {
                    write!(w, " -> Result<vk::Result>")?;
                }
            }
            CommandOutputTransform::IntoObject(transform) => {
                write!(w, " -> ")?;
                if has_error_set {
                    write!(w, "Result<")?;
                }
                if transform.into_bool {
                    write!(w, "bool")?;
                } else {
                    let TypeDecl::Pointer(pointer_decl) = &fp_type.parameters[transform.output_param].ty else {
                        panic!("expected pointer type for output");
                    };
                    write_type_decl(
                        w,
                        oracle,
                        &pointer_decl.element_type,
                        TypeContext::parameter(false).in_namespace(),
                    )?;
                }
                if has_error_set {
                    write!(w, ">")?;
                }
            }
            CommandOutputTransform::IntoEnumerate(_) => {
                if has_error_set {
                    write!(w, "-> Result<EnumerateResult>")?;
                }
            }
            CommandOutputTransform::IntoBool => {
                write!(w, " -> bool")?;
            }
        }
        writeln!(w, "{{")?;

        let impl_name = AsIdent(&cmd_impl.short_name);
        if cmd.dependencies.is_always() {
            writeln!(w, "let fp = self.fp_{impl_name};")?;
        } else {
            let spec_name = cmd.spec_name.as_str();
            writeln!(w, "let fp = self.fp_{impl_name}.expect(\"{spec_name} is not loaded\");")?;
        }

        for param in &fp_type.parameters {
            write_command_param_body(w, oracle, param, fp_type)?;
        }

        if !cmd_func.success_codes.is_empty() {
            write!(w, "let err = ")?
        }

        write!(w, "(fp)(")?;
        for param in &fp_type.parameters {
            write_command_param_forward(w, oracle, param)?;
            write!(w, ",")?;
        }
        write!(w, ")")?;

        match &cmd_func.output_transform {
            CommandOutputTransform::None => {
                if !cmd_func.success_codes.is_empty() {
                    writeln!(w, "; match err {{")?;
                    let mut counter = 0;
                    for &index in &cmd_func.success_codes {
                        let name = AsConstantName(&oracle.constants[index]);
                        if counter > 0 {
                            write!(w, " | ")?;
                        }
                        counter += 1;
                        write!(w, "vk::Result::{name}")?;
                    }
                    if counter == 1 {
                        writeln!(w, "=> Ok(()),")?;
                    } else {
                        writeln!(w, "=> Ok(err),")?;
                    }
                    writeln!(w, "_ => Err(err),")?;
                    writeln!(w, "}}")?;
                }
            }
            CommandOutputTransform::IntoObject(transform) => {
                let param = &fp_type.parameters[transform.output_param];
                let param_ident = AsIdent(&param.name);
                let is_non_null = match &param.ty {
                    TypeDecl::Pointer(pointer_decl) => type_decl_is_non_null(oracle, &pointer_decl.element_type),
                    _ => {
                        panic!("expected object parameter to be a pointer type")
                    }
                };
                if cmd_func.success_codes.is_empty() {
                    let postfix = if transform.into_bool {
                        " != vk::FALSE"
                    } else if is_non_null {
                        ".unwrap()"
                    } else {
                        ""
                    };
                    writeln!(w, "; {param_ident}.assume_init(){postfix}")?
                } else {
                    writeln!(w, "; match err {{")?;
                    for (counter, &index) in cmd_func.success_codes.iter().enumerate() {
                        let name = AsConstantName(&oracle.constants[index]);
                        if counter > 0 {
                            write!(w, " | ")?;
                        }
                        write!(w, "vk::Result::{name}")?;
                    }
                    write!(w, " => ")?;
                    if transform.into_bool {
                        write!(w, "Ok({param_ident}.assume_init() != vk::FALSE),")?;
                    } else if is_non_null {
                        write!(w, "{param_ident}.assume_init().ok_or(vk::Result::ERROR_UNKNOWN),")?;
                    } else {
                        write!(w, "Ok({param_ident}.assume_init()),")?;
                    }
                    writeln!(w, "_ => Err(err),")?;
                    writeln!(w, "}}")?;
                }
            }
            CommandOutputTransform::IntoEnumerate(_) => {
                if cmd_func.success_codes.is_empty() {
                    writeln!(w, ";")?;
                } else {
                    writeln!(
                        w,
                        "; match err {{\
                         vk::Result::SUCCESS => Ok(EnumerateResult::Success),\
                         vk::Result::INCOMPLETE => Ok(EnumerateResult::Incomplete),\
                         _ => Err(err),}}"
                    )?;
                }
            }
            CommandOutputTransform::IntoBool => {
                writeln!(w, " != vk::FALSE")?;
            }
        }
        writeln!(w, "}} ")?;

        if let CommandOutputTransform::IntoEnumerate(transform) = &cmd_func.output_transform {
            let element_decl = match &fp_type.parameters[transform.elements_param].ty {
                TypeDecl::Pointer(pointer_decl) => pointer_decl.element_type.as_ref(),
                _ => panic!("expected pointer type for elements"),
            };

            writeln!(w, "pub unsafe fn {function_name}_to_vec(&self, ")?;
            for (_, param) in fp_type
                .parameters
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != transform.count_param && index != transform.elements_param)
            {
                write_command_param_declaration(w, oracle, param, false)?;
            }
            write!(w, ") -> ")?;

            let needs_error_code = !cmd_func.error_codes.is_empty();

            if needs_error_code {
                write!(w, "Result<")?;
            }
            write!(w, "Vec<")?;
            write_type_decl(w, oracle, element_decl, TypeContext::default().in_namespace())?;
            write!(w, ">")?;
            if needs_error_code {
                write!(w, ">")?;
            }
            writeln!(w, "{{")?;

            if needs_error_code {
                write!(w, "enumerate_generic_to_vec")?;
            } else {
                write!(w, "enumerate_generic_unchecked_to_vec")?;
            }
            write!(w, "(|len, ptr| self.{function_name}(")?;
            for (index, param) in fp_type.parameters.iter().enumerate() {
                if index == transform.count_param {
                    write!(w, "len,")?;
                } else if index == transform.elements_param {
                    write!(w, "ptr,")?;
                } else {
                    match &param.transform {
                        ParameterTransform::FromMemberHandle | ParameterTransform::FromSliceLength(_) => {}
                        _ => {
                            let param_ident = AsIdent(&param.name);
                            write!(w, "{param_ident},")?;
                        }
                    }
                }
            }
            writeln!(w, ")) }}")?;
        }

        let output_param_indices: Vec<usize> = fp_type
            .parameters
            .iter()
            .enumerate()
            .filter_map(|(index, param)| {
                (matches!(param.transform, ParameterTransform::FromSlice(_))
                    && match &param.ty {
                        TypeDecl::Pointer(pointer_decl) => !pointer_decl.is_const,
                        _ => false,
                    })
                .then_some(index)
            })
            .collect();
        if matches!(&cmd_func.output_transform, CommandOutputTransform::None) && output_param_indices.len() == 1 {
            let output_param_index = output_param_indices[0];

            let element_decl = match &fp_type.parameters[output_param_index].ty {
                TypeDecl::Pointer(pointer_decl) => pointer_decl.element_type.as_ref(),
                _ => panic!("expected pointer type for output"),
            };
            if !matches!(element_decl, TypeDecl::BuiltIn(BuiltInDecl::Void)) {
                writeln!(w, "pub unsafe fn {function_name}_single(&self, ")?;
                for (_, param) in fp_type
                    .parameters
                    .iter()
                    .enumerate()
                    .filter(|&(index, _)| index != output_param_index)
                {
                    write_command_param_declaration(w, oracle, param, true)?;
                }
                write!(w, ") -> ")?;

                let needs_success_code = cmd_func.success_codes.len() > 1;

                write!(w, "Result<")?;
                if needs_success_code {
                    write!(w, "(vk::Result, ")?;
                }
                write_type_decl(w, oracle, element_decl, TypeContext::default().in_namespace())?;
                if needs_success_code {
                    write!(w, ")")?;
                }
                writeln!(w, "> {{")?;

                let output_ident = AsIdent(&fp_type.parameters[output_param_index].name);
                writeln!(
                    w,
                    "let mut {output_ident} = Default::default();\
                    self.{function_name}("
                )?;
                for (index, param) in fp_type.parameters.iter().enumerate() {
                    if index == output_param_index {
                        write!(w, "slice::from_mut(&mut {output_ident}),")?;
                    } else {
                        let param_ident = AsIdent(&param.name);
                        match &param.transform {
                            ParameterTransform::FromMemberHandle | ParameterTransform::FromSliceLength(_) => {}
                            ParameterTransform::FromSlice(_) => {
                                if param.is_optional {
                                    write!(w, "{param_ident}.map(slice::from_ref),")?;
                                } else {
                                    write!(w, "slice::from_ref({param_ident}),")?;
                                }
                            }
                            _ => {
                                write!(w, "{param_ident},")?;
                            }
                        }
                    }
                }
                writeln!(w, ").map(")?;
                if needs_success_code {
                    write!(w, "|res| (res, {output_ident})")?;
                } else {
                    write!(w, "|_| {output_ident}")?;
                }
                writeln!(w, ") }}")?;
            }
        }
    }

    writeln!(w, "}}")?;
    Ok(())
}

fn write_supports_dependency(
    w: &mut impl IoWrite,
    oracle: &Oracle,
    current_index: ExtensionIndex,
    expr: &ExtensionDependencyExpr,
) -> Res {
    match expr {
        ExtensionDependencyExpr::Never | ExtensionDependencyExpr::Always => {
            unimplemented!()
        }
        ExtensionDependencyExpr::Version(v) => {
            write!(
                w,
                "self.core_version >= vk::Version::from_raw_parts({}, {}, 0)",
                v.major, v.minor
            )?;
        }
        ExtensionDependencyExpr::Extension(index) => {
            let ident = AsIdent(&oracle.extensions[*index].short_name);
            if *index == current_index {
                write!(w, "self.{ident}")?;
            } else {
                write!(w, "self.supports_{ident}()")?;
            }
        }
        ExtensionDependencyExpr::Feature => {
            unimplemented!("feature dependencies not implement yet");
        }
        ExtensionDependencyExpr::And(v) => {
            for (i, dep) in v.iter().enumerate() {
                if i != 0 {
                    write!(w, " && ")?;
                }
                let bracket = matches!(dep, ExtensionDependencyExpr::Or(_));
                if bracket {
                    write!(w, "(")?;
                }
                write_supports_dependency(w, oracle, current_index, dep)?;
                if bracket {
                    write!(w, ")")?;
                }
            }
        }
        ExtensionDependencyExpr::Or(v) => {
            for (i, dep) in v.iter().enumerate() {
                if i != 0 {
                    write!(w, " || ")?;
                }
                let bracket = matches!(dep, ExtensionDependencyExpr::And(_));
                if bracket {
                    write!(w, "(")?;
                }
                write_supports_dependency(w, oracle, current_index, dep)?;
                if bracket {
                    write!(w, ")")?;
                }
            }
        }
    }
    Ok(())
}

fn write_enable_dependency(
    w: &mut impl IoWrite,
    oracle: &Oracle,
    current_index: ExtensionIndex,
    expr: &ExtensionDependencyExpr,
) -> Res {
    match expr {
        ExtensionDependencyExpr::Never | ExtensionDependencyExpr::Always => {
            unimplemented!()
        }
        ExtensionDependencyExpr::Version(v) => {
            writeln!(
                w,
                "// depends on minimum core version, caller must specify\n\
                 debug_assert!(self.core_version >= vk::Version::from_raw_parts({}, {}, 0));",
                v.major, v.minor
            )?;
        }
        ExtensionDependencyExpr::Extension(index) => {
            let ident = AsIdent(&oracle.extensions[*index].short_name);
            if *index == current_index {
                writeln!(w, "self.{ident} = true;")?;
            } else {
                writeln!(w, "self.enable_{ident}();")?;
            }
        }
        ExtensionDependencyExpr::Feature => {
            unimplemented!("feature dependencies not implement yet");
        }
        ExtensionDependencyExpr::And(deps) => {
            for dep in deps.iter() {
                write_enable_dependency(w, oracle, current_index, dep)?;
            }
        }
        ExtensionDependencyExpr::Or(deps) => {
            // only handle pairs
            if deps.len() == 2 {
                match (&deps[0], &deps[1]) {
                    (ExtensionDependencyExpr::Version(v), other) | (other, ExtensionDependencyExpr::Version(v)) => {
                        writeln!(
                            w,
                            "if self.core_version < vk::Version::from_raw_parts({}, {}, 0) {{",
                            v.major, v.minor
                        )?;
                        write_enable_dependency(w, oracle, current_index, other)?;
                        writeln!(w, "}}")?;
                    }
                    (ExtensionDependencyExpr::Extension(index_a), ExtensionDependencyExpr::Extension(index_b)) => {
                        let ident_a = AsIdent(&oracle.extensions[*index_a].short_name);
                        let ident_b = AsIdent(&oracle.extensions[*index_b].short_name);
                        writeln!(
                            w,
                            "// ambiguous dependency, caller must enable one explicitly\n\
                             debug_assert!(self.supports_{ident_a}() || self.supports_{ident_b}());"
                        )?;
                    }
                    _ => unimplemented!(),
                }
            } else {
                match (&deps[0], &deps[1], &deps[2]) {
                    (
                        ExtensionDependencyExpr::Extension(index_a),
                        ExtensionDependencyExpr::Extension(index_b),
                        ExtensionDependencyExpr::Extension(index_c),
                    ) => {
                        let ident_a = AsIdent(&oracle.extensions[*index_a].short_name);
                        let ident_b = AsIdent(&oracle.extensions[*index_b].short_name);
                        let ident_c = AsIdent(&oracle.extensions[*index_c].short_name);
                        writeln!(
                            w,
                            "// ambiguous dependency, caller must enable one explicitly\n\
                             debug_assert!(self.supports_{ident_a}() || self.supports_{ident_b}() || self.supports_{ident_c}());"
                        )?;
                    }
                    _ => {
                        println!("{deps:?}");
                        unimplemented!()
                    }
                }
            }
        }
    }
    Ok(())
}

fn write_check_dependency(
    w: &mut impl IoWrite,
    oracle: &Oracle,
    category: CommandCategory,
    expr: &ExtensionDependencyExpr,
) -> Res {
    match expr {
        ExtensionDependencyExpr::Never => {
            write!(w, "false")?;
        }
        ExtensionDependencyExpr::Always => {
            write!(w, "true")?;
        }
        ExtensionDependencyExpr::Version(v) => {
            write!(
                w,
                "extensions.core_version >= vk::Version::from_raw_parts({}, {}, 0)",
                v.major, v.minor
            )?;
        }
        ExtensionDependencyExpr::Feature => {
            unimplemented!("feature dependencies not implement yet");
        }
        ExtensionDependencyExpr::Extension(index) => {
            let ext = &oracle.extensions[*index];
            if category == CommandCategory::Device && ext.category == ExtensionCategory::Instance {
                write!(w, "instance.")?;
            }
            let ident = AsIdent(&ext.short_name);
            write!(w, "extensions.{ident}")?;
        }
        ExtensionDependencyExpr::And(deps) => {
            for (i, dep) in deps.iter().enumerate() {
                if i != 0 {
                    write!(w, " && ")?;
                }
                let bracket = matches!(dep, ExtensionDependencyExpr::Or(_));
                if bracket {
                    write!(w, "(")?;
                }
                write_check_dependency(w, oracle, category, dep)?;
                if bracket {
                    write!(w, ")")?;
                }
            }
        }
        ExtensionDependencyExpr::Or(deps) => {
            for (i, dep) in deps.iter().enumerate() {
                if i != 0 {
                    write!(w, " || ")?;
                }
                let bracket = matches!(dep, ExtensionDependencyExpr::And(_));
                if bracket {
                    write!(w, "(")?;
                }
                write_check_dependency(w, oracle, category, dep)?;
                if bracket {
                    write!(w, ")")?;
                }
            }
        }
    }
    Ok(())
}

fn write_extensions(w: &mut impl IoWrite, oracle: &Oracle, category: ExtensionCategory) -> Res {
    let struct_name = match category {
        ExtensionCategory::Instance => "InstanceExtensions",
        ExtensionCategory::Device => "DeviceExtensions",
    };
    writeln!(w, "#[derive(Debug, Copy, Clone)]")?;
    writeln!(w, "pub struct {struct_name} {{ pub core_version: vk::Version,")?;
    for ext in oracle.extensions.iter().filter(|ext| ext.category == category) {
        let ident = AsIdent(&ext.short_name);
        writeln!(w, "pub {ident}: bool,")?;
    }
    writeln!(w, "}}")?;

    writeln!(w, "impl {struct_name} {{")?;

    writeln!(w, "fn enable_by_name(&mut self, name: &CStr) {{")?;
    {
        let mut is_first = true;
        for ext in oracle.extensions.iter().filter(|ext| ext.category == category) {
            let ident = AsIdent(&ext.short_name);
            let spec_name = &ext.spec_name;
            if !is_first {
                write!(w, "else ")?;
            }
            is_first = false;
            writeln!(w, "if name == c\"{spec_name}\" {{ self.{ident} = true; }}")?;
        }
    }
    writeln!(w, "}}")?;

    writeln!(
        w,
        "pub fn new(core_version: vk::Version) -> Self {{ Self {{ core_version,"
    )?;
    for ext in oracle.extensions.iter().filter(|ext| ext.category == category) {
        let ident = AsIdent(&ext.short_name);
        writeln!(w, "{ident}: false,")?;
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

    for ext in oracle.extensions.iter().filter(|ext| ext.affects_category(category)) {
        let ident = AsIdent(&ext.short_name);

        writeln!(w, "pub fn supports_{ident}(&self) -> bool {{")?;
        write_supports_dependency(w, oracle, ext.index, &ext.category_dependencies(category, oracle))?;
        writeln!(w, "}}")?;

        writeln!(w, "pub fn enable_{ident}(&mut self) {{")?;
        write_enable_dependency(w, oracle, ext.index, &ext.category_dependencies(category, oracle))?;
        writeln!(w, "}}")?;
    }

    writeln!(
        w,
        "pub fn to_name_vec(&self) -> Vec<&'static CStr> {{ let mut v = Vec::new();"
    )?;
    for ext in oracle.extensions.iter().filter(|ext| ext.category == category) {
        let ident = AsIdent(&ext.short_name);
        let spec_name = &ext.spec_name;
        writeln!(w, "if self.{ident} {{ v.push(c\"{spec_name}\"); }}")?;
    }
    writeln!(w, "v }}")?;

    writeln!(w, "}}")?;
    Ok(())
}

fn write_builders(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    for ty in oracle.types.iter() {
        let TypeDetail::Aggregate(aggregate_type) = &ty.detail else {
            continue;
        };
        if aggregate_type.is_union {
            continue;
        }

        let is_special_case = matches!(ty.spec_name.as_str(), "VkBaseOutStructure" | "VkBaseInStructure");
        if is_special_case {
            continue;
        }

        let is_extended = !aggregate_type.extended_by.is_empty();
        let has_pointer_member = aggregate_type
            .members
            .iter()
            .any(|member| matches!(&member.ty, TypeDecl::Pointer(_)) && member.spec_name.as_str() != "pNext");
        let needs_lifetime = has_pointer_member || is_extended;
        let needs_setters = !aggregate_type.returned_only_hint;
        let needs_builder = is_extended || needs_setters;

        let type_name = AsTypeName {
            ty,
            in_option: false,
            in_namespace: false,
        };

        if needs_builder {
            let generics = if needs_lifetime { "<'a>" } else { "" };
            let generics_or_anon = if needs_lifetime { "<'a>" } else { "<'_>" };

            writeln!(w, "\n#[repr(transparent)]")?;
            writeln!(w, "#[derive(Default)]")?;
            writeln!(w, "pub struct {type_name}Builder{generics} {{")?;
            writeln!(w, "inner: vk::{type_name},")?;
            if needs_lifetime {
                writeln!(w, "phantom: PhantomData<&'a ()>,")?;
            }
            writeln!(w, "}}")?;

            writeln!(w, "impl{generics} Builder{generics_or_anon} for vk::{type_name} {{")?;
            writeln!(w, "type Type = {type_name}Builder{generics};")?;
            writeln!(w, "fn builder() -> Self::Type {{ Default::default() }} }}")?;

            if is_extended {
                writeln!(w, "pub trait {type_name}Next {{ }}")?;
            }

            writeln!(w, "impl{generics} {type_name}Builder{generics} {{")?;
            if is_extended {
                writeln!(
                    w,
                    "pub fn insert_next<T: {type_name}Next>(mut self, next: &'a mut T) -> Self {{"
                )?;
                writeln!(
                    w,
                    "unsafe {{ insert_next(&mut self as *mut Self as *mut _, next as *mut T as *mut _); }}"
                )?;
                writeln!(w, "self }}")?;
            }
            writeln!(
                w,
                "pub fn get_mut(&mut self) -> &mut vk::{type_name} {{ &mut self.inner }}"
            )?;
            if needs_setters {
                for member in &aggregate_type.members {
                    if member.default.is_some() {
                        continue;
                    }

                    let ident = AsIdent(&member.short_name);

                    match &member.setter_transform {
                        ParameterTransform::None => {
                            let pointer_transform = PointerTransform::from_type_decl(&member.ty, false);

                            write!(w, "pub fn {ident}(mut self, {ident}: ")?;
                            match pointer_transform {
                                PointerTransform::None => {
                                    write_type_decl(
                                        w,
                                        oracle,
                                        &member.ty,
                                        TypeContext::parameter(member.is_optional).in_namespace(),
                                    )?;
                                }
                                PointerTransform::Ref => {
                                    let TypeDecl::Pointer(pointer_decl) = &member.ty else {
                                        panic!("unexpected type for slice");
                                    };
                                    if member.is_optional {
                                        write!(w, "Option<")?;
                                    }
                                    write!(w, "&'a ")?;
                                    write_type_decl(
                                        w,
                                        oracle,
                                        &pointer_decl.element_type,
                                        TypeContext::default().in_namespace(),
                                    )?;
                                    if member.is_optional {
                                        write!(w, ">")?;
                                    }
                                }
                                PointerTransform::CStr => {
                                    if member.is_optional {
                                        write!(w, "Option<&'a CStr>")?;
                                    } else {
                                        write!(w, "&'a CStr")?;
                                    }
                                }
                            }
                            writeln!(w, ") -> Self {{ self.inner.{ident} = ")?;
                            match pointer_transform {
                                PointerTransform::None => {
                                    if !member.is_optional && type_decl_is_non_null(oracle, &member.ty) {
                                        write!(w, "Some({ident})")?;
                                    } else {
                                        write!(w, "{ident}")?;
                                    }
                                }
                                PointerTransform::Ref => {
                                    if member.is_optional {
                                        let TypeDecl::Pointer(pointer_decl) = &member.ty else {
                                            panic!("unexpected type for reference");
                                        };
                                        let null = if pointer_decl.is_const { "null" } else { "null_mut" };
                                        write!(w, "{ident}.map_or(ptr::{null}(), |r| r)")?;
                                    } else {
                                        write!(w, "{ident}")?;
                                    }
                                }
                                PointerTransform::CStr => {
                                    if member.is_optional {
                                        write!(w, "{ident}.map_or(ptr::null(), |r| r.as_ptr())")?;
                                    } else {
                                        write!(w, "{ident}.as_ptr()")?;
                                    }
                                }
                            }
                            writeln!(w, "; self }}")?;
                        }
                        ParameterTransform::FromBool => {
                            writeln!(w, "pub fn {ident}(mut self, {ident}: bool) -> Self {{")?;
                            writeln!(
                                w,
                                "self.inner.{ident} = if {ident} {{ vk::TRUE }} else {{ vk::FALSE }};"
                            )?;
                            writeln!(w, "self }}")?;
                        }
                        ParameterTransform::FromMemberHandle | ParameterTransform::FromOutput => {
                            panic!("not expected for members")
                        }
                        ParameterTransform::FromSlice(_) => {}
                        ParameterTransform::FromSliceLength(transform) => {
                            let as_dest = match &member.ty {
                                TypeDecl::BuiltIn(BuiltInDecl::U32) => "as u32",
                                TypeDecl::BuiltIn(BuiltInDecl::USize) => "",
                                _ => panic!("unexpected destination type {:?} for slice length", member.ty),
                            };

                            if let Some(first_slice_index) = transform
                                .required_param
                                .first()
                                .or_else(|| transform.optional_param.first())
                                .copied()
                            {
                                let is_multi_slice =
                                    (transform.required_param.len() + transform.optional_param.len()) > 1;
                                let first_slice_member = &aggregate_type.members[first_slice_index];
                                let first_slice_ident = AsIdent(&first_slice_member.short_name);
                                writeln!(w, "pub fn {first_slice_ident}(mut self,")?;
                                for index in transform.required_param.iter().chain(transform.optional_param.iter()) {
                                    let slice_member = &aggregate_type.members[*index];
                                    let slice_ident = AsIdent(&slice_member.short_name);
                                    write!(w, "{slice_ident}: ")?;
                                    if slice_member.is_optional && is_multi_slice {
                                        write!(w, "Option<")?;
                                    }
                                    write!(w, "&'a ")?;
                                    let TypeDecl::Pointer(pointer_decl) = &slice_member.ty else {
                                        panic!("unexpected type for slice");
                                    };
                                    if !pointer_decl.is_const {
                                        write!(w, "mut ")?;
                                    }
                                    write!(w, "[")?;
                                    if matches!(
                                        pointer_decl.element_type.as_ref(),
                                        TypeDecl::BuiltIn(BuiltInDecl::Void)
                                    ) {
                                        write!(w, "u8")?;
                                    } else {
                                        write_type_decl(
                                            w,
                                            oracle,
                                            &pointer_decl.element_type,
                                            TypeContext::default().in_namespace(),
                                        )?;
                                    }
                                    write!(w, "]")?;
                                    if slice_member.is_optional && is_multi_slice {
                                        write!(w, ">")?;
                                    }
                                    writeln!(w, ",")?;
                                }
                                writeln!(w, ") -> Self {{")?;

                                write!(w, "self.inner.{ident} = ")?;
                                if first_slice_member.is_optional && is_multi_slice {
                                    for &index in transform.optional_param.iter() {
                                        if index != first_slice_index {
                                            write!(w, ".or(")?;
                                        }
                                        let other_member = &aggregate_type.members[index];
                                        let other_ident = AsIdent(&other_member.short_name);
                                        let TypeDecl::Pointer(pointer_decl) = &other_member.ty else {
                                            panic!("unexpected type for slice");
                                        };
                                        let as_ref = if pointer_decl.is_const { "" } else { ".as_ref()" };
                                        write!(w, "{other_ident}{as_ref}.map(|s| s.len() {as_dest})")?;
                                        if index != first_slice_index {
                                            write!(w, ")")?;
                                        }
                                    }
                                    writeln!(w, ".unwrap_or(0);")?;
                                } else {
                                    write!(w, "{first_slice_ident}.len() {as_dest};")?;
                                }

                                for &index in transform.required_param.iter().chain(transform.optional_param.iter()) {
                                    let slice_member = &aggregate_type.members[index];
                                    let slice_ident = AsIdent(&slice_member.short_name);
                                    if slice_member.is_optional && is_multi_slice {
                                        if transform.optional_param.len() > 1 {
                                            writeln!(w, "if let Some(len) = {slice_ident}.map(|s| s.len()) {{")?;
                                            writeln!(w, "assert_eq!(self.inner.{ident}, len {as_dest}); }}")?;
                                        }
                                    } else {
                                        if index != first_slice_index {
                                            writeln!(
                                                w,
                                                "assert_eq!(self.inner.{ident}, {slice_ident}.len() {as_dest});"
                                            )?;
                                        }
                                    }
                                }

                                for &index in transform.required_param.iter().chain(transform.optional_param.iter()) {
                                    let slice_member = &aggregate_type.members[index];
                                    let slice_ident = AsIdent(&slice_member.short_name);
                                    write!(w, "self.inner.{slice_ident} = ")?;
                                    let TypeDecl::Pointer(pointer_decl) = &slice_member.ty else {
                                        panic!("unexpected type for slice");
                                    };
                                    let as_ptr = if pointer_decl.is_const { "as_ptr" } else { "as_mut_ptr" };
                                    if slice_member.is_optional && is_multi_slice {
                                        let null = if pointer_decl.is_const { "null" } else { "null_mut" };
                                        write!(w, "{slice_ident}.map_or(ptr::{null}(), |s| s.{as_ptr}())")?;
                                    } else {
                                        write!(w, "{slice_ident}.{as_ptr}()")?;
                                    }
                                    if matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::Void)) {
                                        if pointer_decl.is_const {
                                            write!(w, " as *const _")?;
                                        } else {
                                            write!(w, " as *mut _")?;
                                        }
                                    }
                                    write!(w, ";")?;
                                }

                                writeln!(w, "self }}")?;
                            }

                            if !transform.separate_param.is_empty() {
                                write!(w, "pub fn {ident}(mut self, {ident}: ")?;
                                write_type_decl(w, oracle, &member.ty, TypeContext::default().in_namespace())?;
                                writeln!(w, ") -> Self {{")?;
                                writeln!(w, "self.inner.{ident} = {ident}; self }}")?;
                            }

                            for &slice_index in &transform.separate_param {
                                let slice_member = &aggregate_type.members[slice_index];
                                let slice_ident = AsIdent(&slice_member.short_name);
                                write!(w, "pub fn {slice_ident}(mut self, {slice_ident}: &'a ")?;
                                let TypeDecl::Pointer(pointer_decl) = &slice_member.ty else {
                                    panic!("unexpected type for slice");
                                };
                                if !pointer_decl.is_const {
                                    write!(w, "mut ")?;
                                }
                                write!(w, "[")?;
                                write_type_decl(
                                    w,
                                    oracle,
                                    &pointer_decl.element_type,
                                    TypeContext::default().in_namespace(),
                                )?;
                                writeln!(w, "],) -> Self {{")?;

                                write!(w, "self.inner.{ident} = {slice_ident}.len() {as_dest};")?;

                                let as_ptr = if pointer_decl.is_const { "as_ptr" } else { "as_mut_ptr" };
                                write!(w, "self.inner.{slice_ident} = {slice_ident}.{as_ptr}();")?;

                                writeln!(w, "self }}")?;
                            }
                        }
                    }
                }
            }
            writeln!(w, "}}")?;

            writeln!(
                w,
                "impl{generics} Deref for {type_name}Builder{generics} {{ type Target = vk::{type_name};"
            )?;
            writeln!(w, "fn deref(&self) -> &Self::Target {{ &self.inner }} }}")?;
        }

        for base_ty in aggregate_type
            .extends
            .iter()
            .map(|&type_index| &oracle.types[type_index])
        {
            let base_type_name = AsTypeName {
                ty: base_ty,
                in_option: false,
                in_namespace: false,
            };
            writeln!(w, "impl {base_type_name}Next for vk::{type_name} {{ }}")?;
            if needs_builder {
                let lifetime = if needs_lifetime { "<'_>" } else { "" };
                writeln!(w, "impl {base_type_name}Next for {type_name}Builder{lifetime} {{ }}")?;
            }
        }
    }

    Ok(())
}

fn write_vk(oracle: &Oracle) -> Res {
    let mut writer = io::BufWriter::new(File::create("spark/src/vk.rs")?);

    write_version_comment(&mut writer, oracle)?;

    writeln!(&mut writer)?;
    io::copy(&mut io::BufReader::new(File::open("input/vk_prefix.rs")?), &mut writer)?;

    write_global_constants(&mut writer, oracle)?;
    write_types(&mut writer, oracle)?;
    Ok(())
}

fn write_lib(oracle: &Oracle) -> Res {
    let mut writer = io::BufWriter::new(File::create("spark/src/lib.rs")?);

    write_version_comment(&mut writer, oracle)?;

    writeln!(&mut writer)?;
    io::copy(&mut io::BufReader::new(File::open("input/lib_prefix.rs")?), &mut writer)?;

    write_commands(&mut writer, oracle, CommandCategory::Global)?;
    write_extensions(&mut writer, oracle, ExtensionCategory::Instance)?;
    write_commands(&mut writer, oracle, CommandCategory::Instance)?;
    write_extensions(&mut writer, oracle, ExtensionCategory::Device)?;
    write_commands(&mut writer, oracle, CommandCategory::Device)?;

    Ok(())
}

fn write_builder(oracle: &Oracle) -> Res {
    let mut writer = io::BufWriter::new(File::create("spark/src/builder.rs")?);

    write_version_comment(&mut writer, oracle)?;

    writeln!(&mut writer)?;
    io::copy(
        &mut io::BufReader::new(File::open("input/builder_prefix.rs")?),
        &mut writer,
    )?;

    write_builders(&mut writer, oracle)?;

    Ok(())
}

fn main() -> Res {
    let xml_file_name = "../Vulkan-Docs/xml/vk.xml";
    let oracle = Oracle::new(xml_file_name);

    write_vk(&oracle)?;
    write_lib(&oracle)?;
    write_builder(&oracle)?;

    for file in ["vk.rs", "lib.rs", "builder.rs"] {
        process::Command::new("rustfmt")
            .arg("--edition")
            .arg("2024")
            .arg(file)
            .current_dir("spark/src")
            .output()?;
    }

    Ok(())
}
