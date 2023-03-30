use std::iter::Peekable;

use crate::parser::ast::{BinaryExpression, Expression, Literal};
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
        let mut left = self.parse_prefix(tokens).ok_or(anyhow!(
            "Cannot parse an expression starting with {:?}",
            self
        ))?;

        while tokens
            .peek()
            .filter(|x| x != &&Token::Semicolon && precedence < x.precedence())
            .is_some()
        {
            let token = tokens.next().expect("Already peeked");
            let Some(expression_type) = token.binary_expression_type() else {
                break;
            };

            let right = token
                .parse_infix(tokens)
                .ok_or(anyhow!("somethign else {:?}", self))?;

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
    ) -> Option<Expression> {
        match self {
            Token::Identifier(name) => Some(Expression::Identifier(name.clone())),
            Token::Int(value) => Some(Expression::Literal(Literal::Int(*value))),
            Token::True => Some(Literal::True.into()),
            Token::False => Some(Literal::False.into()),
            Token::Bang => {
                //TODO: fix error propagation;
                let right = tokens.next()?.parse_expression(tokens, 6).ok()?;
                Some(Expression::Not(Box::new(right)))
            }
            Token::Minus => {
                let right = tokens.next()?.parse_expression(tokens, 6).ok()?;
                Some(Expression::Oposite(Box::new(right)))
            }
            Token::LParen => todo!(),
            Token::If => todo!(),
            Token::Function => todo!(),
            _ => None,
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
    ) -> Option<Expression> {
        let precedence = self.precedence();
        let next = tokens.next()?;
        next.parse_expression(tokens, precedence).ok()
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
    pub fn into_identifier(self) -> Option<String> {
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
