#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Statement {
    Let {
        indentifier: String,
        value: Box<Expression>,
    },
    Return(Box<Expression>),
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    Negative(Box<Expression>),
    Negate(Box<Expression>),
    Eq(Box<Expression>),
    LT(Box<Expression>),
    GT(Box<Expression>),
    Add(Box<Expression>),
    Sub(Box<Expression>),
    Mul(Box<Expression>),
    Div(Box<Expression>),
}
//maybe precedence should be a method on Token.
impl Expression {
    fn precedence(&self) -> u8 {
        match self {
            Expression::Literal(_) | Expression::Identifier(_) => 0,
            Expression::Negative(_) | Expression::Negate(_) => 1,
            Expression::Eq(_) => 2,
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
