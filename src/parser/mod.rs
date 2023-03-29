mod ast;

use std::iter::Peekable;

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use ast::Node;
use indextree::{Arena, NodeId};

use self::ast::Literal;

struct Program {
    root_id: NodeId,
    nodes: Arena<Node>,
}

impl Program {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Arena::with_capacity(32);
        let mut tokens = lexer.tokens.into_iter().peekable();
        let mut root_id = None;

        while let Some(current) = tokens.next() {
            // println!("{:?}", current);
            if let Some(node) = Self::new_helper(&current, &mut nodes, &mut tokens) {
                let result = nodes.new_node(node);
                if root_id.is_none() {
                    root_id = Some(result);
                }
            }
        }

        Program {
            nodes,
            root_id: root_id.unwrap(),
        }
    }

    fn new_helper(
        current: &Token,
        arena: &mut Arena<Node>,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
        //return an result
    ) -> Option<Node> {
        match current {
            Token::Let => {
                // println!("{:?}", current);
                //currently does not autoformat lmao: https://github.com/rust-lang/rustfmt/issues/4914
                let Some(Token::Identifier(name)) = tokens.peek().cloned() else { return None; };
                tokens.next();

                if tokens.peek() != Some(&Token::Assign) {
                    return None;
                };
                tokens.next();

                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}

                //TODO: remove clone
                let let_statment = Node::Let {
                    indentifier: name,
                    value: arena.new_node(Node::Literal(Literal::Int(1))),
                };
                // println!("{:?}", let_statment);
                Some(let_statment)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::ast::ConcreteNode;

    use super::*;

    #[test]
    fn parse_let_statment() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

        let lexer = Lexer::new(input);
        let program = Program::new(lexer);

        let result = ConcreteNode::from(
            program.nodes.get(program.root_id).unwrap().get(),
            &program.nodes,
        );

        // let result = program
        //     .nodes
        //     .iter()
        //     .map(|x|)
        //     .collect::<Vec<_>>();

        println!("{:?}", result);
        // assert_eq!(program.nodes, ()); // Arena::from(vec![Node::Literal(Literal::Int(1))]));
    }
}
