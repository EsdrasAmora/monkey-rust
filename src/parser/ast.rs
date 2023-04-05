use either::Either;
use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

use crate::lexer::token::Identifier;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Let {
        identifier: Identifier,
        value: Box<Expression>,
    },
    Return(Box<Expression>),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(SmolStr),
    Oposite(UnaryExpression),
    Not(UnaryExpression),
    Eq(BinaryExpression),
    NotEq(BinaryExpression),
    Lt(BinaryExpression),
    Gt(BinaryExpression),
    Add(BinaryExpression),
    Sub(BinaryExpression),
    Mul(BinaryExpression),
    Div(BinaryExpression),
    If(IfExpression),
    Function(FunctionExpression),
    Call(CallExpression),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression(pub Box<Expression>);

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct CallExpression {
    pub arguments: Option<Vec<Expression>>,
    pub function: Either<SmolStr, FunctionExpression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct FunctionExpression {
    pub parameters: Option<Vec<Identifier>>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
pub struct BlockStatement(pub Vec<Statement>);

impl BlockStatement {
    pub fn new(value: Vec<Statement>) -> Self {
        Self(value)
    }
}

impl Expression {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Literal {
    Int(i64),
    String(SmolStr),
    True,
    False,
    Nil,
}

impl Literal {
    pub fn into_exp(self) -> Box<Expression> {
        Expression::Literal(self).boxed()
    }
}

impl From<Literal> for Expression {
    fn from(literal: Literal) -> Self {
        Expression::Literal(literal)
    }
}
