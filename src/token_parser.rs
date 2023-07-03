use anyhow::{anyhow, ensure, Result};
use std::{
    iter::Peekable,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

use crate::{
    ast::{
        BinaryExpression, BlockStatement, CallExpression, Expression, FunctionExpression,
        IfExpression, Literal, Statement, UnaryExpression, UnaryOperator,
    },
    token::{Identifier, Token},
};

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
                let identifier = self.try_next().and_then(TryInto::try_into)?;
                self.try_eat(&Token::Assign)?;
                let expression = self
                    .try_next()
                    .and_then(|token| self.parse_expression(token, 0))?;
                self.try_eat(&Token::Semicolon)?;
                Ok(Statement::Let {
                    identifier,
                    value: Box::new(expression),
                })
            }
            Token::Return => {
                let expression = self
                    .try_next()
                    .and_then(|token| self.parse_expression(token, 0))?;
                self.try_eat(&Token::Semicolon)?;
                Ok(Statement::Return(Box::new(expression)))
            }
            _ => {
                let expression = self.parse_expression(token, 0)?;
                //intentionally ignore the error as the semicolon is optional
                self.try_eat(&Token::Semicolon).ok();
                Ok(Statement::Expression(Box::new(expression)))
            }
        }
    }

    fn parse_expression(&mut self, current_token: Token, precedence: u8) -> Result<Expression> {
        let mut left = self.parse_prefix(current_token)?;

        while let Some(token) =
            self.next_if(|x| x != &Token::Semicolon && precedence < x.precedence())
        {
            if let Some(operator) = token.binary_expression_type() {
                let right = self
                    .try_next()
                    .and_then(|exp| self.parse_expression(exp, token.precedence()))?;
                left = Expression::BinaryExp(BinaryExpression {
                    operator,
                    lhs: Box::new(left),
                    rhs: Box::new(right),
                });
            } else if matches!(token, Token::LParen) {
                //WTF: how can I assign left at the same time it is being moved?
                left = Expression::Call(CallExpression {
                    function: left.boxed(),
                    arguments: self.parse_call_arguments()?,
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    #[inline]
    fn parse_prefix(&mut self, token: Token) -> Result<Expression> {
        match token {
            Token::Identifier(name) => Ok(Expression::Identifier(name)),
            Token::Int(value) => Ok(Expression::Literal(Literal::Int(value))),
            Token::True => Ok(Literal::True.into()),
            Token::False => Ok(Literal::False.into()),
            Token::String(value) => Ok(Expression::Literal(Literal::String(value))),
            Token::Nil => Ok(Literal::Nil.into()),
            Token::Bang => self.parse_unary_expression(UnaryOperator::Not),
            Token::Minus => self.parse_unary_expression(UnaryOperator::Minus),
            Token::LParen => self.parse_grouped_expression(),
            Token::If => self.parse_if_expression(),
            Token::Function => self.parse_fn_expression(),
            _ => Err(anyhow!("Cannot parse expression starting with {:?}", token)),
        }
    }

    #[inline]
    fn parse_grouped_expression(&mut self) -> Result<Expression> {
        let expression = self
            .try_next()
            .and_then(|exp| self.parse_expression(exp, 0))?;
        self.try_eat(&Token::RParen)?;
        Ok(expression)
    }

    #[inline]
    fn parse_unary_expression(&mut self, operator: UnaryOperator) -> Result<Expression> {
        let exp = self
            .try_next()
            .and_then(|token| self.parse_expression(token, 6))?;
        Ok(Expression::UnaryExpression(UnaryExpression {
            operator,
            value: Box::new(exp),
        }))
    }

    #[inline]
    fn parse_if_expression(&mut self) -> Result<Expression> {
        self.try_eat(&Token::LParen)?;
        let condition = self
            .try_next()
            .and_then(|token| self.parse_expression(token, 0))?;
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

        self.try_eat(&Token::LBrace)?;
        let alternative = Some(self.parse_block()?);
        Ok(Expression::If(IfExpression {
            condition: Box::new(condition),
            consequence,
            alternative,
        }))
    }

    #[inline]
    fn parse_fn_expression(&mut self) -> Result<Expression> {
        self.try_eat(&Token::LParen)?;
        let parameters = self.parse_function_parameters()?;
        self.try_eat(&Token::LBrace)?;
        let body = self.parse_block()?;
        Ok(Expression::Function(FunctionExpression {
            parameters,
            body,
        }))
    }

    #[inline]
    fn parse_block(&mut self) -> Result<BlockStatement> {
        let mut statements = vec![];

        while let Some(token) = self.next_if(|x| x != &Token::RBrace) {
            statements.push(self.parse_statement(token)?);
        }

        self.try_eat(&Token::RBrace)?;
        Ok(BlockStatement::new(statements))
    }

    #[inline]
    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>> {
        let mut arguments = vec![];
        if self.next_if_eq(&Token::RParen).is_some() {
            return Ok(arguments);
        }

        self.try_next()
            .and_then(|token| self.parse_expression(token, 0))
            .map(|exp| arguments.push(exp))?;

        while self.next_if_eq(&Token::Comma).is_some() {
            self.try_next()
                .and_then(|token| self.parse_expression(token, 0))
                .map(|exp| arguments.push(exp))?
        }

        self.try_eat(&Token::RParen)?;
        Ok(arguments)
    }

    #[inline]
    fn parse_function_parameters(&mut self) -> Result<Vec<Identifier>> {
        let mut parameters = vec![];

        if self.next_if_eq(&Token::RParen).is_some() {
            return Ok(parameters);
        }

        parameters.push(self.try_next()?.try_into()?);

        while self.next_if_eq(&Token::Comma).is_some() {
            parameters.push(self.try_next()?.try_into()?);
        }

        self.try_eat(&Token::RParen)?;
        Ok(parameters)
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
}
