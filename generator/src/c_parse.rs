use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1, hex_digit1, multispace0},
    combinator::{all_consuming, map, map_res, not, opt, peek},
    error::VerboseError,
    multi::{many0, separated_list1},
    number::complete::float,
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum DependencyExpr<'a> {
    Never,
    Always,
    Version((u16, u16)),
    Extension(&'a str),
    Feature((&'a str, &'a str)),
    And(Vec<DependencyExpr<'a>>),
    Or(Vec<DependencyExpr<'a>>),
}

impl<'a> DependencyExpr<'a> {
    pub fn is_never(&self) -> bool {
        matches!(self, DependencyExpr::Never)
    }

    pub fn is_always(&self) -> bool {
        matches!(self, DependencyExpr::Always)
    }

    pub fn visit_leaves(&mut self, f: &impl Fn(&mut DependencyExpr)) {
        match self {
            Self::Never | Self::Always | Self::Version(_) | Self::Extension(_) | Self::Feature(_) => f(self),
            Self::And(v) | Self::Or(v) => {
                for dep in v.iter_mut() {
                    dep.visit_leaves(f)
                }
            }
        }
    }

    fn matches(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Never, Self::Never) => true,
            (Self::Always, Self::Always) => true,
            (Self::Version(a), Self::Version(b)) => a == b,
            (Self::Extension(a), Self::Extension(b)) => a == b,
            (Self::Feature(a), Self::Feature(b)) => a == b,
            (Self::And(ref a), Self::And(ref b)) | (Self::Or(ref a), Self::Or(ref b)) => match (a.len(), b.len()) {
                (0, 0) => true,
                (1, 1) => a[0].matches(&b[0]),
                (2, 2) => (a[0].matches(&b[0]) && a[1].matches(&b[1])) || (a[0].matches(&b[1]) && a[1].matches(&b[0])),
                _ => false,
            },
            _ => false,
        }
    }

    fn try_merge_and(a: &Self, b: &Self) -> Option<Self> {
        if a.matches(b) {
            Some(a.clone())
        } else {
            match (a, b) {
                (Self::Never, _) | (_, Self::Never) => Some(Self::Never),
                (Self::Always, other) | (other, Self::Always) => Some(other.clone()),
                (Self::Version(a), Self::Version(b)) => Some(Self::Version(*a.max(b))),
                _ => None,
            }
        }
    }

    fn try_merge_or(a: &Self, b: &Self) -> Option<Self> {
        if a.matches(b) {
            Some(a.clone())
        } else {
            match (a, b) {
                (Self::Never, other) | (other, Self::Never) => Some(other.clone()),
                (Self::Always, _) | (_, Self::Always) => Some(Self::Always),
                (Self::Version(a), Self::Version(b)) => Some(Self::Version(*a.min(b))),
                _ => None,
            }
        }
    }

    fn max_involved_version(&self) -> (u16, u16) {
        match self {
            Self::Never | Self::Always | Self::Extension(_) | Self::Feature(_) => (0, 0),
            Self::Version(v) => *v,
            Self::And(v) | Self::Or(v) => {
                let mut result = (0, 0);
                for dep in v.iter() {
                    result = result.max(dep.max_involved_version());
                }
                result
            }
        }
    }

    pub fn simplify(&mut self) {
        match self {
            Self::Never => {}
            Self::Always => {}
            Self::Version(v) => {
                if *v == (1, 0) {
                    *self = Self::Always;
                }
            }
            Self::Extension(_) => {}
            Self::Feature(_) => {}
            Self::And(v) => {
                let mut tmp = Vec::new();
                for mut dep in v.drain(..) {
                    dep.simplify();
                    if let Self::And(mut inner) = dep {
                        tmp.append(&mut inner);
                    } else {
                        tmp.push(dep);
                    }
                }
                let mut result = Vec::new();
                'outer: for dep in tmp.drain(..) {
                    for prev in &mut result {
                        if let Some(merged) = Self::try_merge_and(prev, &dep) {
                            *prev = merged;
                            continue 'outer;
                        }
                    }
                    result.push(dep);
                }
                match result.len() {
                    0 => unreachable!(),
                    1 => *self = result.pop().unwrap(),
                    _ => *v = result,
                }
            }
            Self::Or(v) => {
                let mut tmp = Vec::new();
                for mut dep in v.drain(..) {
                    dep.simplify();
                    if let Self::Or(mut inner) = dep {
                        tmp.append(&mut inner);
                    } else {
                        tmp.push(dep);
                    }
                }
                let mut result = Vec::new();
                'outer: for dep in tmp.drain(..) {
                    for prev in &mut result {
                        if let Some(merged) = Self::try_merge_or(prev, &dep) {
                            *prev = merged;
                            continue 'outer;
                        }
                    }
                    result.push(dep);
                }
                result.sort_by(|a, b| {
                    let av = a.max_involved_version();
                    let bv = b.max_involved_version();
                    av.cmp(&bv).reverse()
                });
                match result.len() {
                    0 => unreachable!(),
                    1 => *self = result.pop().unwrap(),
                    _ => *v = result,
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CDecoration {
    None,
    Const,
    Pointer,
    PointerToConst,
    PointerToPointer,
    PointerToConstPointerToConst,
}

impl CDecoration {
    pub fn is_pointer(&self) -> bool {
        match self {
            CDecoration::None | CDecoration::Const => false,
            CDecoration::Pointer
            | CDecoration::PointerToConst
            | CDecoration::PointerToPointer
            | CDecoration::PointerToConstPointerToConst => true,
        }
    }

    pub fn is_mutable(&self) -> bool {
        match self {
            CDecoration::None
            | CDecoration::Const
            | CDecoration::PointerToConst
            | CDecoration::PointerToConstPointerToConst => false,
            CDecoration::Pointer | CDecoration::PointerToPointer => true,
        }
    }
}

impl fmt::Display for CDecoration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            CDecoration::None | CDecoration::Const => "",
            CDecoration::Pointer => "* mut ",
            CDecoration::PointerToConst => "* const ",
            CDecoration::PointerToPointer => "* mut *mut ",
            CDecoration::PointerToConstPointerToConst => "*const *const ",
        };
        s.fmt(f)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CArraySize<'a> {
    Literal(usize),
    Ident(&'a str),
}

impl fmt::Display for CArraySize<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CArraySize::Literal(n) => n.fmt(f),
            CArraySize::Ident(s) => s.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBaseType<'a> {
    Void,
    Char,
    Int,
    F32,
    F64,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    USize,
    Named(&'a str),
}

impl<'a> CBaseType<'a> {
    pub fn try_name(&self) -> Option<&'a str> {
        if let CBaseType::Named(name) = *self {
            Some(name)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CType<'a> {
    pub base: CBaseType<'a>,
    pub decoration: CDecoration,
    pub array_size: Option<CArraySize<'a>>,
    pub bit_count: Option<u32>,
}

impl<'a> CType<'a> {
    pub fn is_base_type(&self, base: CBaseType) -> bool {
        self.base == base && !self.decoration.is_pointer() && self.array_size.is_none() && self.bit_count.is_none()
    }

    pub fn strip_array(&self) -> CType<'a> {
        if self.array_size.is_some() {
            let decoration = match self.decoration {
                CDecoration::None => CDecoration::Pointer,
                CDecoration::Const => CDecoration::PointerToConst,
                _ => panic!("cannot convert array to pointer type"),
            };
            CType {
                base: self.base,
                decoration,
                array_size: None,
                bit_count: self.bit_count,
            }
        } else {
            Clone::clone(self)
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CVariableDecl<'a> {
    pub name: &'a str,
    pub ty: CType<'a>,
}

#[derive(Debug)]
pub struct CFunctionDecl<'a> {
    pub proto: CVariableDecl<'a>,
    pub parameters: Vec<CVariableDecl<'a>>,
}

#[derive(Debug)]
pub enum CConstant {
    UInt(usize),
    UInt32(u32),
    UInt64(u64),
    Float(f32),
}

fn ignore_remainder<T>((_i, o): (&str, T)) -> T {
    o
}

type Res<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

fn version_num(i: &str) -> Res<u16> {
    map_res(digit1, str::parse::<u16>)(i)
}

fn version(i: &str) -> Res<(u16, u16)> {
    tuple((
        preceded(tag("VK_VERSION_"), version_num),
        preceded(tag("_"), version_num),
    ))(i)
}

pub fn c_try_parse_version(i: &str) -> Option<(u16, u16)> {
    all_consuming(version)(i).map(ignore_remainder).ok()
}

fn depends_expr_inner(i: &str) -> Res<DependencyExpr> {
    alt((
        delimited(char('('), depends_expr, char(')')),
        map(version, DependencyExpr::Version),
        map(
            separated_pair(take_while1(is_ident), tag("::"), take_while1(is_ident)),
            DependencyExpr::Feature,
        ),
        map(take_while1(is_ident), DependencyExpr::Extension),
    ))(i)
}

fn depends_expr(i: &str) -> Res<DependencyExpr> {
    let (mut i, mut dep) = depends_expr_inner(i)?;
    loop {
        // equal precedence!
        let (op_i, op_c) = opt(alt((char('+'), char(','))))(i)?;
        if let Some(c) = op_c {
            let (other_i, other) = depends_expr_inner(op_i)?;
            i = other_i;
            dep = match c {
                '+' => DependencyExpr::And(vec![dep, other]),
                ',' => DependencyExpr::Or(vec![dep, other]),
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }
    Ok((i, dep))
}

pub fn c_parse_depends(i: &str) -> DependencyExpr {
    all_consuming(depends_expr)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn parse_i32(i: &str) -> Res<i32> {
    alt((
        preceded(tag("0x"), map_res(hex_digit1, |s: &str| i32::from_str_radix(s, 16))),
        preceded(char('-'), map(map_res(digit1, str::parse::<i32>), |n| -n)),
        map_res(digit1, str::parse::<i32>),
    ))(i)
}

pub fn c_parse_int(i: &str) -> i32 {
    all_consuming(parse_i32)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn is_ident(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
}

fn ident(i: &str) -> Res<&str> {
    preceded(multispace0, take_while1(is_ident))(i)
}

fn keyword<'a>(k: &'static str) -> impl FnMut(&'a str) -> Res<'a, &str> {
    delimited(multispace0, tag(k), not(peek(take_while1(is_ident))))
}

fn op<'a>(c: char) -> impl FnMut(&'a str) -> Res<'a, char> {
    preceded(multispace0, char(c))
}

fn array_size(i: &str) -> Res<CArraySize> {
    alt((
        map(map_res(digit1, str::parse::<usize>), CArraySize::Literal),
        map(ident, CArraySize::Ident),
    ))(i)
}

fn base_type(i: &str) -> Res<CBaseType> {
    alt((
        map(keyword("void"), |_| CBaseType::Void),
        map(keyword("char"), |_| CBaseType::Char),
        map(keyword("int"), |_| CBaseType::Int),
        map(keyword("float"), |_| CBaseType::F32),
        map(keyword("double"), |_| CBaseType::F64),
        map(keyword("uint8_t"), |_| CBaseType::U8),
        map(keyword("uint16_t"), |_| CBaseType::U16),
        map(keyword("uint32_t"), |_| CBaseType::U32),
        map(keyword("uint64_t"), |_| CBaseType::U64),
        map(keyword("int8_t"), |_| CBaseType::I8),
        map(keyword("int16_t"), |_| CBaseType::I16),
        map(keyword("int32_t"), |_| CBaseType::I32),
        map(keyword("int64_t"), |_| CBaseType::I64),
        map(keyword("size_t"), |_| CBaseType::USize),
        map(ident, CBaseType::Named),
    ))(i)
}

fn variable_decl(i: &str) -> Res<CVariableDecl> {
    let (i, const0) = opt(keyword("const"))(i)?;
    let (i, _) = opt(keyword("struct"))(i)?;
    let (i, base) = base_type(i)?;
    let (i, ptr0) = opt(op('*'))(i)?;
    let (i, const1) = opt(keyword("const"))(i)?;
    let (i, ptr1) = opt(op('*'))(i)?;
    let (i, var_name) = ident(i)?;
    let (i, array_sizes) = many0(delimited(op('['), array_size, op(']')))(i)?;
    let (i, bit_count) = opt(preceded(op(':'), map_res(digit1, str::parse::<u32>)))(i)?;

    let array_size = array_sizes.split_first().map(|(&first, rest)| {
        rest.iter().fold(first, |acc, x| match (acc, x) {
            (CArraySize::Literal(a), CArraySize::Literal(b)) => CArraySize::Literal(a * b),
            _ => panic!("cannot fold array sizes"),
        })
    });

    Ok((
        i,
        CVariableDecl {
            name: var_name,
            ty: CType {
                base,
                decoration: match (const0.is_some(), ptr0.is_some(), const1.is_some(), ptr1.is_some()) {
                    (false, false, false, false) => CDecoration::None,
                    (true, false, false, false) => CDecoration::Const,
                    (false, true, false, false) => CDecoration::Pointer,
                    (true, true, false, false) => CDecoration::PointerToConst,
                    (false, true, false, true) => CDecoration::PointerToPointer,
                    (true, true, true, true) => CDecoration::PointerToConstPointerToConst,
                    v => panic!("unsupported decoration {:?}", v),
                },
                array_size,
                bit_count,
            },
        },
    ))
}

pub fn c_parse_variable_decl(i: &str) -> CVariableDecl {
    all_consuming(terminated(variable_decl, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn function_decl(i: &str) -> Res<CFunctionDecl> {
    let (i, ret_base) = base_type(i)?;
    let (i, ret_ptr) = opt(op('*'))(i)?;
    let (i, func_name) = ident(i)?;
    let (i, parameters) = delimited(
        op('('),
        alt((
            separated_list1(op(','), variable_decl),
            map(keyword("void"), |_| Vec::new()),
        )),
        tuple((op(')'), op(';'))),
    )(i)?;
    Ok((
        i,
        CFunctionDecl {
            proto: CVariableDecl {
                name: func_name,
                ty: CType {
                    base: ret_base,
                    decoration: if ret_ptr.is_some() {
                        CDecoration::Pointer
                    } else {
                        CDecoration::None
                    },
                    array_size: None,
                    bit_count: None,
                },
            },
            parameters,
        },
    ))
}

pub fn c_parse_function_decl(i: &str) -> CFunctionDecl {
    all_consuming(terminated(function_decl, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn function_ptr_typedef(i: &str) -> Res<'_, CFunctionDecl> {
    let (i, ret_base) = preceded(keyword("typedef"), base_type)(i)?;
    let (i, ret_ptr) = opt(op('*'))(i)?;
    let (i, func_name) = delimited(tuple((op('('), keyword("VKAPI_PTR"), op('*'))), ident, op(')'))(i)?;
    let (i, parameters) = delimited(
        op('('),
        alt((
            separated_list1(op(','), variable_decl),
            map(keyword("void"), |_| Vec::new()),
        )),
        tuple((op(')'), op(';'))),
    )(i)?;
    Ok((
        i,
        CFunctionDecl {
            proto: CVariableDecl {
                name: func_name,
                ty: CType {
                    base: ret_base,
                    decoration: if ret_ptr.is_some() {
                        CDecoration::Pointer
                    } else {
                        CDecoration::None
                    },
                    array_size: None,
                    bit_count: None,
                },
            },
            parameters,
        },
    ))
}

pub fn c_parse_func_pointer_typedef(i: &str) -> CFunctionDecl {
    all_consuming(terminated(function_ptr_typedef, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

fn typedef(i: &str) -> Res<CVariableDecl> {
    let (i, base) = preceded(keyword("typedef"), base_type)(i)?;
    let (i, ptr) = opt(op('*'))(i)?;
    let (i, var_name) = terminated(ident, op(';'))(i)?;
    Ok((
        i,
        CVariableDecl {
            name: var_name,
            ty: CType {
                base,
                decoration: if ptr.is_some() {
                    CDecoration::Pointer
                } else {
                    CDecoration::None
                },
                array_size: None,
                bit_count: None,
            },
        },
    ))
}

pub fn c_try_parse_typedef(i: &str) -> Option<CVariableDecl> {
    all_consuming(terminated(typedef, multispace0))(i)
        .map(ignore_remainder)
        .ok()
}

fn constant_expr_inner(i: &str) -> Res<CConstant> {
    alt((
        map(terminated(float, alt((char('f'), char('F')))), CConstant::Float),
        map(
            terminated(map_res(digit1, str::parse::<u64>), tag("ULL")),
            CConstant::UInt64,
        ),
        map(
            terminated(map_res(digit1, str::parse::<u32>), tag("U")),
            CConstant::UInt32,
        ),
        map(map_res(digit1, str::parse::<usize>), CConstant::UInt),
        delimited(char('('), constant_expr, char(')')),
        map(preceded(char('~'), constant_expr_inner), |e| match e {
            CConstant::UInt32(x) => CConstant::UInt32(!x),
            CConstant::UInt64(x) => CConstant::UInt64(!x),
            _ => panic!("cannot bitwise invert unsized literal"),
        }),
    ))(i)
}

fn constant_expr(i: &str) -> Res<CConstant> {
    alt((
        map(
            separated_pair(constant_expr_inner, char('-'), constant_expr_inner),
            |(a, b)| match a {
                CConstant::UInt32(x) => match b {
                    CConstant::UInt32(y) => CConstant::UInt32(x - y),
                    CConstant::UInt(y) => CConstant::UInt32(x - y as u32),
                    _ => panic!("bad rhs type in arithmetic"),
                },
                _ => panic!("bad lhs type in arithmetic"),
            },
        ),
        constant_expr_inner,
    ))(i)
}

pub fn c_parse_constant_expr(i: &str) -> CConstant {
    all_consuming(constant_expr)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}

#[derive(Debug)]
pub enum CExpr<'a> {
    Bracket(Box<Self>),
    Mul(Box<(Self, Self)>),
    Div(Box<(Self, Self)>),
    Add(Box<(Self, Self)>),
    Literal(usize),
    Ident(&'a str),
}

impl<'a> CExpr<'a> {
    pub fn write_to(&self, w: &mut impl fmt::Write, f: impl Fn(&'a str) -> String + Copy) -> fmt::Result {
        match self {
            Self::Bracket(e) => {
                write!(w, "(")?;
                e.write_to(w, f)?;
                write!(w, ")")
            }
            Self::Mul(e) => {
                e.0.write_to(w, f)?;
                write!(w, " * ")?;
                e.1.write_to(w, f)
            }
            Self::Div(e) => {
                e.0.write_to(w, f)?;
                write!(w, " / ")?;
                e.1.write_to(w, f)
            }
            Self::Add(e) => {
                e.0.write_to(w, f)?;
                write!(w, " + ")?;
                e.1.write_to(w, f)
            }
            Self::Literal(n) => write!(w, "{}", n),
            Self::Ident(s) => write!(w, "{}", f(s)),
        }
    }
}

fn expr_inner(i: &str) -> Res<CExpr<'_>> {
    preceded(
        multispace0,
        alt((
            map(delimited(char('('), expr, char(')')), |expr| {
                CExpr::Bracket(Box::new(expr))
            }),
            map(map_res(digit1, str::parse::<usize>), CExpr::Literal),
            map(take_while1(is_ident), CExpr::Ident),
        )),
    )(i)
}

fn expr(i: &str) -> Res<CExpr<'_>> {
    alt((
        map(separated_pair(expr_inner, op('+'), expr), |(a, b)| {
            CExpr::Add(Box::new((a, b)))
        }),
        map(separated_pair(expr_inner, op('*'), expr), |(a, b)| {
            CExpr::Mul(Box::new((a, b)))
        }),
        map(separated_pair(expr_inner, op('/'), expr), |(a, b)| {
            CExpr::Div(Box::new((a, b)))
        }),
        expr_inner,
    ))(i)
}

pub fn c_parse_expr(i: &str) -> CExpr<'_> {
    all_consuming(expr)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {} -> {:?}", i, res))
}
