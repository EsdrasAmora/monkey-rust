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
//TODO: split Node into statements and expressions or add a vec of nodeIds to Program and use them as rootNode pointers;
#[derive(Debug)]
pub enum ConcreteNode {
    Let {
        indentifier: String,
        value: Box<ConcreteNode>,
    },
    Return(Box<ConcreteNode>),
    Literal(Literal),
    // Indentifier(String),
    // Add(BinaryExpression),
    // Sub(BinaryExpression),
    // Mul(BinaryExpression),
    // Div(BinaryExpression),
}

impl ConcreteNode {
    pub fn from(value: &Node, arena: &Arena<Node>) -> Self {
        match value {
            Node::Let { indentifier, value } => ConcreteNode::Let {
                indentifier: indentifier.clone(),
                value: Box::from(ConcreteNode::from(arena.get(*value).unwrap().get(), arena)),
            },
            Node::Return { value } => ConcreteNode::Return(Box::from(ConcreteNode::from(
                arena.get(*value).unwrap().get(),
                arena,
            ))),
            Node::Literal(literal) => ConcreteNode::Literal(literal.clone()),
        }
    }
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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Literal {
    Int(i64),
    String(String),
    True,
    False,
}
