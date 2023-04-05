use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
    vec,
};

use crate::{
    lexer::token::Identifier,
    parser::ast::{
        BinaryExpression, CallExpression, Expression, FunctionExpression, IfExpression, Literal,
        Statement, UnaryExpression,
    },
};
use anyhow::{anyhow, bail, ensure, Result};
use either::Either::{Left, Right};
use smol_str::SmolStr;
use std::vec::IntoIter;

use crate::lexer::token::Token;

use super::ast::BlockStatement;

pub struct TokenParser(Peekable<IntoIter<Token>>);

impl Deref for TokenParser {
    type Target = Peekable<IntoIter<Token>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for TokenParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl TokenParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self(tokens.into_iter().peekable())
    }

    pub fn parse_statement(&mut self, token: Token) -> Result<Statement> {
        match token {
            Token::Let => {
                //find a way to peak them consume the iterator;
                let name = self
                    .next_if(Token::is_identifier)
                    .and_then(Token::into_identifier)
                    .ok_or(anyhow!(
                        "Expected token {:?} but found {:?}",
                        Token::Identifier(Identifier::new(SmolStr::default())),
                        self.peek(),
                    ))?;

                self.try_eat(&Token::Assign)?;
                let fixme = self.try_next()?;
                let expression = self.parse_expression(fixme, 0)?;
                self.try_eat(&Token::Semicolon)?;
                Ok(Statement::Let {
                    identifier: name,
                    value: Box::new(expression),
                })
            }
            Token::Return => {
                let fixme = self.try_next()?;
                let expression = self.parse_expression(fixme, 0)?;
                self.try_eat(&Token::Semicolon)?;
                Ok(Statement::Return(Box::new(expression)))
            }
            _ => {
                let expression = self.parse_expression(token, 0)?;
                self.try_eat(&Token::Semicolon)?;
                Ok(Statement::Expression(Box::new(expression)))
            }
        }
    }

    fn parse_expression(&mut self, current_token: Token, precedence: u8) -> Result<Expression> {
        let mut left = self.parse_prefix(current_token)?;

        while self
            .peek()
            .filter(|x| x != &&Token::Semicolon && precedence < x.precedence())
            .is_some()
        {
            let token = self.next().expect("Already peeked");

            if let Some(expression_type) = token.binary_expression_type() {
                let right = self.parse_infix(&token)?;
                //WTF: how can I assign left at the same time it is being moved?
                left = expression_type(BinaryExpression {
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                });
                continue;
            }

            if matches!(token, Token::LParen) {
                let function = match left {
                    Expression::Identifier(name) => Left(name),
                    Expression::Function(function) => Right(function),
                    _ => bail!("Expected identifier or function but found {:?}", left),
                };
                left = Expression::Call(CallExpression {
                    function,
                    arguments: self.parse_call_arguments()?,
                });
                continue;
            }
            break;
        }

        Ok(left)
    }

    #[inline]
    fn try_next(&mut self) -> Result<Token> {
        self.next()
            .ok_or(anyhow!("Unexpected end of file, no more tokens"))
    }

    #[inline]
    fn try_eat(&mut self, expect: &Token) -> Result<()> {
        ensure!(
            self.next_if_eq(expect).is_some(),
            "Expected token {:?} but found {:?}",
            expect,
            self.peek()
        );
        Ok(())
    }

    #[inline]
    fn parse_prefix(&mut self, token: Token) -> Result<Expression> {
        match token {
            Token::Identifier(name) => Ok(Expression::Identifier(name.into_inner())),
            Token::Int(value) => Ok(Expression::Literal(Literal::Int(value))),
            Token::True => Ok(Literal::True.into()),
            Token::False => Ok(Literal::False.into()),
            Token::String(value) => Ok(Expression::Literal(Literal::String(value))),
            Token::Nil => Ok(Literal::Nil.into()),
            Token::Bang => Ok(Expression::Not(self.parse_unary_expression()?)),
            Token::Minus => Ok(Expression::Oposite(self.parse_unary_expression()?)),
            Token::LParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_fn_expression(),
            _ => Err(anyhow!("Cannot parse expression starting with {:?}", token)),
        }
    }

    #[inline]
    fn parse_infix(&mut self, token: &Token) -> Result<Expression> {
        let precedence = token.precedence();
        let next = self.try_next()?;
        self.parse_expression(next, precedence)
    }

    #[inline]
    fn parse_grouped_expression(&mut self) -> Result<Expression> {
        let expression = self.try_next()?;
        let expression = self.parse_expression(expression, 0)?;

        self.try_eat(&Token::RParen)?;
        Ok(expression)
    }

    #[inline]
    fn parse_unary_expression(&mut self) -> Result<UnaryExpression> {
        let right = self.try_next()?;
        let right = self.parse_expression(right, 6)?;
        Ok(UnaryExpression(Box::new(right)))
    }

    #[inline]
    fn parse_if_expression(&mut self) -> Result<Expression> {
        self.try_eat(&Token::LParen)?;
        let fixme = self.try_next()?;
        let condition = self.parse_expression(fixme, 0)?;

        self.try_eat(&Token::RParen)?;
        self.try_eat(&Token::LBrace)?;
        let consequence = self.parse_block()?;

        if self.next_if_eq(&Token::Else).is_none() {
            return Ok(Expression::If(IfExpression {
                condition: Box::new(condition),
                consequence,
                alternative: None,
            }));
        }

        //TODO: Not sure if I should return errors here
        self.try_eat(&Token::LBrace)?;
        let alternative = Some(self.parse_block()?);
        Ok(Expression::If(IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    fn parse_block(&mut self) -> Result<BlockStatement> {
        let mut statements = vec![];

        while self.peek().filter(|x| x != &&Token::RBrace).is_some() {
            let fixme = self.try_next()?;
            let statement = self.parse_statement(fixme)?;

            statements.push(statement);
        }

        self.try_eat(&Token::RBrace)?;
        Ok(BlockStatement::new(statements))
    }

    fn parse_call_arguments(&mut self) -> Result<Option<Vec<Expression>>> {
        if self.next_if_eq(&Token::RParen).is_some() {
            return Ok(None);
        }

        let mut arguments = vec![];
        let fixme = self.try_next()?;
        arguments.push(self.parse_expression(fixme, 0)?);

        while self.next_if_eq(&Token::Comma).is_some() {
            let fixme = self.try_next()?;
            arguments.push(self.parse_expression(fixme, 0)?);
        }

        self.try_eat(&Token::RParen)?;
        Ok(Some(arguments))
    }

    fn parse_fn_expression(&mut self) -> Result<Expression> {
        self.try_eat(&Token::LParen)?;

        let parameters = self
            .next_if_eq(&Token::RParen)
            .map_or_else(|| self.parse_function_parameters().ok(), |_| None);
        //TODO: handle parse_function_parameters error

        self.try_eat(&Token::LBrace)?;

        let body = self.parse_block()?;
        Ok(Expression::Function(FunctionExpression {
            parameters,
            body,
        }))
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>> {
        //all identifiers, create own struct
        let mut parameters = vec![];
        parameters.push(
            self.try_next()?
                .try_into()
                .map_err(|token| anyhow!("Expected identifier but found: {:?}", token))?,
        );

        while self.next_if_eq(&Token::Comma).is_some() {
            parameters.push(
                self.try_next()?
                    .try_into()
                    .map_err(|token| anyhow!("Expected identifier but found: {:?}", token))?,
            );
        }

        self.try_eat(&Token::RParen)?;
        Ok(parameters)
    }
}
