use crate::token::{Identifier, Token};
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
    Identifier(Identifier),
    BinaryExp(BinaryExpression),
    UnaryExpression(UnaryExpression),
    If(IfExpression),
    IndexExpression(IndexExpression),
    Function(FunctionExpression),
    Call(CallExpression),
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum BinaryOperator {
    Eq,
    NotEq,
    Lt,
    Lte,
    Gt,
    Gte,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub enum UnaryOperator {
    Not,
    Minus,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct UnaryExpression {
    pub value: Box<Expression>,
    pub operator: UnaryOperator,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct CallExpression {
    pub arguments: Vec<Expression>,
    pub function: Box<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct IndexExpression {
    pub container: Box<Expression>,
    pub index: Box<Expression>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
pub struct FunctionExpression {
    pub parameters: Vec<Identifier>,
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
    pub operator: BinaryOperator,
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
    Array(Vec<Expression>),
    Hash(Vec<(Expression, Expression)>),
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
    #[inline]
    pub fn binary_expression_type(&self) -> Option<BinaryOperator> {
        match self {
            Token::Plus => Some(BinaryOperator::Add),
            Token::Minus => Some(BinaryOperator::Sub),
            Token::Slash => Some(BinaryOperator::Div),
            Token::Asterisk => Some(BinaryOperator::Mul),
            Token::Eq => Some(BinaryOperator::Eq),
            Token::NotEq => Some(BinaryOperator::NotEq),
            Token::Lt => Some(BinaryOperator::Lt),
            Token::Lte => Some(BinaryOperator::Lte),
            Token::Gt => Some(BinaryOperator::Gt),
            Token::Gte => Some(BinaryOperator::Gte),
            _ => None,
        }
    }
}
