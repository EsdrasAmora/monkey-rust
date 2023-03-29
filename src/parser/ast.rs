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
    Add(Box<Expression>),
    Sub(Box<Expression>),
    Mul(Box<Expression>),
    Div(Box<Expression>),
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
    True,
    False,
    Nill,
}

// use indextree::{Arena, NodeId};

// #[derive(Debug, Eq, PartialEq)]
// pub enum Node {
//     Let { indentifier: String, value: NodeId },
//     Return { value: NodeId },
//     Literal(Literal),
//     // Add(BinaryExpression),
//     // Sub(BinaryExpression),
//     // Mul(BinaryExpression),
//     // Div(BinaryExpression),
// }

// impl ConcreteNode {
//     pub fn from(value: &Node, arena: &Arena<Node>) -> Self {
//         match value {
//             Node::Let { indentifier, value } => ConcreteNode::Let {
//                 indentifier: indentifier.clone(),
//                 value: Box::from(ConcreteNode::from(arena.get(*value).unwrap().get(), arena)),
//             },
//             Node::Return { value } => ConcreteNode::Return(Box::from(ConcreteNode::from(
//                 arena.get(*value).unwrap().get(),
//                 arena,
//             ))),
//             Node::Literal(literal) => ConcreteNode::Literal(literal.clone()),
//         }
//     }
// }

// impl Node {
//     pub fn pretty_print(&self, arena: Arena<Self>) {
//         let values = arena.iter().map(|x| x.get()).collect::<Vec<_>>();
//         println!("{:?}", values);
//     }

//     fn matches(a: &Node, b: &Node) -> bool {
//         std::mem::discriminant(a) == std::mem::discriminant(b)
//     }
// }

// #[derive(Debug, Eq, PartialEq)]
// pub struct BinaryExpression {
//     left: NodeId,
//     right: NodeId,
// }
