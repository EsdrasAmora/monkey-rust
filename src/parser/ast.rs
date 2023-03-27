use id_arena::{Arena, Id};

type AstNodeId = Id<AstNode>;

#[derive(Debug, Eq, PartialEq)]
pub enum AstNode {
    Const(i64),
    Var(String),
    Add { lhs: AstNodeId, rhs: AstNodeId },
    Sub { lhs: AstNodeId, rhs: AstNodeId },
    Mul { lhs: AstNodeId, rhs: AstNodeId },
    Div { lhs: AstNodeId, rhs: AstNodeId },
}
