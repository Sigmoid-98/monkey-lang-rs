mod char_util;
mod token;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::combinator::{map, map_res, recognize};
use nom::{AsBytes, IResult};
use std::str::Utf8Error;

use crate::lexer::char_util::{
    complete_byte_slice_str_from_utf8, complete_str_from_str, concat_slice_vec, convert_vec_utf8,
};
use crate::lexer::token::{Token, Tokens};
use nom::character::complete::{alpha1, alphanumeric1, digit1, multispace0};
use nom::multi::many0;
use nom::sequence::{delimited, pair};
use std::str;

macro_rules! syntax_func_map_tag {
    ($func_name: ident, $tag_string: literal, $output_token: expr) => {
        fn $func_name<'a>(s: &'a [u8]) -> IResult<&[u8], Token> {
            map(tag($tag_string), |_| $output_token)(s)
        }
    };
}

// operators
syntax_func_map_tag!(equal_operator, "==", Token::Equal);
syntax_func_map_tag!(not_equal_operator, "!=", Token::NotEqual);
syntax_func_map_tag!(assign_operator, "=", Token::Assign);
syntax_func_map_tag!(plus_operator, "+", Token::Plus);
syntax_func_map_tag!(minus_operator, "-", Token::Minus);
syntax_func_map_tag!(multiply_operator, "*", Token::Multiply);
syntax_func_map_tag!(divide_operator, "/", Token::Divide);
syntax_func_map_tag!(not_operator, "!", Token::Not);
syntax_func_map_tag!(greater_than_equal_operator, ">=", Token::GreaterThanEqual);
syntax_func_map_tag!(lesser_than_equal_operator, "<=", Token::LessThanEqual);
syntax_func_map_tag!(greater_than_operator, ">", Token::GreaterThan);
syntax_func_map_tag!(lesser_than_operator, "<", Token::LessThan);

pub fn lex_operator(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        equal_operator,
        not_equal_operator,
        assign_operator,
        plus_operator,
        minus_operator,
        multiply_operator,
        divide_operator,
        not_operator,
        greater_than_equal_operator,
        lesser_than_equal_operator,
        greater_than_operator,
        lesser_than_operator,
    ))(input)
}

// punctuations
syntax_func_map_tag!(comma_punctuation, ",", Token::Comma);
syntax_func_map_tag!(semicolon_punctuation, ";", Token::SemiColon);
syntax_func_map_tag!(colon_punctuation, ":", Token::Colon);
syntax_func_map_tag!(lparen_punctuation, "(", Token::LParen);
syntax_func_map_tag!(rparen_punctuation, ")", Token::RParen);
syntax_func_map_tag!(lbrace_punctuation, "{", Token::LBrace);
syntax_func_map_tag!(rbrace_punctuation, "}", Token::RBrace);
syntax_func_map_tag!(lbracket_punctuation, "[", Token::LBracket);
syntax_func_map_tag!(rbracket_punctuation, "]", Token::RBracket);

pub fn lex_punctuations(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        comma_punctuation,
        semicolon_punctuation,
        colon_punctuation,
        lparen_punctuation,
        rparen_punctuation,
        lbrace_punctuation,
        rbrace_punctuation,
        lbracket_punctuation,
        rbracket_punctuation,
    ))(input)
}

// strings
fn pis(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    use std::result::Result::{Err, Ok};

    let (i1, c1) = take(1usize)(input)?;
    match c1.as_bytes() {
        b"\"" => Ok((input, vec![])),
        b"\\" => {
            let (i2, c2) = take(1usize)(i1)?;
            pis(i2).map(|(slice, done)| (slice, concat_slice_vec(c2, done)))
        }
        c => pis(i1).map(|(slice, done)| (slice, concat_slice_vec(c, done))),
    }
}

fn string(input: &[u8]) -> IResult<&[u8], String> {
    delimited(tag("\""), map_res(pis, convert_vec_utf8), tag("\""))(input)
}

fn lex_string(input: &[u8]) -> IResult<&[u8], Token> {
    map(string, Token::StringLiteral)(input)
}

fn lex_reserved_ident(input: &[u8]) -> IResult<&[u8], Token> {
    map_res(
        recognize(pair(
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        |s| {
            let c = complete_byte_slice_str_from_utf8(s);
            c.map(|syntax| match syntax {
                "let" => Token::Let,
                "fn" => Token::Function,
                "if" => Token::If,
                "else" => Token::Else,
                "return" => Token::Return,
                "true" => Token::BoolLiteral(true),
                "false" => Token::BoolLiteral(false),
                _ => Token::Ident(syntax.to_string()),
            })
        },
    )(input)
}

// Integers parser
fn lex_integer(input: &[u8]) -> IResult<&[u8], Token> {
    map(
        map_res(
            map_res(digit1, complete_byte_slice_str_from_utf8),
            complete_str_from_str,
        ),
        Token::IntLiteral,
    )(input)
}

// Illegal tokens
fn lex_illegal(input: &[u8]) -> IResult<&[u8], Token> {
    map(take(1usize), |_| Token::Illegal)(input)
}

fn lex_token(input: &[u8]) -> IResult<&[u8], Token> {
    alt((
        lex_operator,
        lex_punctuations,
        lex_string,
        lex_reserved_ident,
        lex_integer,
        lex_illegal,
    ))(input)
}

fn lex_tokens(input: &[u8]) -> IResult<&[u8], Vec<Token>> {
    many0(delimited(multispace0, lex_token, multispace0))(input)
}

pub struct Lexer;

impl Lexer {
    pub fn lex_tokens(bytes: &[u8]) -> IResult<&[u8], Vec<Token>> {
        lex_tokens(bytes)
            .map(|(slice, result)| (slice, [&result[..], &vec![Token::EOF][..]].concat()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer1() {
        let input = &b"=+(){},;"[..];
        let (_, result) = Lexer::lex_tokens(input).unwrap();
        let expected_results = vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn test_lexer2() {
        let input = "let five = 5;\
            let ten = 10;\
            let add = fn(x, y) {\
                x + y;\
            };\
            let result = add(five, ten);"
            .as_bytes();

        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::IntLiteral(5),
            Token::SemiColon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::IntLiteral(10),
            Token::SemiColon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::SemiColon,
            Token::RBrace,
            Token::SemiColon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn test_lexer3() {
        let input = "if (a == 10) {\
                return a;\
             } else if (a != 20) {\
                return !a;\
            } else if (a > 20) {\
                return -30 / 40 * 50;\
            } else if (a < 30) {\
                return true;\
            }\
            return false;\
            "
            .as_bytes();

        let (_, result) = Lexer::lex_tokens(input).unwrap();

        let expected_results = vec![
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::Equal,
            Token::IntLiteral(10),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::NotEqual,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Not,
            Token::Ident("a".to_owned()),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::GreaterThan,
            Token::IntLiteral(20),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Minus,
            Token::IntLiteral(30),
            Token::Divide,
            Token::IntLiteral(40),
            Token::Multiply,
            Token::IntLiteral(50),
            Token::SemiColon,
            Token::RBrace,
            Token::Else,
            Token::If,
            Token::LParen,
            Token::Ident("a".to_owned()),
            Token::LessThan,
            Token::IntLiteral(30),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::BoolLiteral(true),
            Token::SemiColon,
            Token::RBrace,
            Token::Return,
            Token::BoolLiteral(false),
            Token::SemiColon,
            Token::EOF,
        ];

        assert_eq!(result, expected_results);
    }

    #[test]
    fn string_literals() {
        let (_, result) = Lexer::lex_tokens(&b"\"foobar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foobar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens(&b"\"foo bar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo bar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens(&b"\"foo\nbar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\nbar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens(&b"\"foo\tbar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\tbar".to_owned()), Token::EOF]
        );

        let (_, result) = Lexer::lex_tokens(&b"\"foo\\\"bar\""[..]).unwrap();
        assert_eq!(
            result,
            vec![Token::StringLiteral("foo\"bar".to_owned()), Token::EOF]
        );

        let (_, result) =
            Lexer::lex_tokens(&b"\"foo\\\"bar with \xf0\x9f\x92\x96 emojis\""[..]).unwrap();
        assert_eq!(
            result,
            vec![
                Token::StringLiteral("foo\"bar with 💖 emojis".to_owned()),
                Token::EOF
            ]
        );
    }

    #[test]
    fn id_with_numbers() {
        let (_, result) = Lexer::lex_tokens(&b"hello2 hel301oo120"[..]).unwrap();
        let expected = vec![
            Token::Ident("hello2".to_owned()),
            Token::Ident("hel301oo120".to_owned()),
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn array_tokens() {
        let (_, result) = Lexer::lex_tokens(&b"[1, 2];"[..]).unwrap();
        let expected = vec![
            Token::LBracket,
            Token::IntLiteral(1),
            Token::Comma,
            Token::IntLiteral(2),
            Token::RBracket,
            Token::SemiColon,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn hash_tokens() {
        let (_, result) = Lexer::lex_tokens(&b"{\"hello\": \"world\"}"[..]).unwrap();
        let expected = vec![
            Token::LBrace,
            Token::StringLiteral("hello".to_owned()),
            Token::Colon,
            Token::StringLiteral("world".to_owned()),
            Token::RBrace,
            Token::EOF,
        ];
        assert_eq!(result, expected);
    }
}
