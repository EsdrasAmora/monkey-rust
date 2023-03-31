use serde::Serialize;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Statement {
    Let {
        identifier: String,
        value: Box<Expression>,
    },
    Return(Box<Expression>),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

impl Expression {
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Literal {
    Int(i64),
    String(String),
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
