use std::{iter::Peekable, vec};

use crate::parser::ast::{
    BinaryExpression, Expression, FunctionExpression, IfExpression, Literal, Statement,
};
use anyhow::{anyhow, ensure, Result};
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
    pub fn parse_statement(
        self: Token,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Statement> {
        match self {
            Token::Let => {
                //find a way to peak them consume the iterator;
                let name = tokens
                    .next_if(Token::is_identifier)
                    .and_then(Token::into_identifier)
                    .ok_or(anyhow!(
                        "Expected token to be {:?}, but got {:?} instead",
                        Token::Identifier(SmolStr::default()),
                        tokens.peek(),
                    ))?;

                ensure!(
                    tokens.next_if_eq(&Token::Assign).is_some(),
                    "Expected assign after identifier found: {:?}",
                    tokens.peek()
                );
                let expression = tokens
                    .next()
                    .ok_or(anyhow!("Missing next token"))?
                    .parse_expression(tokens, 1)?;
                ensure!(
                    tokens.next_if_eq(&Token::Semicolon).is_some(),
                    "Expected semicolumn at the end of statement but found: {:?}",
                    tokens.peek()
                );
                Ok(Statement::Let {
                    identifier: name,
                    value: Box::new(expression),
                })
            }
            Token::Return => {
                let expression = tokens
                    .next()
                    .ok_or(anyhow!("Missing next token"))?
                    .parse_expression(tokens, 1)?;
                ensure!(
                    tokens.next_if_eq(&Token::Semicolon).is_some(),
                    "Expected semicolumn at the end of statement but found: {:?}",
                    tokens.peek()
                );
                Ok(Statement::Return(Box::new(expression)))
            }
            _ => {
                let expression = self.parse_expression(tokens, 1)?;
                ensure!(
                    tokens.next_if_eq(&Token::Semicolon).is_some(),
                    "Expected semicolumn at the end of statement but found: {:?}",
                    tokens.peek()
                );
                Ok(Statement::Expression(Box::new(expression)))
            }
        }
    }

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

            //TODO: how can I assign left at the same time it is being moved?
            left = expression_type(BinaryExpression {
                lhs: Box::new(left),
                rhs: Box::new(right),
            });
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
            Token::LParen => {
                let expression = tokens
                    .next()
                    .ok_or(anyhow!("Missing next token"))?
                    .parse_expression(tokens, 0)?;

                ensure!(
                    tokens.next_if_eq(&Token::RParen).is_some(),
                    "Missing closing parenthesis. Found {:?}",
                    tokens.peek()
                );

                Ok(expression)
            }
            Token::If => {
                ensure!(
                    tokens.next_if_eq(&Token::LParen).is_some(),
                    "Missing opening parem. Found {:?}",
                    tokens.peek()
                );

                let condition = tokens
                    .next()
                    .ok_or(anyhow!("Missing next token"))?
                    .parse_expression(tokens, 0)?;

                ensure!(
                    tokens.next_if_eq(&Token::RParen).is_some(),
                    "Missing closing parem. Found {:?}",
                    tokens.peek()
                );
                ensure!(
                    tokens.next_if_eq(&Token::LBrace).is_some(),
                    "Missing opening brace. Found {:?}",
                    tokens.peek()
                );
                let consequence = Token::parse_block(tokens)?;

                if tokens.next_if_eq(&Token::Else).is_none() {
                    return Ok(Expression::If(IfExpression {
                        condition: Box::new(condition),
                        consequence,
                        alternative: None,
                    }));
                }

                //TODO: Not sure if I should return errors here
                ensure!(
                    tokens.next_if_eq(&Token::LBrace).is_some(),
                    "Missing opening brace. Found {:?}",
                    tokens.peek()
                );
                let alternative = Some(Token::parse_block(tokens)?);
                Ok(Expression::If(IfExpression {
                    condition: Box::new(condition),
                    consequence,
                    alternative,
                }))
            }
            Token::Function => {
                ensure!(
                    tokens.next_if_eq(&Token::LParen).is_some(),
                    "Missing opening parem. Found {:?}",
                    tokens.peek()
                );

                let parameters = tokens
                    .next_if_eq(&Token::RParen)
                    .map_or_else(|| Token::parse_function_parameters(tokens).ok(), |_| None);
                //TODO: handle parse_function_parameters error

                ensure!(
                    tokens.next_if_eq(&Token::LBrace).is_some(),
                    "Missing opening brace. Found {:?}",
                    tokens.peek()
                );

                let body = Token::parse_block(tokens)?;
                Ok(Expression::Function(FunctionExpression {
                    parameters,
                    body,
                }))
            }
            _ => Err(anyhow!("Cannot parse expression starting with {:?}", self)),
        }
    }

    fn parse_function_parameters(
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Vec<SmolStr>> {
        //all identifiers, create own struct
        let mut parameters = vec![];

        parameters.push(
            tokens
                .next()
                .ok_or(anyhow!("Missing next token"))?
                .try_into_identifier()
                .map_err(|token| anyhow!("Expected identifier but found: {:?}", token))?,
        );

        while tokens.next_if_eq(&Token::Comma).is_some() {
            parameters.push(
                tokens
                    .next()
                    .ok_or(anyhow!("Missing next token"))?
                    .try_into_identifier()
                    .map_err(|token| anyhow!("Expected identifier but found: {:?}", token))?,
            );
        }

        ensure!(
            tokens.next_if_eq(&Token::RParen).is_some(),
            "Missing closing parenthesis. Found {:?}",
            tokens.peek()
        );

        Ok(parameters)
    }

    fn parse_block(tokens: &mut Peekable<impl Iterator<Item = Token>>) -> Result<Vec<Statement>> {
        let mut statements = vec![];

        while tokens.peek().filter(|x| x != &&Token::RBrace).is_some() {
            let statement = tokens
                .next()
                .ok_or(anyhow!("Missing next token"))?
                .parse_statement(tokens)?;
            statements.push(statement);
        }

        ensure!(
            tokens.next_if_eq(&Token::RBrace).is_some(),
            "Missing closing brace. Found {:?}",
            tokens.peek()
        );

        Ok(statements)
    }

    //TODO: find why I can't wrap the expressions directly using Some()?
    #[inline]
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

    #[inline]
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
