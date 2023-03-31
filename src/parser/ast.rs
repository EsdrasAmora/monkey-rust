use serde::{Deserialize, Serialize};
use smol_str::SmolStr;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Statement {
    Let {
        identifier: SmolStr,
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
}

type UnaryExpression = Box<Expression>;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
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
