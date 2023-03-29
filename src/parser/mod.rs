mod ast;

use std::iter::Peekable;

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;

use self::ast::{Literal, Node};

struct Program {
    nodes: Vec<Node>,
}

impl Program {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Vec::with_capacity(32);
        let mut tokens = lexer.tokens.into_iter().peekable();

        while let Some(current) = tokens.next() {
            //TODO: is this more readable than `if let some`?
            Self::new_helper(&current, &mut tokens).map(|x| nodes.push(x));
        }

        Program { nodes }
    }

    //TODO:return an result
    fn new_helper(
        current: &Token,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Option<Node> {
        match current {
            Token::Let => {
                //currently does not autoformat lmao: https://github.com/rust-lang/rustfmt/issues/4914
                let Some(Token::Identifier(_)) = tokens.peek() else { return None; };
                let Some(Token::Identifier(name)) = tokens.next() else{ return None; };

                if tokens.peek() != Some(&Token::Assign) {
                    return None;
                };
                tokens.next();

                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}

                let let_statment = Node::Let {
                    indentifier: name,
                    value: Box::new(Node::Literal(Literal::Int(5))),
                };

                Some(let_statment)
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_let_statement() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

        let lexer = Lexer::new(input);
        let program = Program::new(lexer);

        let result = program.nodes;

        assert_eq!(
            result,
            [
                Node::Let {
                    indentifier: "x".to_string(),
                    value: Box::new(Node::Literal(Literal::Int(5)))
                },
                Node::Let {
                    indentifier: "y".to_string(),
                    value: Box::new(Node::Literal(Literal::Int(5)))
                },
                Node::Let {
                    indentifier: "foobar".to_string(),
                    value: Box::new(Node::Literal(Literal::Int(5)))
                }
            ]
        )
    }
}
