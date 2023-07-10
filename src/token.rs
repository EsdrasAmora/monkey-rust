use std::fmt::Display;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Token {
    Illegal,
    Identifier(Identifier),
    //Literals
    Int(i64),
    String(SmolStr),
    True,
    False,
    Nil,
    Dot,
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
    Lte,
    Gt,
    Gte,
    //Delimiters
    Comma,
    Colon,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
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
    pub fn inner(&self) -> SmolStr {
        self.0.clone()
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)?;
        Ok(())
    }
}

impl TryFrom<Token> for Identifier {
    type Error = anyhow::Error;

    #[inline]
    fn try_from(token: Token) -> Result<Self, Self::Error> {
        match token {
            Token::Identifier(name) => Ok(name),
            _ => Err(anyhow!("Expected identifier but found: {:?}", token)),
        }
    }
}

impl Token {
    #[inline]
    pub fn precedence(&self) -> u8 {
        match self {
            Token::Eq | Token::NotEq => 2,
            Token::Gt | Token::Gte | Token::Lt | Token::Lte => 3,
            Token::Plus | Token::Minus => 4,
            Token::Slash | Token::Asterisk => 5,
            Token::LParen => 7,
            Token::LBracket => 8,
            _ => 0,
        }
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
