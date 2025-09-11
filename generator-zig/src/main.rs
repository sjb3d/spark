use heck::AsSnakeCase;
use std::{
    collections::HashMap,
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
        f.write_fmt(format_args!("@\"{}\"", AsSnakeCase(self.0.as_ref())))
    }
}

struct AsPrefixedIdent<T: AsRef<str>>(&'static str, pub T);

impl<T: AsRef<str>> fmt::Display for AsPrefixedIdent<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", self.0, AsSnakeCase(self.1.as_ref())))
    }
}

struct AsPostfixedIdent<T: AsRef<str>>(pub T, &'static str);

impl<T: AsRef<str>> fmt::Display for AsPostfixedIdent<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}{}", AsSnakeCase(self.0.as_ref()), self.1))
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

struct AsTypeName<'a>(&'a Type);

impl<'a> fmt::Display for AsTypeName<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if matches!(&self.0.detail, TypeDetail::External(_)) {
            f.write_str(&self.0.spec_name)
        } else {
            let name = AsDimAwareUpperCamelCase(&self.0.short_name);
            if matches!(&self.0.detail, TypeDetail::FunctionPointer(_)) {
                f.write_fmt(format_args!("Fp{name}"))
            } else {
                f.write_fmt(format_args!("{name}"))
            }
        }
    }
}

struct AsNumber(pub Literal);

impl fmt::Display for AsNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            Literal::Int(n) => f.write_fmt(format_args!("{n}")),
            Literal::U32(n) => f.write_fmt(format_args!("0x{n:x}")),
            Literal::U64(n) => f.write_fmt(format_args!("0x{n:x}")),
            Literal::F32(n) => f.write_fmt(format_args!("{n}")),
        }
    }
}

struct AsError<T: AsRef<str>>(pub T);

impl<T: AsRef<str>> fmt::Display for AsError<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.as_ref();
        let s = s.strip_prefix("error_").unwrap_or(s);
        f.write_fmt(format_args!("{}", AsDimAwareUpperCamelCase(s)))
    }
}

fn write_version_comment(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    writeln!(
        w,
        "// Generated from vk.xml version {}.{}.{}",
        oracle.header_version.0, oracle.header_version.1, oracle.header_version.2
    )?;
    Ok(())
}

fn write_global_constants(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    for constant in oracle
        .constants
        .iter()
        .filter(|constant| constant.enum_type_index.is_none())
        .filter(|constant| !matches!(constant.spec_name.as_str(), "VK_TRUE" | "VK_FALSE"))
    {
        let ident = AsIdent(&constant.short_name);
        match &constant.value {
            ConstantValue::Literal(literal) => {
                let number = AsNumber(*literal);
                write!(w, "pub const {ident}")?;
                if let Some(type_decl) = literal.type_decl() {
                    write!(w, ": ")?;
                    write_type_decl(w, oracle, &type_decl, TypeContext::default())?;
                }
                writeln!(w, " = {number};")?;
            }
            ConstantValue::Alias(_) => {}
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, Default)]
struct TypeContext {
    is_optional_parameter: Option<bool>,
    is_external: bool,
    is_pointed_to: bool,
}

impl TypeContext {
    fn external() -> Self {
        Self {
            is_optional_parameter: None,
            is_external: true,
            is_pointed_to: false,
        }
    }

    fn parameter(is_optional: bool) -> Self {
        Self {
            is_optional_parameter: Some(is_optional),
            is_external: false,
            is_pointed_to: false,
        }
    }

    fn array_element(self) -> Self {
        Self {
            is_optional_parameter: None,
            is_external: self.is_external,
            is_pointed_to: false,
        }
    }

    fn pointee(self) -> Self {
        Self {
            is_optional_parameter: None,
            is_external: self.is_external,
            is_pointed_to: true,
        }
    }
}

fn type_decl_is_opaque(oracle: &Oracle, type_decl: &TypeDecl) -> bool {
    match type_decl {
        TypeDecl::BuiltIn(BuiltInDecl::Void) => true,
        TypeDecl::External(ExternalDecl::Opaque) => true,
        TypeDecl::Type(type_index) => match &oracle.types[*type_index].detail {
            TypeDetail::Alias(inner_decl) | TypeDetail::External(inner_decl) => type_decl_is_opaque(oracle, inner_decl),
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
                            "anyopaque"
                        } else {
                            "void"
                        },
                    BuiltInDecl::Char => "u8",
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
            let type_info = &oracle.types[*type_index];
            if matches!(type_info.detail, TypeDetail::FunctionPointer(_)) {
                write!(w, "?")?;
            }
            let type_name = AsTypeName(&oracle.types[*type_index]);
            write!(w, "{type_name}")?;
        }
        TypeDecl::Array(array_decl) => {
            write!(w, "[")?;
            match &array_decl.array.size {
                ArraySize::Unknown | ArraySize::Named(_) => {
                    panic!("cannot declare array of uknown length");
                }
                ArraySize::Literal(value) => {
                    let number = AsNumber(*value);
                    write!(w, "{number}")?;
                }
                ArraySize::Constant(constant_index) => {
                    let ident = AsIdent(&oracle.constants[*constant_index].short_name);
                    write!(w, "{ident}")?;
                }
            }
            if array_decl.array.is_null_terminated && !context.is_external {
                write!(w, " - 1: 0")?;
            }
            write!(w, "]")?;
            write_type_decl(w, oracle, &array_decl.element_type, context.array_element())?;
        }
        TypeDecl::Pointer(pointer_decl) => {
            let is_opaque = type_decl_is_opaque(oracle, &pointer_decl.element_type);
            if context.is_external {
                if is_opaque {
                    write!(w, "?*")?;
                } else {
                    write!(w, "[*c]")?;
                }
            } else {
                if context.is_optional_parameter.unwrap_or(true) {
                    write!(w, "?")?;
                }
                if let Some(array) = pointer_decl.array_hint.as_ref().filter(|_| !is_opaque) {
                    if array.is_null_terminated {
                        write!(w, "[*:0]")?;
                    } else {
                        write!(w, "[*]")?;
                    }
                } else {
                    write!(w, "*")?;
                }
            };
            if pointer_decl.is_const {
                write!(w, "const ")?;
            }
            write_type_decl(w, oracle, &pointer_decl.element_type, context.pointee())?;
        }
        TypeDecl::External(opaque_decl) => match opaque_decl {
            ExternalDecl::Opaque => {
                if context.is_pointed_to {
                    write!(w, "anyopaque")?
                } else {
                    write!(w, "opaque {{}}")?
                }
            }
            ExternalDecl::CULong => write!(w, "c_ulong")?,
        },
    }
    Ok(())
}

fn type_decl_default(oracle: &Oracle, decl: &TypeDecl) -> Option<String> {
    match decl {
        TypeDecl::BuiltIn(built_in) => match built_in {
            BuiltInDecl::Void => None,
            _ => Some("0".to_owned()),
        },
        TypeDecl::Array(array_decl) => {
            let mut element_type = Vec::new();
            write_type_decl(
                &mut element_type,
                oracle,
                &array_decl.element_type,
                TypeContext::default(),
            )
            .unwrap();
            let element_type = String::from_utf8(element_type).unwrap();

            let element_default = type_decl_default(oracle, &array_decl.element_type)?;
            let count = match &array_decl.array.size {
                ArraySize::Unknown | ArraySize::Named(_) => {
                    panic!("cannot set default for array of unknown length")
                }
                ArraySize::Literal(value) => {
                    let number = AsNumber(*value);
                    format!("{number}")
                }
                ArraySize::Constant(constant_index) => {
                    let ident = AsIdent(&oracle.constants[*constant_index].short_name);
                    format!("{ident}")
                }
            };

            if array_decl.array.is_null_terminated {
                Some(format!("[_:0]{element_type}{{ {element_default} }} ** ({count} - 1)"))
            } else {
                Some(format!("[_]{element_type}{{ {element_default} }} ** {count}"))
            }
        }
        TypeDecl::Pointer(_) => Some("null".to_owned()),
        TypeDecl::Type(type_index) => type_default(oracle, *type_index),
        TypeDecl::External(opaque_decl) => match opaque_decl {
            ExternalDecl::Opaque => None,
            ExternalDecl::CULong => Some("0".to_owned()),
        },
    }
}

fn type_default(oracle: &Oracle, type_index: TypeIndex) -> Option<String> {
    let type_info = &oracle.types[type_index];
    match &type_info.detail {
        TypeDetail::Aggregate(aggregate_type) => {
            if aggregate_type.is_union {
                let member = aggregate_type.members.first()?;
                let ident = AsIdent(&member.short_name);
                let member_default = type_decl_default(oracle, &member.ty)?;
                Some(format!(".{{ .{ident} = {member_default} }}"))
            } else {
                Some(".{}".to_owned())
            }
        }
        TypeDetail::Alias(decl) | TypeDetail::External(decl) => match type_info.spec_name.as_str() {
            "Version" => Some("Version.from_int(0)".to_owned()),
            "VkBool32" => Some(".@\"false\"".to_owned()),
            _ => type_decl_default(oracle, decl),
        },
        TypeDetail::Enum(enum_type) => {
            if enum_type.bitmask_width.is_some() {
                Some(".none".to_owned())
            } else {
                Some("@enumFromInt(0)".to_owned())
            }
        }
        TypeDetail::FunctionPointer(_) => Some("null".to_owned()),
        TypeDetail::Handle(_) => Some(".null_handle".to_owned()),
    }
}

fn write_types(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    for ty in oracle
        .types
        .iter()
        .filter(|ty| !matches!(ty.spec_name.as_str(), "VkBool32" | "Version"))
    {
        let type_name = AsTypeName(ty);
        match &ty.detail {
            TypeDetail::Alias(type_decl) | TypeDetail::External(type_decl) => {
                write!(w, "pub const {type_name} = ")?;
                write_type_decl(w, oracle, type_decl, TypeContext::default())?;
                writeln!(w, ";")?;
            }
            TypeDetail::Enum(enum_type) => {
                if let Some(bitmask_width) = enum_type.bitmask_width {
                    let field_bit_count = match bitmask_width {
                        BitWidth::U32 => 5,
                        BitWidth::U64 => 6,
                    };
                    let bits_short_name = ty.short_name.replacen("Flags", "FlagBits", 1);
                    let bits_type_name = AsDimAwareUpperCamelCase(bits_short_name);
                    writeln!(w, "pub const {bits_type_name} = enum(u{field_bit_count}) {{")?;
                    let mut multi_bit_constants = Vec::new();
                    for constant in enum_type.values.iter().map(|&index| &oracle.constants[index]) {
                        if let ConstantValue::Literal(literal) = constant.value {
                            if let Some(bit_index) = match literal {
                                Literal::U32(n) => {
                                    let tz = n.trailing_zeros();
                                    (tz < 32 && (1 << tz) == n).then_some(tz)
                                }
                                Literal::U64(n) => {
                                    let tz = n.trailing_zeros();
                                    (tz < 64 && (1 << tz) == n).then_some(tz)
                                }
                                Literal::Int(_) | Literal::F32(_) => None,
                            } {
                                let ident = AsIdent(&constant.short_name);
                                writeln!(w, "{ident} = {bit_index},")?;
                            } else {
                                multi_bit_constants.push(constant);
                            }
                        }
                    }
                    writeln!(w, "_, }};")?;
                    writeln!(w, "pub const {type_name} = BitField({bits_type_name});")?;
                    if !multi_bit_constants.is_empty() {
                        let masks_short_name = ty.short_name.replacen("Flags", "FlagMasks", 1);
                        let masks_type_name = AsDimAwareUpperCamelCase(masks_short_name);
                        writeln!(w, "pub const {masks_type_name} = struct {{")?;
                        for constant in multi_bit_constants.drain(..) {
                            let ident = AsIdent(&constant.short_name);
                            let value = match constant.value {
                                ConstantValue::Literal(Literal::U32(n)) => n as u64,
                                ConstantValue::Literal(Literal::U64(n)) => n,
                                _ => {
                                    panic!("unsupported bitmask constant type")
                                }
                            };
                            writeln!(w, "pub const {ident} = {type_name} {{ .bits = 0x{value:x} }};")?;
                        }
                        writeln!(w, "}};")?;
                    }
                } else {
                    writeln!(w, "pub const {type_name} = enum(i32) {{")?;
                    for constant in enum_type.values.iter().map(|&index| &oracle.constants[index]) {
                        if let ConstantValue::Literal(literal) = constant.value {
                            let ident = AsIdent(&constant.short_name);
                            let value = AsNumber(literal);
                            writeln!(w, "{ident} = {value},")?;
                        }
                    }
                    writeln!(w, "_, }};")?;
                }
            }
            TypeDetail::Aggregate(aggregate_type) => {
                let agg_type = if aggregate_type.is_union { "union" } else { "struct" };
                writeln!(w, "pub const {type_name} = extern {agg_type} {{")?;
                for member in &aggregate_type.members {
                    let ident = AsIdent(&member.short_name);
                    write!(w, "{ident}: ")?;
                    write_type_decl(w, oracle, &member.ty, TypeContext::default())?;
                    if !aggregate_type.is_union {
                        if let Some(constant_index) = member.default {
                            let constant = &oracle.constants[constant_index];
                            if constant.enum_type_index.is_none() {
                                panic!("expected enum value default");
                            }
                            let ident = AsIdent(&constant.short_name);
                            write!(w, " = .{ident}")?;
                        } else if let Some(default) = type_decl_default(oracle, &member.ty) {
                            write!(w, " = {default}")?;
                        }
                    }
                    writeln!(w, ",")?;
                }
                if !aggregate_type.extended_by.is_empty() {
                    writeln!(w, "const Self = @This();")?;
                    writeln!(w, "pub fn insert_next(self: *Self, next: anytype) void {{")?;
                    writeln!(w, "switch (@TypeOf(next)) {{ inline ")?;
                    for &extend_type_index in &aggregate_type.extended_by {
                        let extend_type = &oracle.types[extend_type_index];
                        let extend_type_name = AsTypeName(extend_type);
                        writeln!(w, "*{extend_type_name},")?;
                    }
                    writeln!(
                        w,
                        " => {{ next.p_next = @constCast(self.p_next); self.p_next = next; }},"
                    )?;
                    writeln!(w, "else => @compileError(\"invalid extension struct type\"),")?;
                    writeln!(w, "}} }}")?;
                }
                writeln!(w, "}};")?;
            }
            TypeDetail::Handle(handle_type) => {
                let enum_type = match handle_type {
                    HandleType::USize => "usize",
                    HandleType::U64 => "u64",
                };
                writeln!(w, "pub const {type_name} = enum({enum_type}) {{ null_handle = 0, _ }};")?;
            }
            TypeDetail::FunctionPointer(function_pointer_type) => {
                writeln!(w, "pub const {type_name} = *const fn(")?;
                let mut needs_separator = false;
                for parameter in &function_pointer_type.parameters {
                    if needs_separator {
                        write!(w, ", ")?;
                    } else {
                        needs_separator = true;
                    }
                    write_type_decl(w, oracle, &parameter.ty, TypeContext::external())?;
                }
                write!(w, ") callconv(.c) ")?;
                write_type_decl(w, oracle, &function_pointer_type.return_type, TypeContext::external())?;
                writeln!(w, ";")?;
            }
        }
    }
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
                "self.core_version.to_int() >= make_version({}, {}, 0).to_int()",
                v.major, v.minor
            )?;
        }
        ExtensionDependencyExpr::Extension(index) => {
            let ext = &oracle.extensions[*index];
            if *index == current_index {
                let ident = AsIdent(&ext.short_name);
                write!(w, "self.{ident}")?;
            } else {
                let supports = AsPrefixedIdent("supports_", &ext.short_name);
                write!(w, "self.{supports}()")?;
            }
        }
        ExtensionDependencyExpr::Feature => {
            unimplemented!("feature dependencies not implement yet");
        }
        ExtensionDependencyExpr::And(v) => {
            for (i, dep) in v.iter().enumerate() {
                if i != 0 {
                    write!(w, " and ")?;
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
                    write!(w, " or ")?;
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
                 assert(self.core_version.to_int() >= make_version({}, {}, 0).to_int());",
                v.major, v.minor
            )?;
        }
        ExtensionDependencyExpr::Extension(index) => {
            let ext = &oracle.extensions[*index];
            if *index == current_index {
                let ident = AsIdent(&ext.short_name);
                writeln!(w, "self.{ident} = true;")?;
            } else {
                let enable = AsPrefixedIdent("enable_", &ext.short_name);
                writeln!(w, "self.{enable}();")?;
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
                            "if (self.core_version.to_int() < make_version({}, {}, 0).to_int()) {{",
                            v.major, v.minor
                        )?;
                        write_enable_dependency(w, oracle, current_index, other)?;
                        writeln!(w, "}}")?;
                    }
                    (ExtensionDependencyExpr::Extension(index_a), ExtensionDependencyExpr::Extension(index_b)) => {
                        let ext_a = &oracle.extensions[*index_a];
                        let ext_b = &oracle.extensions[*index_b];
                        let supports_a = AsPrefixedIdent("supports_", &ext_a.short_name);
                        let supports_b = AsPrefixedIdent("supports_", &ext_b.short_name);
                        writeln!(
                            w,
                            "// ambiguous dependency, caller must enable one or the other\n\
                             assert(self.{supports_a}() or self.{supports_b}());"
                        )?;
                    }
                    _ => unimplemented!(),
                }
            } else {
                println!("{deps:?}");
                unimplemented!();
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
                "extensions.core_version.to_int() >= make_version({}, {}, 0).to_int()",
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
                    write!(w, " and ")?;
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
                    write!(w, " or ")?;
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

fn write_extension_names(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    writeln!(w, "\nconst ExtensionNames = struct {{")?;
    for ext in &oracle.extensions {
        let spec_name = &ext.spec_name;
        let ident = AsIdent(&ext.short_name);
        writeln!(w, "const {ident} = \"{spec_name}\";")?;
    }
    writeln!(w, "}};")?;

    Ok(())
}

fn write_extensions(w: &mut impl IoWrite, oracle: &Oracle, category: ExtensionCategory) -> Res {
    let struct_name = match category {
        ExtensionCategory::Instance => "InstanceExtensions",
        ExtensionCategory::Device => "DeviceExtensions",
    };
    writeln!(w, "\npub const {struct_name} = packed struct {{ core_version: Version,")?;

    for ext in oracle.extensions.iter().filter(|ext| ext.category == category) {
        let ident = AsIdent(&ext.short_name);
        writeln!(w, "{ident}: bool = false,")?;
    }

    writeln!(
        w,
        "\npub fn enable_by_name(self: *{struct_name}, maybe_name: ?[*:0]const u8) void {{"
    )?;
    writeln!(w, "const name = maybe_name orelse return;")?;
    let mut cond = "if";
    for ext in oracle.extensions.iter().filter(|ext| ext.category == category) {
        let ident = AsIdent(&ext.short_name);
        writeln!(w, "{cond} (std.mem.orderZ(u8, name, ExtensionNames.{ident}) == .eq) {{")?;
        writeln!(w, "self.{ident} = true; }}")?;
        cond = "else if";
    }
    writeln!(w, "}}")?;

    writeln!(
        w,
        "\npub fn from_properties(instance_version: Version, properties: []const ExtensionProperties) {struct_name} {{"
    )?;
    writeln!(w, "var self: {struct_name} = .{{ .core_version = instance_version, }};")?;
    writeln!(
        w,
        "for (properties) |*prop| {{ self.enable_by_name(&prop.extension_name); }}"
    )?;
    writeln!(w, "return self; }}")?;

    writeln!(
        w,
        "\npub fn to_name_array(self: {struct_name}, allocator: Allocator) Allocator.Error![][*:0]const u8 {{"
    )?;
    writeln!(w, "var names = std.ArrayListUnmanaged([*:0]const u8) {{ }};")?;
    for ext in oracle.extensions.iter().filter(|ext| ext.category == category) {
        let ident = AsIdent(&ext.short_name);
        writeln!(
            w,
            "if (self.{ident}) try names.append(allocator, ExtensionNames.{ident});"
        )?;
    }
    writeln!(w, "return names.toOwnedSlice(allocator);")?;
    writeln!(w, "}}")?;

    for ext in oracle.extensions.iter().filter(|ext| ext.affects_category(category)) {
        let supports = AsPrefixedIdent("supports_", &ext.short_name);
        let enable = AsPrefixedIdent("enable_", &ext.short_name);

        writeln!(w, "\npub fn {supports}(self: {struct_name}) bool {{ return")?;
        write_supports_dependency(w, oracle, ext.index, &ext.category_dependencies(category, oracle))?;
        writeln!(w, "; }}")?;

        writeln!(w, "pub fn {enable}(self: *{struct_name}) void {{")?;
        write_enable_dependency(w, oracle, ext.index, &ext.category_dependencies(category, oracle))?;
        writeln!(w, "}}")?;
    }

    writeln!(w, "}};")?;
    Ok(())
}

fn write_command_param_declaration(w: &mut impl IoWrite, oracle: &Oracle, param: &ParameterDecl) -> Res {
    let ident = AsIdent(&param.name);
    match &param.transform {
        ParameterTransform::None => {
            write!(w, "{ident}: ")?;
            write_type_decl(w, oracle, &param.ty, TypeContext::parameter(param.is_optional))?;
            writeln!(w, ",")?;
        }
        ParameterTransform::FromBool => {
            write!(w, "{ident}: bool,")?;
        }
        ParameterTransform::FromMemberHandle => {}
        ParameterTransform::FromSlice(_) => {
            let TypeDecl::Pointer(pointer_decl) = &param.ty else {
                panic!("unexpected type for slice");
            };
            let opt = if param.is_optional { "?" } else { "" };
            let con = if pointer_decl.is_const { "const " } else { "" };
            write!(w, "{ident}: {opt}[]{con}")?;
            if matches!(pointer_decl.element_type.as_ref(), TypeDecl::BuiltIn(BuiltInDecl::Void)) {
                write!(w, "u8")?;
            } else {
                write_type_decl(w, oracle, &pointer_decl.element_type, TypeContext::default())?;
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
                    writeln!(w, "if ({ident}) |s| {{ assert(s.len == {length_check}); }}")?;
                } else {
                    writeln!(w, "assert({ident}.len == {length_check});")?;
                }
            }
        }
        ParameterTransform::FromSliceLength(transform) => {
            let ident = AsIdent(&param.name);
            let mut is_first = true;
            for &param_index in &transform.required_param {
                let slice_ident = AsIdent(&fp_type.parameters[param_index].name);
                if is_first {
                    write!(w, "const {ident}: ")?;
                    write_type_decl(w, oracle, &param.ty, TypeContext::parameter(param.is_optional))?;
                    write!(w, " = @intCast({slice_ident}.len);")?;
                    is_first = false;
                } else {
                    writeln!(w, "assert({ident} == {slice_ident}.len);")?;
                }
            }
            for &param_index in &transform.optional_param {
                let slice_ident = AsIdent(&fp_type.parameters[param_index].name);
                writeln!(w, "if ({slice_ident}) |s| {{ assert({ident} == s.len); }}")?;
            }
        }
        ParameterTransform::FromOutput => {
            let ident = AsIdent(&param.name);
            let TypeDecl::Pointer(pointer_decl) = &param.ty else {
                panic!("expected pointer type for output");
            };
            write!(w, "var {ident}: ")?;
            write_type_decl(w, oracle, &pointer_decl.element_type, TypeContext::default())?;
            writeln!(w, " = undefined;")?;
        }
        _ => {}
    }
    Ok(())
}

fn write_command_param_forward(w: &mut impl IoWrite, param: &ParameterDecl) -> Res {
    let ident = AsIdent(&param.name);
    match &param.transform {
        ParameterTransform::None => {
            write!(w, "{ident}")?;
        }
        ParameterTransform::FromBool => {
            write!(w, "Bool32.from_bool({ident})")?;
        }
        ParameterTransform::FromMemberHandle => {
            write!(w, "self.handle")?;
        }
        ParameterTransform::FromSlice(_) => {
            if param.is_optional {
                write!(w, "if ({ident}) |slice| slice.ptr orelse null")?;
            } else {
                write!(w, "{ident}.ptr")?;
            }
        }
        ParameterTransform::FromSliceLength { .. } => {
            write!(w, "{ident}")?;
        }
        ParameterTransform::FromOutput => {
            write!(w, "&{ident}")?;
        }
    }
    Ok(())
}

fn write_commands(w: &mut impl IoWrite, oracle: &Oracle, category: CommandCategory) -> Res {
    let struct_name = match category {
        CommandCategory::Global => "GlobalCommands",
        CommandCategory::Instance => "InstanceCommands",
        CommandCategory::Device => "DeviceCommands",
    };
    writeln!(w, "\npub const {struct_name} = struct {{")?;

    match category {
        CommandCategory::Global => {}
        CommandCategory::Instance => {
            writeln!(w, "handle: Instance, extensions: InstanceExtensions,")?;
        }
        CommandCategory::Device => {
            writeln!(w, "handle: Device, extensions: DeviceExtensions,")?;
        }
    }

    for cmd in oracle.commands.iter().filter(|cmd| cmd.category == category) {
        if let CommandDetail::Function(cmd_func) = &cmd.detail {
            let fp_name = AsPrefixedIdent("fp_", &cmd.short_name);
            let type_name = AsTypeName(&oracle.types[cmd_func.function_type_index]);
            let opt = if cmd.dependencies.is_always() { "" } else { "?" };
            writeln!(w, "{fp_name}: {opt}{type_name},")?;
        }
    }

    match category {
        CommandCategory::Global => {
            writeln!(
                w,
                "\npub fn init(fp_get_instance_proc_addr: FpGetInstanceProcAddr) MissingFunctionError!{struct_name} {{"
            )?;
            writeln!(w, "return .{{")?;
        }
        CommandCategory::Instance => {
            writeln!(w, "\npub fn init(globals: GlobalCommands, instance: Instance, create_info: *const InstanceCreateInfo) MissingFunctionError!{struct_name} {{")?;
            writeln!(
                w,
                "var extensions: InstanceExtensions = .{{ .core_version = make_version(1, 0, 0), }};"
            )?;
            writeln!(
                w,
                "if (create_info.p_application_info) |app_info| {{ extensions.core_version = app_info.api_version; }}"
            )?;
            writeln!(w, "if (create_info.pp_enabled_extension_names) |extension_names| {{ for (0..create_info.enabled_extension_count) |i| {{ extensions.enable_by_name(extension_names[i]); }} }}")?;
            writeln!(w, "return .{{ .handle = instance, .extensions = extensions,")?;
        }
        CommandCategory::Device => {
            writeln!(w, "\npub fn init(globals: GlobalCommands, instance: InstanceCommands, device: Device, create_info: *const DeviceCreateInfo) MissingFunctionError!{struct_name} {{")?;
            writeln!(
                w,
                "var extensions: DeviceExtensions = .{{ .core_version = instance.extensions.core_version, }};"
            )?;
            writeln!(w, "if (create_info.pp_enabled_extension_names) |extension_names| {{ for (0..create_info.enabled_extension_count) |i| {{ extensions.enable_by_name(extension_names[i]); }} }}")?;
            writeln!(w, "return .{{ .handle = device, .extensions = extensions,")?;
        }
    }
    for (cmd, cmd_func) in oracle.commands.iter().filter_map(|cmd| match &cmd.detail {
        CommandDetail::Function(cmd_func) if cmd.category == category => Some((cmd, cmd_func)),
        _ => None,
    }) {
        let fp_name = AsPrefixedIdent("fp_", &cmd.short_name);
        write!(w, ".{fp_name} = ")?;

        let loader = match category {
            CommandCategory::Global => {
                match cmd.spec_name.as_str() {
                    "vkGetInstanceProcAddr" => {
                        // special case: copy loader entry point
                        writeln!(w, "fp_get_instance_proc_addr,")?;
                        continue;
                    }
                    "vkEnumerateInstanceVersion" => {
                        // special case: always try to load, allow null if not present
                        writeln!(
                            w,
                            "@ptrCast(fp_get_instance_proc_addr(.null_handle, \"vkEnumerateInstanceVersion\")),"
                        )?;
                        continue;
                    }
                    _ => "get_proc_addr(fp_get_instance_proc_addr, ",
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

        if cmd.dependencies.is_always() {
            let spec_name = &cmd.spec_name;
            writeln!(w, "@ptrCast(try {loader}\"{spec_name}\")),")?;
        } else {
            for alt_cmd in iter::once(cmd).chain(cmd_func.aliases.iter().map(|&index| &oracle.commands[index])) {
                let spec_name = &alt_cmd.spec_name;
                write!(w, "if (")?;
                write_check_dependency(w, oracle, category, &alt_cmd.dependencies)?;
                writeln!(w, ") @ptrCast(try {loader}\"{spec_name}\")) else ")?;
            }
            writeln!(w, "null,")?;
        }
    }
    writeln!(w, "}}; }}")?;

    if let Some(impl_file_name) = match category {
        CommandCategory::Global => Some("input/global_impl.zig"),
        CommandCategory::Instance => Some("input/instance_impl.zig"),
        CommandCategory::Device => None,
    } {
        io::copy(&mut io::BufReader::new(File::open(impl_file_name)?), w)?;
    }

    let mut success_type_names: HashMap<Vec<ConstantIndex>, String> = HashMap::new();
    success_type_names.insert(
        oracle
            .constants
            .iter()
            .filter_map(|constant| {
                matches!(constant.spec_name.as_str(), "VK_SUCCESS" | "VK_INCOMPLETE").then_some(constant.index)
            })
            .collect(),
        "EnumerateResult".to_owned(),
    );

    for (cmd, cmd_func) in oracle.commands.iter().filter_map(|cmd| match &cmd.detail {
        CommandDetail::Function(cmd_func) if cmd.category == category => Some((cmd, cmd_func)),
        _ => None,
    }) {
        let is_special_case = matches!(
            cmd.spec_name.as_str(),
            "vkGetInstanceProcAddr" | "vkEnumerateInstanceVersion" | "vkGetDeviceProcAddr"
        );
        if is_special_case {
            continue;
        }

        let function_name = AsIdent(&cmd.short_name);
        let result_prefix = AsDimAwareUpperCamelCase(&cmd.short_name);
        let TypeDetail::FunctionPointer(fp_type) = &oracle.types[cmd_func.function_type_index].detail else {
            panic!("expected function pointer type");
        };

        if matches!(cmd_func.output_transform, CommandOutputTransform::None)
            && cmd_func.success_codes.len() > 1
            && !success_type_names.contains_key(&cmd_func.success_codes)
        {
            writeln!(w, "pub const {result_prefix}Result = enum {{")?;
            for &index in &cmd_func.success_codes {
                let ident = AsIdent(&oracle.constants[index].short_name);
                write!(w, "{ident},")?;
            }
            writeln!(w, "}};")?;

            success_type_names.insert(cmd_func.success_codes.clone(), format!("{result_prefix}Result"));
        }

        let has_error_set = !cmd_func.error_codes.is_empty() || !cmd_func.success_codes.is_empty();
        if has_error_set {
            writeln!(w, "pub const {result_prefix}Error = error {{")?;
            for &index in &cmd_func.error_codes {
                let error = AsError(&oracle.constants[index].short_name);
                write!(w, "{error},")?;
            }
            write!(w, "Unexpected,")?;
            writeln!(w, "}};")?;
        }

        writeln!(w, "pub fn {function_name}(self: {struct_name}, ")?;
        for param in &fp_type.parameters {
            write_command_param_declaration(w, oracle, param)?;
        }
        write!(w, ") ")?;
        if has_error_set {
            write!(w, "{result_prefix}Error!")?;
        }
        match &cmd_func.output_transform {
            CommandOutputTransform::None => {
                if cmd_func.success_codes.len() > 1 {
                    let success_type_name = success_type_names.get(&cmd_func.success_codes).unwrap();
                    write!(w, "{success_type_name}")?;
                } else if cmd_func.success_codes.len() == 1 {
                    write!(w, "void")?;
                } else {
                    write_type_decl(w, oracle, &fp_type.return_type, TypeContext::default())?;
                }
            }
            CommandOutputTransform::IntoObject(transform) => {
                if transform.into_bool {
                    write!(w, "bool")?;
                } else {
                    let TypeDecl::Pointer(pointer_decl) = &fp_type.parameters[transform.output_param].ty else {
                        panic!("expected pointer type for output");
                    };
                    write_type_decl(w, oracle, &pointer_decl.element_type, TypeContext::default())?;
                }
            }
            CommandOutputTransform::IntoEnumerate(_) => {
                if cmd_func.success_codes.is_empty() {
                    write!(w, "void")?;
                } else {
                    write!(w, "EnumerateResult")?;
                }
            }
            CommandOutputTransform::IntoBool => {
                write!(w, "bool")?;
            }
        }
        writeln!(w, " {{")?;

        for param in &fp_type.parameters {
            write_command_param_body(w, oracle, param, fp_type)?;
        }

        match &cmd_func.output_transform {
            CommandOutputTransform::IntoBool => write!(w, "return ")?,
            _ => {
                if !cmd_func.success_codes.is_empty() {
                    write!(w, "switch (")?
                }
            }
        }

        let fp_name = AsPrefixedIdent("fp_", &cmd.short_name);
        write!(w, "self.{fp_name}")?;
        if !cmd.dependencies.is_always() {
            write!(w, ".?")?;
        }
        write!(w, "(")?;
        let mut is_first = true;
        for param in &fp_type.parameters {
            if is_first {
                is_first = false;
            } else {
                write!(w, ", ")?;
            }
            write_command_param_forward(w, param)?;
        }
        write!(w, ")")?;

        match &cmd_func.output_transform {
            CommandOutputTransform::None => {
                if cmd_func.success_codes.is_empty() {
                    writeln!(w, ";")?;
                } else {
                    writeln!(w, ") {{")?;
                    for &index in &cmd_func.success_codes {
                        let ident = AsIdent(&oracle.constants[index].short_name);
                        if cmd_func.success_codes.len() > 1 {
                            write!(w, ".{ident} => return .{ident},")?;
                        } else {
                            write!(w, ".{ident} => return,")?;
                        }
                    }
                    for &index in &cmd_func.error_codes {
                        let ident = AsIdent(&oracle.constants[index].short_name);
                        let error = AsError(&oracle.constants[index].short_name);
                        write!(w, ".{ident} => return error.{error},")?;
                    }
                    writeln!(w, "else => return error.Unexpected,")?;
                    writeln!(w, "}}")?;
                }
            }
            CommandOutputTransform::IntoObject(transform) => {
                let postfix = if transform.into_bool { ".to_bool()" } else { "" };
                let param_ident = AsIdent(&fp_type.parameters[transform.output_param].name);
                if cmd_func.success_codes.is_empty() {
                    writeln!(w, "; return {param_ident}{postfix};")?
                } else {
                    writeln!(w, ") {{")?;
                    for (counter, &index) in cmd_func.success_codes.iter().enumerate() {
                        let ident = AsIdent(&oracle.constants[index].short_name);
                        if counter > 0 {
                            write!(w, ", ")?;
                        }
                        write!(w, ".{ident}")?;
                    }
                    writeln!(w, " => return {param_ident}{postfix},")?;
                    for &index in &cmd_func.error_codes {
                        let ident = AsIdent(&oracle.constants[index].short_name);
                        let error = AsError(&oracle.constants[index].short_name);
                        write!(w, ".{ident} => return error.{error},")?;
                    }
                    writeln!(w, "else => return error.Unexpected,")?;
                    writeln!(w, "}}")?;
                }
            }
            CommandOutputTransform::IntoEnumerate(_) => {
                if cmd_func.success_codes.is_empty() {
                    writeln!(w, ";")?;
                } else {
                    writeln!(w, ") {{")?;
                    writeln!(w, ".success => return .success,")?;
                    writeln!(w, ".incomplete => return .incomplete,")?;
                    for &index in &cmd_func.error_codes {
                        let ident = AsIdent(&oracle.constants[index].short_name);
                        let error = AsError(&oracle.constants[index].short_name);
                        write!(w, ".{ident} => return error.{error},")?;
                    }
                    writeln!(w, "else => return error.Unexpected,")?;
                    writeln!(w, "}}")?;
                }
            }
            CommandOutputTransform::IntoBool => {
                writeln!(w, ".to_bool();")?;
            }
        }
        writeln!(w, "}}")?;

        if let CommandOutputTransform::IntoEnumerate(transform) = &cmd_func.output_transform {
            let element_decl = match &fp_type.parameters[transform.elements_param].ty {
                TypeDecl::Pointer(pointer_decl) => pointer_decl.element_type.as_ref(),
                _ => panic!("expected pointer type for elements"),
            };

            if !cmd_func.error_codes.is_empty() {
                writeln!(
                    w,
                    "pub const {result_prefix}OrAllocatorError = {result_prefix}Error || Allocator.Error;"
                )?;
            }

            let to_array_name = AsPostfixedIdent(&cmd.short_name, "_to_array");
            writeln!(w, "pub fn {to_array_name}(self: {struct_name}, allocator: Allocator, ")?;
            for (_, param) in fp_type
                .parameters
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != transform.count_param && index != transform.elements_param)
            {
                write_command_param_declaration(w, oracle, param)?;
            }
            if cmd_func.error_codes.is_empty() {
                write!(w, ") Allocator.Error![]")?;
            } else {
                write!(w, ") {result_prefix}OrAllocatorError![]")?;
            }
            write_type_decl(w, oracle, element_decl, TypeContext::default())?;
            writeln!(w, "{{")?;

            writeln!(w, "const enumerator = struct {{ self: *const {struct_name},")?;
            for (_, param) in fp_type
                .parameters
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != transform.count_param && index != transform.elements_param)
            {
                write_command_param_declaration(w, oracle, param)?;
            }

            write!(w, "pub fn enumerate(enumerator: @This(), len: *u32, elements: ?[*]")?;
            write_type_decl(w, oracle, element_decl, TypeContext::default())?;
            let ret_type = if cmd_func.success_codes.is_empty() {
                "void"
            } else {
                "!EnumerateResult"
            };
            writeln!(w, ") {ret_type} {{")?;

            write!(w, "return enumerator.self.{function_name}(")?;
            for (index, param) in fp_type.parameters.iter().enumerate() {
                if index == transform.count_param {
                    write!(w, "len,")?;
                } else if index == transform.elements_param {
                    write!(w, "elements,")?;
                } else {
                    let ident = AsIdent(&param.name);
                    match &param.transform {
                        ParameterTransform::None | ParameterTransform::FromBool | ParameterTransform::FromSlice(_) => {
                            write!(w, "enumerator.{ident},")?
                        }
                        ParameterTransform::FromMemberHandle
                        | ParameterTransform::FromSliceLength(_)
                        | ParameterTransform::FromOutput => {}
                    }
                }
            }
            writeln!(w, "); }}")?;

            writeln!(w, "}} {{ .self = &self,")?;
            for (_, param) in fp_type
                .parameters
                .iter()
                .enumerate()
                .filter(|&(index, _)| index != transform.count_param && index != transform.elements_param)
            {
                let ident = AsIdent(&param.name);
                if matches!(
                    param.transform,
                    ParameterTransform::None | ParameterTransform::FromBool | ParameterTransform::FromSlice(_)
                ) {
                    writeln!(w, ".{ident} = {ident},")?;
                }
            }
            writeln!(w, "}};")?;

            if cmd_func.success_codes.is_empty() {
                write!(w, "return enumerate_generic_unchecked_to_array(")?;
            } else {
                write!(w, "return enumerate_generic_to_array({result_prefix}OrAllocatorError,")?;
            };
            write_type_decl(w, oracle, element_decl, TypeContext::default())?;
            writeln!(w, ", enumerator, allocator);")?;

            writeln!(w, "}}")?;
        }
    }

    writeln!(w, "}};")?;
    Ok(())
}

fn write_tests(w: &mut impl IoWrite, oracle: &Oracle) -> Res {
    writeln!(w, "test {{")?;
    writeln!(w, "const c = @cImport({{ @cInclude(\"vulkan/vulkan.h\"); }});")?;
    writeln!(w, "const expectEqual = std.testing.expectEqual;")?;

    writeln!(w, "const major = 1;")?;
    writeln!(w, "const minor = 3;")?;
    writeln!(w, "const patch = 99;")?;
    writeln!(w, "const v = make_version(major, minor, patch);")?;
    writeln!(
        w,
        "try expectEqual(v.to_int(), c.VK_MAKE_VERSION(major, minor, patch));"
    )?;
    writeln!(w, "try expectEqual(v.major, major);")?;
    writeln!(w, "try expectEqual(v.minor, minor);")?;
    writeln!(w, "try expectEqual(v.patch, patch);")?;

    for ty in oracle.types.iter() {
        let spec_name = &ty.spec_name;
        let type_name = AsTypeName(ty);
        if match &ty.detail {
            TypeDetail::Aggregate(_) => {
                // skip structures with bitfields for now (emitted as opaque by cImport)
                !(spec_name.contains("AccelerationStructure") && spec_name.contains("Instance"))
            }
            TypeDetail::Enum(enum_type) => {
                if enum_type.bitmask_width.is_none() {
                    for constant in enum_type
                        .values
                        .iter()
                        .map(|&index| &oracle.constants[index])
                        .filter(|constant| !matches!(constant.value, ConstantValue::Alias(_)))
                    {
                        let constant_spec_name = &constant.spec_name;
                        let ident = AsIdent(&constant.short_name);
                        writeln!(w, "if (@hasDecl(c, \"{constant_spec_name}\")) try expectEqual(@intFromEnum({type_name}.{ident}), c.{constant_spec_name});")?;
                    }
                }
                true
            }
            _ => false,
        } {
            writeln!(
                w,
                "if (@hasDecl(c, \"{spec_name}\")) try expectEqual(@sizeOf({type_name}), @sizeOf(c.{spec_name}));"
            )?;
        }
    }

    writeln!(w, "}}")?;
    Ok(())
}

fn write_output(oracle: &Oracle, with_tests: bool) -> Res {
    let mut writer = io::BufWriter::new(File::create("zvulkan/vulkan.zig")?);

    write_version_comment(&mut writer, oracle)?;

    writeln!(&mut writer)?;
    io::copy(
        &mut io::BufReader::new(File::open("input/vulkan_prefix.zig")?),
        &mut writer,
    )?;

    write_global_constants(&mut writer, oracle)?;
    write_types(&mut writer, oracle)?;
    write_extension_names(&mut writer, oracle)?;
    write_extensions(&mut writer, oracle, ExtensionCategory::Instance)?;
    write_extensions(&mut writer, oracle, ExtensionCategory::Device)?;
    write_commands(&mut writer, oracle, CommandCategory::Global)?;
    write_commands(&mut writer, oracle, CommandCategory::Instance)?;
    write_commands(&mut writer, oracle, CommandCategory::Device)?;
    if with_tests {
        write_tests(&mut writer, oracle)?;
    }

    writeln!(&mut writer)?;
    io::copy(
        &mut io::BufReader::new(File::open("input/vulkan_postfix.zig")?),
        &mut writer,
    )?;

    Ok(())
}

fn main() -> Res {
    let xml_file_name = "../Vulkan-Docs/xml/vk.xml";
    let oracle = Oracle::new(xml_file_name);

    let with_tests = false;
    write_output(&oracle, with_tests)?;

    process::Command::new("zig")
        .arg("fmt")
        .arg("vulkan.zig")
        .current_dir("zvulkan")
        .output()?;

    Ok(())
}
