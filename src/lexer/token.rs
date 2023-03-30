use std::iter::Peekable;

use crate::parser::ast::{Expression, Literal};
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Illegal,
    Identifier(String),
    //Literals
    Int(i64),
    String(String),
    True,
    False,
    Nill,
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
    pub fn parse_expression(
        self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
        precedence: u8,
    ) -> Result<Expression> {
        let left_expression = self.parse_prefix(tokens).ok_or(anyhow!(
            "Cannot parse an expression starting with {:?}",
            self
        ))?;

        Ok(left_expression)
    }

    //add precedence.
    fn parse_prefix(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Option<Expression> {
        match self {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            Token::Int(value) => Some(Expression::Literal(Literal::Int(*value))),
            Token::Bang => {
                //TODO: fix error propagation;
                let right = tokens.next()?.parse_expression(tokens, 6).ok()?;
                Some(Expression::Not(Box::new(right)))
            }
            Token::Minus => {
                let right = tokens.next()?.parse_expression(tokens, 6).ok()?;
                Some(Expression::Oposite(Box::new(right)))
            }
            _ => None,
        }
    }
    fn parse_infix(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
        expression: Expression,
    ) -> Option<Expression> {
        Some(Expression::Literal(Literal::Nill))
    }

    pub fn into_identifier(self) -> Option<String> {
        match self {
            Token::Identifier(name) => Some(name),
            _ => None,
        }
    }
    pub fn is_identifier(&self) -> bool {
        matches!(self, Token::Identifier(_))
    }
}
