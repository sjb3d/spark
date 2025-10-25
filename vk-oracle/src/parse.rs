use crate::{dependency::*, oracle::*};
use nom::{
    branch::alt,
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{char, digit1, hex_digit1, multispace0},
    combinator::{all_consuming, map, map_res, not, opt, peek, value},
    error::VerboseError,
    multi::{many0, separated_list1},
    number::complete::float,
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CDecoration {
    None,
    Const,
    Pointer,
    PointerToConst,
    PointerToPointer,
    PointerToConstPointerToConst,
}

#[derive(Debug, Clone, Copy)]
pub enum CArraySize<'a> {
    Literal(usize),
    Ident(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBuiltInType {
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CBaseType<'a> {
    BuiltIn(CBuiltInType),
    Named(&'a str),
}

#[derive(Debug, Clone, Copy)]
pub struct CType<'a> {
    pub base: CBaseType<'a>,
    pub decoration: CDecoration,
    pub array_size: Option<CArraySize<'a>>,
    pub bit_count: Option<u32>,
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

#[derive(Debug, Clone, Copy)]
pub enum CConstant {
    AnyInt(isize),
    UInt32(u32),
    UInt64(u64),
    Float(f32),
}

impl TryFrom<CConstant> for u32 {
    type Error = ();
    fn try_from(value: CConstant) -> Result<Self, Self::Error> {
        match value {
            CConstant::AnyInt(n) => Ok(n as u32),
            CConstant::UInt32(n) => Ok(n),
            CConstant::UInt64(n) => Ok(n as u32),
            CConstant::Float(_) => Err(()),
        }
    }
}

impl TryFrom<CConstant> for u64 {
    type Error = ();
    fn try_from(value: CConstant) -> Result<Self, Self::Error> {
        match value {
            CConstant::AnyInt(n) => Ok(n as u64),
            CConstant::UInt32(n) => Ok(n as u64),
            CConstant::UInt64(n) => Ok(n),
            CConstant::Float(_) => Err(()),
        }
    }
}

fn ignore_remainder<T>((_i, o): (&str, T)) -> T {
    o
}

type Res<'a, T> = IResult<&'a str, T, VerboseError<&'a str>>;

fn version_num(i: &str) -> Res<u16> {
    map_res(digit1, str::parse::<u16>)(i)
}

fn version(i: &str) -> Res<Version> {
    map(
        tuple((
            preceded(
                alt((
                    tag("VK_VERSION_"),
                    tag("VK_BASE_VERSION_"),
                    tag("VK_GRAPHICS_VERSION_"),
                    tag("VK_COMPUTE_VERSION_"),
                )),
                version_num,
            ),
            preceded(tag("_"), version_num),
        )),
        |(a, b)| Version::new(a, b),
    )(i)
}

pub fn try_parse_version(i: &str) -> Option<Version> {
    all_consuming(version)(i).map(ignore_remainder).ok()
}

fn depends_expr_inner(i: &str) -> Res<DependencyExpr<&str>> {
    alt((
        delimited(char('('), depends_expr, char(')')),
        map(version, DependencyExpr::Version),
        map(
            separated_pair(take_while1(is_ident), tag("::"), take_while1(is_ident)),
            |_| DependencyExpr::Feature,
        ),
        map(take_while1(is_ident), DependencyExpr::Extension),
    ))(i)
}

fn depends_expr(i: &str) -> Res<DependencyExpr<&str>> {
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

pub fn parse_depends(i: &str) -> DependencyExpr<&str> {
    all_consuming(depends_expr)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {i} -> {res:?}"))
}

fn is_ident(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
}

fn ident(i: &str) -> Res<&str> {
    preceded(multispace0, take_while1(is_ident))(i)
}

fn keyword<'a>(k: &'static str) -> impl FnMut(&'a str) -> Res<'a, &'a str> {
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

fn built_in_type(i: &str) -> Res<CBuiltInType> {
    alt((
        map(keyword("void"), |_| CBuiltInType::Void),
        map(keyword("char"), |_| CBuiltInType::Char),
        map(keyword("int"), |_| CBuiltInType::Int),
        map(keyword("float"), |_| CBuiltInType::F32),
        map(keyword("double"), |_| CBuiltInType::F64),
        map(keyword("uint8_t"), |_| CBuiltInType::U8),
        map(keyword("uint16_t"), |_| CBuiltInType::U16),
        map(keyword("uint32_t"), |_| CBuiltInType::U32),
        map(keyword("uint64_t"), |_| CBuiltInType::U64),
        map(keyword("int8_t"), |_| CBuiltInType::I8),
        map(keyword("int16_t"), |_| CBuiltInType::I16),
        map(keyword("int32_t"), |_| CBuiltInType::I32),
        map(keyword("int64_t"), |_| CBuiltInType::I64),
        map(keyword("size_t"), |_| CBuiltInType::USize),
    ))(i)
}

fn base_type(i: &str) -> Res<CBaseType> {
    alt((map(built_in_type, CBaseType::BuiltIn), map(ident, CBaseType::Named)))(i)
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
                    v => panic!("unsupported decoration {v:?} {base:?}"),
                },
                array_size,
                bit_count,
            },
        },
    ))
}

pub fn parse_variable_decl(i: &str) -> CVariableDecl {
    all_consuming(terminated(variable_decl, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {i} -> {res:?}"))
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

pub fn parse_func_pointer_typedef(i: &str) -> CFunctionDecl {
    all_consuming(terminated(function_ptr_typedef, multispace0))(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {i} -> {res:?}"))
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

pub fn parse_typedef(i: &str) -> Option<CVariableDecl> {
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
        map(
            preceded(tag("0x"), map_res(hex_digit1, |s: &str| isize::from_str_radix(s, 16))),
            CConstant::AnyInt,
        ),
        map(
            preceded(char('-'), map(map_res(digit1, str::parse::<isize>), |n| -n)),
            CConstant::AnyInt,
        ),
        map(map_res(digit1, str::parse::<isize>), CConstant::AnyInt),
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
                    CConstant::AnyInt(y) => CConstant::UInt32(x - y as u32),
                    _ => panic!("bad rhs type in arithmetic"),
                },
                _ => panic!("bad lhs type in arithmetic"),
            },
        ),
        constant_expr_inner,
    ))(i)
}

pub fn parse_constant_expr(i: &str) -> CConstant {
    all_consuming(constant_expr)(i)
        .map(ignore_remainder)
        .unwrap_or_else(|res| panic!("parse fail: {i} -> {res:?}"))
}

fn single_line_comment(i: &str) -> Res<()> {
    value((), pair(tag("//"), is_not("\n\r")))(i)
}

fn header_version(i: &str) -> Res<u16> {
    let (i, _) = opt(single_line_comment)(i)?;
    let (i, _) = op('#')(i)?;
    let (i, _) = keyword("define")(i)?;
    let (i, _) = keyword("VK_HEADER_VERSION")(i)?;
    preceded(multispace0, map_res(digit1, str::parse::<u16>))(i)
}

pub fn try_parse_header_version(i: &str) -> Option<u16> {
    all_consuming(terminated(header_version, multispace0))(i)
        .map(ignore_remainder)
        .ok()
}

fn header_version_complete(i: &str) -> Res<(u16, u16)> {
    let (i, _) = opt(single_line_comment)(i)?;
    let (i, _) = op('#')(i)?;
    let (i, _) = keyword("define")(i)?;
    let (i, _) = keyword("VK_HEADER_VERSION_COMPLETE")(i)?;
    let (i, _) = keyword("VK_MAKE_API_VERSION")(i)?;
    let (i, _) = op('(')(i)?;
    let (i, _) = preceded(multispace0, digit1)(i)?;
    let (i, _) = op(',')(i)?;
    let (i, maj) = preceded(multispace0, map_res(digit1, str::parse::<u16>))(i)?;
    let (i, _) = op(',')(i)?;
    let (i, min) = preceded(multispace0, map_res(digit1, str::parse::<u16>))(i)?;
    let (i, _) = op(',')(i)?;
    let (i, _) = keyword("VK_HEADER_VERSION")(i)?;
    let (i, _) = op(')')(i)?;
    Ok((i, (maj, min)))
}

pub fn try_parse_header_version_complete(i: &str) -> Option<(u16, u16)> {
    all_consuming(terminated(header_version_complete, multispace0))(i)
        .map(ignore_remainder)
        .ok()
}
