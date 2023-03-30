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
    Oposite(Box<Expression>),
    Not(Box<Expression>),
    Eq(BynaryExpression),
    NotEq(BynaryExpression),
    LT(BynaryExpression),
    GT(BynaryExpression),
    Add(BynaryExpression),
    Sub(BynaryExpression),
    Mul(BynaryExpression),
    Div(BynaryExpression),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BynaryExpression {
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}
//maybe precedence should be a method on Token.
impl Expression {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }

    fn precedence(&self) -> u8 {
        match self {
            Expression::Literal(_) | Expression::Identifier(_) => 0,
            Expression::Oposite(_) | Expression::Not(_) => 1,
            Expression::Eq(_) | Expression::NotEq(_) => 2,
            Expression::LT(_) | Expression::GT(_) => 3,
            Expression::Mul(_) | Expression::Div(_) => 4,
            Expression::Add(_) | Expression::Sub(_) => 5,
        }
    }
}

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
