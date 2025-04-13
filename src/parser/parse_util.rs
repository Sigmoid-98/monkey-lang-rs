use crate::lexer::token::{Token, Tokens};
use crate::parser::ast::{Ident, Literal};

use nom::bytes::complete::take;
use nom::error::ErrorKind;
use nom::IResult;

pub(crate) fn parse_literal(input: Tokens) -> IResult<Tokens, Literal> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Tag,
        )))
    } else {
        match t1.tok[0].clone() {
            Token::IntLiteral(name) => Ok((i1, Literal::IntLiteral(name))),
            Token::StringLiteral(s) => Ok((i1, Literal::StringLiteral(s))),
            Token::BoolLiteral(b) => Ok((i1, Literal::BoolLiteral(b))),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                ErrorKind::Tag,
            ))),
        }
    }
}

pub(crate) fn parse_ident(input: Tokens) -> IResult<Tokens, Ident> {
    let (i1, t1) = take(1usize)(input)?;
    if t1.tok.is_empty() {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            ErrorKind::Tag,
        )))
    } else {
        match t1.tok[0].clone() {
            Token::Ident(name) => Ok((i1, Ident(name))),
            _ => Err(nom::Err::Error(nom::error::Error::new(
                input,
                ErrorKind::Tag,
            ))),
        }
    }
}
