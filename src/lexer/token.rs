use crate::parser::ast::{Expression, Literal};

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
    pub fn parse_prefix(&self, expression: Expression) -> Expression {
        Expression::Literal(Literal::Nill)
    }
    pub fn parse_infix(&self) -> Expression {
        Expression::Literal(Literal::Nill)
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
