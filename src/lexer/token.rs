use crate::parser::ast::{BinaryExpression, Expression};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Identifier(Identifier),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Identifier(SmolStr);

impl Identifier {
    #[inline]
    pub fn new(name: SmolStr) -> Self {
        Self(name)
    }

    #[inline]
    pub fn into_inner(self) -> SmolStr {
        self.0
    }
}

impl TryFrom<Token> for Identifier {
    type Error = Token;

    #[inline]
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(token),
        }
    }
}

impl From<Identifier> for SmolStr {
    #[inline]
    fn from(name: Identifier) -> Self {
        name.into_inner()
    }
}

impl From<SmolStr> for Identifier {
    #[inline]
    fn from(name: SmolStr) -> Self {
        Self::new(name)
    }
}

impl Token {
    //WTF: why I can't wrap the expressions directly using Some()?
    #[inline]
    pub fn binary_expression_type(&self) -> Option<fn(BinaryExpression) -> Expression> {
        Some(match self {
            Token::Plus => Expression::Add,
            Token::Minus => Expression::Sub,
            Token::Slash => Expression::Div,
            Token::Asterisk => Expression::Mul,
            Token::Eq => Expression::Eq,
            Token::NotEq => Expression::NotEq,
            Token::Lt => Expression::Lt,
            Token::Gt => Expression::Gt,
            _ => return None,
        })
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
    pub fn into_identifier(self) -> Option<Identifier> {
        match self {
            Token::Identifier(name) => Some(name),
            _ => None,
        }
    }

    #[inline]
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }

    #[inline]
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::Int(_)
                | Token::String(_)
                | Token::True
                | Token::False
                | Token::Nil
                | Token::Identifier(_)
        )
    }
}
