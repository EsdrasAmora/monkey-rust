use crate::parser::ast::{BinaryExpression, Expression};
use anyhow::Result;
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Identifier(SmolStr),
    //Literals
    Int(i64),
    String(SmolStr), // a Cow may would be better here.
    True,
    False,
    Nil,
    //Operators
    Eq,
    NotEq,
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    //Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    //Keywords
    Function,
    Let,
    If,
    Else,
    Return,
}

impl Token {
    //TODO: find why I can't wrap the expressions directly using Some()?
    #[inline]
    pub fn binary_expression_type(&self) -> Option<fn(BinaryExpression) -> Expression> {
        let expression_type = match self {
            Token::Plus => Expression::Add,
            Token::Minus => Expression::Sub,
            Token::Slash => Expression::Div,
            Token::Asterisk => Expression::Mul,
            Token::Eq => Expression::Eq,
            Token::NotEq => Expression::NotEq,
            Token::Lt => Expression::Lt,
            Token::Gt => Expression::Gt,
            _ => return None,
        };
        Some(expression_type)
    }

    #[inline]
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Eq | Token::NotEq => 2,
            Token::Gt | Token::Lt => 3,
            Token::Plus | Token::Minus => 4,
            Token::Slash | Token::Asterisk => 5,
            Token::LParen => 7,
            _ => 0,
        }
    }

    #[inline]
    pub fn try_into_identifier(self) -> Result<SmolStr, Self> {
        match self {
            Token::Identifier(name) => Ok(name),
            _ => Err(self),
        }
    }

    #[inline]
    pub fn into_identifier(self) -> Option<SmolStr> {
        match self {
            Token::Identifier(name) => Some(name),
            _ => None,
        }
    }

    #[inline]
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }
}
