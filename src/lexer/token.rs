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
    // pub fn parse_prefix<'a>(
    //     &'a self,
    //     expression: &'a Expression,
    // ) -> impl Fn(&'a Expression) -> Expression {
    //     // Some(Expression::Literal(Literal::Nill))
    //     move |x| match self {
    //         Token::Minus => Expression::Literal(Literal::Int(-1)),
    //         _ => todo!(),
    //     }
    //     // todo!()
    // }
    //add precedence.
    pub fn parse_prefix(&self) -> Option<Expression> {
        match self {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
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
