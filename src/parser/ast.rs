use crate::lexer::token::Token;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Let {
        identifier: String,
        value: Box<Expression>,
    },
    Return(Box<Expression>),
    Expression(Box<Expression>),
}

//TODO: create a builder for this.
#[derive(Debug, Clone, Eq, PartialEq)]
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BinaryExpression {
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}
//maybe precedence should be a method on Token.
impl Expression {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

// impl TryFrom<Token> for Expression {
//     type Error = anyhow::Error;

//     fn from(value: Token) -> Result<Expression, Self::Error> {
//         match value {
//             Token::Identifier(name) => Expression::Identifier(name.clone()),
//             Token::Int(value) => Expression::Literal(Literal::Int(value)),
//             Token::True => Literal::True.into(),
//             Token::False => Literal::False.into(),
//             Token::Nill => Literal::Nill.into(),
//             Token::String(value) => Expression::Literal(Literal::String(value)),
//             // Token::Illegal => todo!(),
//             // Token::Eq => todo!(),
//             // Token::NotEq => todo!(),
//             // Token::Assign => todo!(),
//             // Token::Plus => todo!(),
//             // Token::Minus => todo!(),
//             // Token::Bang => todo!(),
//             // Token::Asterisk => todo!(),
//             // Token::Slash => todo!(),
//             // Token::Lt => todo!(),
//             // Token::Gt => todo!(),
//             // Token::Comma => todo!(),
//             // Token::Semicolon => todo!(),
//             // Token::LParen => todo!(),
//             // Token::RParen => todo!(),
//             // Token::LBrace => todo!(),
//             // Token::RBrace => todo!(),
//             // Token::Function => todo!(),
//             // Token::Let => todo!(),
//             // Token::If => todo!(),
//             // Token::Else => todo!(),
//             // Token::Return => todo!(),
//         }
//     }
// }

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
    True,
    False,
    Nill,
}

impl From<Literal> for Expression {
    fn from(literal: Literal) -> Self {
        Expression::Literal(literal)
    }
}
