use nom::types::CompleteStr as Input;
use nom::*;

#[derive(Debug, PartialEq, Eq)]
pub enum CDecoration {
    None,
    Pointer,
    PointerToConst,
    PointerToPointer,
    PointerToConstPointerToConst,
}

#[derive(Debug)]
pub struct CType<'a> {
    pub name: &'a str,
    pub decoration: CDecoration,
    pub array_size: Option<&'a str>,
}

#[derive(Debug)]
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
pub enum CExpr {
    Literal(usize),
    Uint32(u32),
    Uint64(u64),
    Float(f32),
}

fn is_ident(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
        _ => false,
    }
}

#[rustfmt::skip]
named!(c_ident<Input, Input>, take_while1!(is_ident));

#[rustfmt::skip]
named!(c_const_keyword<Input, Input>, do_parse!(
    keyword:    tag!("const")           >>
                peek!(not!(c_ident))    >>
    (keyword)));

#[rustfmt::skip]
named!(c_struct_keyword<Input, Input>, do_parse!(
    keyword:    tag!("struct")          >>
                peek!(not!(c_ident))    >>
    (keyword)));

#[rustfmt::skip]
named!(c_variable_decl<Input, CVariableDecl>, ws!(do_parse!(
    const0:     opt!(c_const_keyword)                                   >>
                opt!(c_struct_keyword)                                  >>
    type_name:  c_ident                                                 >>
    ptr0:       opt!(char!('*'))                                        >>
    const1:     opt!(c_const_keyword)                                   >>
    ptr1:       opt!(char!('*'))                                        >>
    var_name:   c_ident                                                 >>
    array_size: opt!(ws!(delimited!(char!('['), c_ident, char!(']'))))  >>
    (CVariableDecl {
        name: var_name.0,
        ty: CType {
            name: type_name.0,
            decoration: match (const0.is_some(), ptr0.is_some(), const1.is_some(), ptr1.is_some()) {
                (false, false, false, false) | (true, false, false, false) => CDecoration::None,
                (false, true, false, false) => CDecoration::Pointer,
                (true, true, false, false) => CDecoration::PointerToConst,
                (false, true, false, true) => CDecoration::PointerToPointer,
                (true, true, true, true) => CDecoration::PointerToConstPointerToConst,
                v => panic!("unsupported decoration {:?}", v),
            },
            array_size: array_size.map(|s| s.0),
        },
    }))));

#[rustfmt::skip]
named!(c_function_decl<Input, CFunctionDecl>, ws!(do_parse!(
    ret_type_name:  c_ident                                             >>
    ret_ptr:        opt!(char!('*'))                                    >>
    func_name:      c_ident                                             >>
                    char!('(')                                          >>
    parameters:     alt!( separated_nonempty_list!(char!(','), c_variable_decl)
                        | tag!("void") => { |_| Vec::new() }
                    )                                                   >>
                    char!(')')                                          >>
                    char!(';')                                          >>
    (CFunctionDecl {
        proto: CVariableDecl {
            name: func_name.0,
            ty: CType {
                name: ret_type_name.0,
                decoration: if ret_ptr.is_some() { CDecoration::Pointer } else { CDecoration::None },
                array_size: None,
            }
        },
        parameters,
    }))));

#[rustfmt::skip]
named!(c_func_pointer_typedef<Input, CFunctionDecl>, ws!(do_parse!(
                    tag!("typedef")                                     >>
    ret_type_name:  c_ident                                             >>
    ret_ptr:        opt!(char!('*'))                                    >>
                    char!('(')                                          >>
                    tag!("VKAPI_PTR")                                   >>
                    char!('*')                                          >>
    func_name:      c_ident                                             >>
                    char!(')')                                          >>
                    char!('(')                                          >>
    parameters:     alt!( separated_nonempty_list!(char!(','), c_variable_decl)
                        | tag!("void") => { |_| Vec::new() }
                    )                                                   >>
                    char!(')')                                          >>
                    char!(';')                                          >>
    (CFunctionDecl {
        proto: CVariableDecl {
            name: func_name.0,
            ty: CType {
                name: ret_type_name.0,
                decoration: if ret_ptr.is_some() { CDecoration::Pointer } else { CDecoration::None },
                array_size: None,
            }
        },
        parameters,
    }))));

#[rustfmt::skip]
named!(c_typedef<Input, CVariableDecl>, ws!(do_parse!(
                    tag!("typedef")                                     >>
    type_name:      c_ident                                             >>
    var_name:       c_ident                                             >>
                    char!(';')                                          >>
    (CVariableDecl {
        name: var_name.0,
        ty: CType {
            name: type_name.0,
            decoration: CDecoration::None,
            array_size: None,
        }
    }))));

#[rustfmt::skip]
named!(c_expr2<Input, CExpr>, alt!(
    terminated!(flat_map!(call!(nom::recognize_float), parse_to!(f32)), char!('f'))
        => { |x| CExpr::Float(x) } |
    terminated!(flat_map!(call!(nom::digit), parse_to!(u64)), tag!("ULL"))
        => { |x| CExpr::Uint64(x) } |
    terminated!(flat_map!(call!(nom::digit), parse_to!(u32)), tag!("U"))
        => { |x| CExpr::Uint32(x) } |
    flat_map!(call!(nom::digit), parse_to!(usize))
        => { |x| CExpr::Literal(x) } |
    delimited!(char!('('), c_expr, char!(')'))
        |
    preceded!(char!('~'), c_expr2)
        => { |e| match e {
            CExpr::Uint32(x) => CExpr::Uint32(!x),
            CExpr::Uint64(x) => CExpr::Uint64(!x),
            _ => panic!("cannot bitwise invert unsized literal"),
        }}
));

// TODO: something more generic?
#[rustfmt::skip]
named!(c_expr<Input, CExpr>, alt!(
    separated_pair!(c_expr2, char!('-'), c_expr2)
        => { |(a, b)| match a {
            CExpr::Uint32(x) => match b {
                CExpr::Uint32(y) => CExpr::Uint32(x - y),
                CExpr::Literal(y) => CExpr::Uint32(x - y as u32),
                _ => panic!("bad rhs type in arithmetic"),
            },
            _ => panic!("bad lhs type in arithmetic"),
        }} |
    c_expr2
));

pub fn c_parse_int(s: &str) -> Option<i32> {
    if s.starts_with("0x") {
        i32::from_str_radix(&s[2..], 16).ok()
    } else {
        i32::from_str_radix(s, 10).ok()
    }
}

pub fn c_parse_variable_decl(s: &str) -> CVariableDecl {
    let (remain, decl) = c_variable_decl(Input(s)).unwrap_or_else(|res| {
        panic!("parse fail: {} -> {:?}", s, res);
    });
    assert!(remain.is_empty());
    decl
}

pub fn c_parse_function_decl(s: &str) -> CFunctionDecl {
    let (remain, typedef) = c_function_decl(Input(s)).unwrap_or_else(|res| {
        panic!("parse fail: {} -> {:?}", s, res);
    });
    assert!(remain.is_empty());
    typedef
}

pub fn c_parse_func_pointer_typedef(s: &str) -> CFunctionDecl {
    let (remain, typedef) = c_func_pointer_typedef(Input(s)).unwrap_or_else(|res| {
        panic!("parse fail: {} -> {:?}", s, res);
    });
    assert!(remain.is_empty());
    typedef
}

pub fn c_parse_typedef(s: &str) -> CVariableDecl {
    let (remain, decl) = c_typedef(Input(s)).unwrap_or_else(|res| {
        panic!("parse fail: {} -> {:?}", s, res);
    });
    assert!(remain.is_empty());
    decl
}

pub fn c_parse_expr(s: &str) -> CExpr {
    let (remain, expr) = c_expr(Input(s)).unwrap_or_else(|res| {
        panic!("parse fail: {} -> {:?}", s, res);
    });
    assert!(remain.is_empty());
    expr
}
