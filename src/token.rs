use anyhow::{anyhow, Result};
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
    Lte,
    Gt,
    Gte,
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
    pub fn inner(&self) -> SmolStr {
        self.0.clone()
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

impl From<SmolStr> for Identifier {
    #[inline]
    fn from(name: SmolStr) -> Self {
        Self::new(name)
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
