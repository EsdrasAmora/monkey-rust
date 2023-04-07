use crate::token::{Identifier, Token};
use either::Either;
use serde::Serialize;
use smol_str::SmolStr;

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum Statement {
    Let {
        identifier: Identifier,
        value: Box<Expression>,
    },
    Return(Box<Expression>),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct UnaryExpression(pub Box<Expression>);

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct CallExpression {
    pub arguments: Option<Vec<Expression>>,
    pub function: Either<SmolStr, FunctionExpression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct FunctionExpression {
    pub parameters: Option<Vec<Identifier>>,
    pub body: BlockStatement,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Default)]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
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

impl Token {
    // WTF: why I can't wrap each branch using Some()?
    // implemented this token method here so it does not depend on the ast.
    #[inline]
    pub fn binary_expression_type(&self) -> Option<fn(BinaryExpression) -> Expression> {
        Some(match self {
            Token::Plus => Expression::Add,
            Token::Minus => Expression::Sub,
            Token::Slash => Expression::Div,
            Token::Asterisk => Expression::Mul,
            Token::Eq => Expression::Eq,
            Token::NotEq => Expression::NotEq,
            Token::Lt => Expression::Lt,
            Token::Gt => Expression::Gt,
            _ => return None,
        })
    }
}
