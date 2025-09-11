use crate::{builder::*, dependency::*, parse::*};
use std::{num, path::Path};
use vk_parse as vk;

macro_rules! typed_index {
    ($index:ident, $element:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $index(num::NonZeroU32);

        impl $index {
            pub(crate) fn position(self) -> usize {
                (self.0.get() - 1) as usize
            }

            pub(crate) fn from_position(pos: usize) -> Self {
                Self(num::NonZeroU32::new((pos + 1) as u32).unwrap())
            }
        }

        impl ::std::ops::Index<$index> for Vec<$element> {
            type Output = $element;
            fn index(&self, index: $index) -> &Self::Output {
                self.index(index.position())
            }
        }
    };
}

typed_index!(ExtensionIndex, Extension);
typed_index!(ConstantIndex, Constant);
typed_index!(TypeIndex, Type);
typed_index!(CommandIndex, Command);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16) -> Self {
        Self { major, minor }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExtensionCategory {
    Instance,
    Device,
}

pub type ExtensionDependencyExpr = DependencyExpr<ExtensionIndex>;

#[derive(Debug)]
pub struct Extension {
    pub index: ExtensionIndex,
    pub spec_name: String,
    pub short_name: String,
    pub category: ExtensionCategory,
    pub promoted_to_version: Option<Version>,
    pub dependencies: ExtensionDependencyExpr,
    pub has_instance_dependency: bool,
}

impl Extension {
    pub fn affects_category(&self, category: ExtensionCategory) -> bool {
        self.category == category || (category == ExtensionCategory::Instance && self.has_instance_dependency)
    }

    pub fn category_dependencies(
        &self,
        category: ExtensionCategory,
        oracle: &Oracle,
    ) -> DependencyExpr<ExtensionIndex> {
        let mut deps = self.dependencies.clone();

        // strip out any dependencies that we cannot check in this category (assume they are true)
        deps.apply_at_leaves(|dep| {
            if let DependencyExpr::Extension(index) = dep {
                if !oracle.extensions[*index].affects_category(category) {
                    *dep = DependencyExpr::Always;
                }
            }
        });

        // add a dependency on ourself
        if self.category == category {
            deps = DependencyExpr::And(vec![DependencyExpr::Extension(self.index), deps]);
        }

        // early out if the extension has been promoted
        if let Some(version) = self.promoted_to_version {
            deps = DependencyExpr::Or(vec![DependencyExpr::Version(version), deps]);
        }

        deps.simplify();
        deps
    }
}

pub type BuiltInDecl = CBuiltInType;

#[derive(Debug)]
pub enum ArraySize {
    Unknown,
    Literal(Literal),
    Constant(ConstantIndex),
    Named(String),
}

#[derive(Debug)]
pub struct ArrayInfo {
    pub size: ArraySize,
    pub is_null_terminated: bool,
}

#[derive(Debug)]
pub struct ArrayDecl {
    pub array: ArrayInfo,
    pub is_const: bool,
    pub element_type: Box<TypeDecl>,
}

#[derive(Debug)]
pub struct PointerDecl {
    pub is_const: bool,
    pub array_hint: Option<ArrayInfo>,
    pub element_type: Box<TypeDecl>,
}

#[derive(Debug)]
pub enum ExternalDecl {
    Opaque,
    CULong,
}

#[derive(Debug)]
pub enum TypeDecl {
    BuiltIn(BuiltInDecl),
    Type(TypeIndex),
    Array(ArrayDecl),
    Pointer(PointerDecl),
    External(ExternalDecl),
}

#[derive(Debug)]
pub struct MemberDecl {
    pub spec_name: String,
    pub short_name: String,
    pub ty: TypeDecl,
    pub default: Option<ConstantIndex>,
    pub is_optional: bool,
    pub setter_transform: ParameterTransform,
}

#[derive(Debug, Clone, Copy)]
pub enum Literal {
    Int(isize),
    U32(u32),
    U64(u64),
    F32(f32),
}

impl Literal {
    pub fn type_decl(&self) -> Option<TypeDecl> {
        match self {
            Literal::Int(_) => None,
            Literal::U32(_) => Some(BuiltInDecl::U32),
            Literal::U64(_) => Some(BuiltInDecl::U64),
            Literal::F32(_) => Some(BuiltInDecl::F32),
        }
        .map(TypeDecl::BuiltIn)
    }
}

#[derive(Debug)]
pub enum ConstantValue {
    Literal(Literal),
    Alias(ConstantIndex),
}

#[derive(Debug)]
pub struct Constant {
    pub index: ConstantIndex,
    pub spec_name: String,
    pub short_name: String,
    pub enum_type_index: Option<TypeIndex>,
    pub value: ConstantValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitWidth {
    U32,
    U64,
}

#[derive(Debug)]
pub struct EnumType {
    pub bitmask_width: Option<BitWidth>,
    pub values: Vec<ConstantIndex>,
}

#[derive(Debug)]
pub struct AggregateType {
    pub is_union: bool,
    pub members: Vec<MemberDecl>,
    pub extends: Vec<TypeIndex>,
    pub extended_by: Vec<TypeIndex>,
    pub returned_only_hint: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum HandleType {
    U64,
    USize,
}

#[derive(Debug)]
pub struct SliceParameterTransform {
    pub length_check: Option<String>,
}

#[derive(Debug)]
pub struct SliceLengthParameterTransform {
    pub required_param: Vec<usize>,
    pub optional_param: Vec<usize>,
    pub separate_param: Vec<usize>,
}

#[derive(Debug)]
pub enum ParameterTransform {
    None,
    FromMemberHandle,
    FromBool,
    FromSlice(SliceParameterTransform),
    FromSliceLength(SliceLengthParameterTransform),
    FromOutput,
}

#[derive(Debug)]
pub struct ParameterDecl {
    pub name: String,
    pub ty: TypeDecl,
    pub transform: ParameterTransform,
    pub is_optional: bool,
}

#[derive(Debug)]
pub struct FunctionPointerType {
    pub return_type: TypeDecl,
    pub parameters: Vec<ParameterDecl>,
}

#[derive(Debug)]
pub enum TypeDetail {
    Alias(TypeDecl),
    Enum(EnumType),
    Aggregate(AggregateType),
    Handle(HandleType),
    FunctionPointer(FunctionPointerType),
    External(TypeDecl),
}

#[derive(Debug)]
pub struct Type {
    pub index: TypeIndex,
    pub spec_name: String,
    pub short_name: String,
    pub detail: TypeDetail,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CommandCategory {
    Global,
    Instance,
    Device,
}

impl From<ExtensionCategory> for CommandCategory {
    fn from(value: ExtensionCategory) -> Self {
        match value {
            ExtensionCategory::Instance => Self::Instance,
            ExtensionCategory::Device => Self::Device,
        }
    }
}

#[derive(Debug)]
pub struct ObjectOutputTransform {
    pub output_param: usize,
    pub into_bool: bool,
}

#[derive(Debug)]
pub struct EnumerateOutputTransform {
    pub count_param: usize,
    pub elements_param: usize,
}

#[derive(Debug)]
pub enum CommandOutputTransform {
    None,
    IntoObject(ObjectOutputTransform),
    IntoEnumerate(EnumerateOutputTransform),
    IntoBool,
}

#[derive(Debug)]
pub struct CommandFunction {
    pub load_on_instance: bool,
    pub function_type_index: TypeIndex,
    pub aliases: Vec<CommandIndex>,
    pub success_codes: Vec<ConstantIndex>,
    pub error_codes: Vec<ConstantIndex>,
    pub output_transform: CommandOutputTransform,
}

#[derive(Debug)]
pub enum CommandDetail {
    Function(CommandFunction),
    Alias(CommandIndex),
}

#[derive(Debug)]
pub struct Command {
    pub index: CommandIndex,
    pub spec_name: String,
    pub short_name: String,
    pub category: CommandCategory,
    pub dependencies: DependencyExpr<ExtensionIndex>,
    pub detail: CommandDetail,
}

#[derive(Debug, Default)]
pub struct Oracle {
    pub header_version: (u16, u16, u16),
    pub types: Vec<Type>,
    pub constants: Vec<Constant>,
    pub extensions: Vec<Extension>,
    pub commands: Vec<Command>,
}

impl Oracle {
    pub fn new(xml_file_name: impl AsRef<Path>) -> Self {
        let (registry, errors) = vk::parse_file(xml_file_name.as_ref()).unwrap();
        for error in &errors {
            eprintln!("Parser error: {error:?}");
        }

        OracleBuilder::new(&registry).build()
    }
}
