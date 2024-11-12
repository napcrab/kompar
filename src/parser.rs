use super::error::{Result, Error};
use num_bigint::BigUint;
use std::str::FromStr;
use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum KomparExpr {
    //Binary operators
    Add(Box<Self>, Box<Self>),
    Sub(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Mod(Box<Self>, Box<Self>),
    Exp(Box<Self>, Box<Self>),
    Equal(Box<Self>, Box<Self>),
    LessEqual(Box<Self>, Box<Self>),
    Less(Box<Self>, Box<Self>),
    Greater(Box<Self>, Box<Self>),
    GreaterEqual(Box<Self>, Box<Self>),
    NotEqual(Box<Self>, Box<Self>),
    And(Box<Self>, Box<Self>),
    Or(Box<Self>, Box<Self>),
    Xor(Box<Self>, Box<Self>),
    Index(Box<Self>, Box<Self>),
    In(Box<Self>, Box<Self>),
    As(Box<Self>, Box<Self>),
    Func(Box<Self>, Box<Self>),
    Call(Box<Self>, Box<Self>),

    // Unary operators
    Not(Box<Self>),
    Neg(Box<Self>),

    // Ident operators
    Field(Box<Self>, String),
    Ident(String),

    // Primitives
    Num(BigUint),
    Float(BigUint, BigUint),
    String(String),
    Char(char),
    Bool(bool),
    Tuple(Vec<Self>),
    List(Vec<Self>),
    Map(Vec<(String, Self)>),
    Set(Vec<Self>),

    // Scope control
    Let(String, Box<Self>, Box<Self>, Box<Self>),
    Apply(Box<Self>, Box<Self>),
    Import(String, Box<Self>),

    // Type definitions
    // Impl(),
    // Trait(),
    // Type(),

    // Control flow
}

pub fn parser() -> impl Parser<char, KomparExpr, Error = Simple<char>> {
    recursive(|expr| { 
        let int = text::int(10)
            .map(|s: String| KomparExpr::Num(BigUint::from_str(s.as_str()).unwrap()));

        let point = text::int(10).then_ignore(just('.')).then(text::int(10))
            .map(|(i, f): (String, String)| KomparExpr::Float(
                BigUint::from_str(i.as_str()).unwrap(),
                BigUint::from_str(f.as_str()).unwrap()
            ));

        let num = point.or(int);

        let escaped_char = just('\\').ignore_then(any()).map(|c: char| match c {
            'n' => '\n', //new line
            't' => '\t', //tab
            _ => c
        });

        let char = just('\'').ignore_then(
                escaped_char
                    .or(none_of("'"))
            ).then_ignore(just('\''))
            .map(|c: char| KomparExpr::Char(c));

        let string = just('\"').ignore_then(
                escaped_char
                    .or(none_of("\""))
                    .repeated()
            ).then_ignore(just('\"'))
            .collect().map(|s: String| KomparExpr::String(s));
        
        let bool = text::keyword("true").map(|()| KomparExpr::Bool(true))
            .or(
                text::keyword("false").map(|()| KomparExpr::Bool(false))
            );

        let ident = text::ident().map(|s: String| KomparExpr::Ident(s));

        let comma_separated = expr.clone()
            .separated_by(just(',').padded())
            .allow_trailing()
            .padded();

        let tuple = comma_separated.clone()
            .delimited_by(just('('), just(')'))
            .map(|e: Vec<KomparExpr>| KomparExpr::Tuple(e));

        let list = comma_separated.clone()
            .delimited_by(just('['), just(']'))
            .map(|e: Vec<KomparExpr>| KomparExpr::List(e));
        
        let set = comma_separated.clone()
            .delimited_by(just('{'), just('}'))
            .map(|e: Vec<KomparExpr>| KomparExpr::Set(e));

        let map = text::ident()
            .then_ignore(just('=').padded())
            .then(expr.clone())
            .separated_by(just(',').padded())
            .allow_trailing()
            .padded()
            .delimited_by(just('{'), just('}'))
            .map(|e: Vec<(String, KomparExpr)>| KomparExpr::Map(e));
        
        let atom = choice((num, char, string, bool, ident, tuple, list, set, map))
            .or(expr.delimited_by(just('('), just(')'))).padded();


        
        atom   
    })
}


