use indextree::{Arena, NodeId};

#[derive(Debug, Eq, PartialEq)]
pub enum Node {
    Let { indentifier: String, value: NodeId },
    Return { value: NodeId },
    Literal(Literal),
    // Add(BinaryExpression),
    // Sub(BinaryExpression),
    // Mul(BinaryExpression),
    // Div(BinaryExpression),
}

enum ConcreteNode<'a> {
    Let {
        indentifier: String,
        value: &'a ConcreteNode<'a>,
    },
    Return(&'a ConcreteNode<'a>),
    Literal(Literal),
    // Indentifier(String),
    // Add(BinaryExpression),
    // Sub(BinaryExpression),
    // Mul(BinaryExpression),
    // Div(BinaryExpression),
}

impl Node {
    pub fn pretty_print(&self, arena: Arena<Self>) {
        let values = arena.iter().map(|x| x.get()).collect::<Vec<_>>();
        println!("{:?}", values);
    }

    fn matches(a: &Node, b: &Node) -> bool {
        std::mem::discriminant(a) == std::mem::discriminant(b)
    }
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
