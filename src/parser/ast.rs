use id_arena::Id;

type NodeId = Id<Node>;

#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    Let { name: String, value: NodeId },
    Return { value: NodeId },
    Literal(Literal),
    Indentifier(String),
    Add(BinaryExpression),
    Sub(BinaryExpression),
    Mul(BinaryExpression),
    Div(BinaryExpression),
}

#[derive(Debug, Eq, PartialEq)]
pub struct BinaryExpression {
    left: NodeId,
    right: NodeId,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
    True,
    False,
}
