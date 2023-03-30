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
    fn parse_expression(self, precedence: u8) -> Result<Expression> {
        let left_expression = self.parse_prefix().ok_or(anyhow!(
            "Cannot parse an expression starting with {:?}",
            self
        ))?;

        return Ok(left_expression);
    }

    //add precedence.
    pub fn parse_prefix(&self) -> Option<Expression> {
        match self {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            Token::Int(value) => Some(Expression::Literal(Literal::Int(*value))),
            Token::Bang => {
                // tokens.next();
                //aa
                Some(Expression::Negate(Box::new(Expression::Identifier(
                    "a".to_string(),
                ))))
            }
            Token::Minus => {
                // tokens.next();
                //aa
                Some(Expression::Negate(Box::new(Expression::Identifier(
                    "a".to_string(),
                ))))
            }
            _ => None,
        }
    }
    pub fn parse_infix(&self, expression: Expression) -> Option<Expression> {
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
