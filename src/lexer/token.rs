use std::iter::Peekable;

use crate::parser::ast::{BinaryExpression, Expression, Literal};
use anyhow::{anyhow, Result};
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
    pub fn parse_expression(
        self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
        precedence: u8,
    ) -> Result<Expression> {
        let mut left = self.parse_prefix(tokens)?;

        while tokens
            .peek()
            .filter(|x| x != &&Token::Semicolon && precedence < x.precedence())
            .is_some()
        {
            let token = tokens.next().expect("Already peeked");
            //currently does not autoformat lmao: https://github.com/rust-lang/rustfmt/issues/4914
            let Some(expression_type) = token.binary_expression_type() else {
                break;
            };

            let right = token.parse_infix(tokens)?;

            left = expression_type(BinaryExpression {
                lhs: Box::new(left),
                rhs: Box::new(right),
            })
        }

        Ok(left)
    }

    fn parse_prefix(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Expression> {
        match self {
            Token::Identifier(name) => Ok(Expression::Identifier(name.clone())),
            Token::Int(value) => Ok(Expression::Literal(Literal::Int(*value))),
            Token::True => Ok(Literal::True.into()),
            Token::False => Ok(Literal::False.into()),
            Token::String(value) => Ok(Expression::Literal(Literal::String(value.clone()))),
            Token::Nil => Ok(Literal::Nil.into()),
            Token::Bang => {
                let right = tokens
                    .next()
                    .ok_or(anyhow!("Missing next token"))?
                    .parse_expression(tokens, 6)?;
                Ok(Expression::Not(Box::new(right)))
            }
            Token::Minus => {
                let right = tokens
                    .next()
                    .ok_or(anyhow!("Missing next token"))?
                    .parse_expression(tokens, 6)?;
                Ok(Expression::Oposite(Box::new(right)))
            }
            Token::LParen => todo!(),
            Token::If => todo!(),
            Token::Function => todo!(),
            _ => Err(anyhow!("Cannot parse expression starting with {:?}", self)),
        }
    }

    //TODO: why I can't wrap the expressions directly using Some()?
    fn binary_expression_type(&self) -> Option<fn(BinaryExpression) -> Expression> {
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

    fn parse_infix(
        &self,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Expression> {
        let precedence = self.precedence();
        let next = tokens.next().ok_or(anyhow!("any"))?;
        next.parse_expression(tokens, precedence)
    }

    #[inline]
    fn precedence(&self) -> u8 {
        match self {
            Token::Eq | Token::NotEq => 2,
            Token::Gt | Token::Lt => 3,
            Token::Plus | Token::Minus => 4,
            Token::Slash | Token::Asterisk => 5,
            Token::LParen => 6,
            _ => {
                println!("No precedence for {:?}", self);
                0
            }
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
