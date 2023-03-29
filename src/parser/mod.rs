mod ast;

use std::iter::Peekable;

use anyhow::{bail, Error, Result};

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;

use self::ast::{Literal, Node};

struct Parser {
    nodes: Vec<Node>,
    errors: Vec<Error>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Vec::with_capacity(32);
        let mut errors = Vec::with_capacity(8);
        let mut tokens = lexer.tokens.into_iter().peekable();

        while let Some(current) = tokens.next() {
            Self::new_helper(&current, &mut tokens)
                .map_or_else(|err| errors.push(err), |val| nodes.push(val))
        }

        Parser { nodes, errors }
    }

    fn new_helper(
        current: &Token,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
    ) -> Result<Node> {
        match current {
            Token::Let => {
                //currently does not autoformat lmao: https://github.com/rust-lang/rustfmt/issues/4914
                let Some(Token::Identifier(_)) = tokens.peek() else { bail!("expected token to be {:?}, got {:?} instead",Token::Identifier("foo".to_owned()), tokens.peek()) };
                let Some(Token::Identifier(name)) = tokens.next() else{ unreachable!() };

                if tokens.peek() != Some(&Token::Assign) {
                    bail!(
                        "expected assign after indentifier found: {:?}",
                        tokens.peek()
                    )
                };
                tokens.next();

                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}

                let let_statment = Node::Let {
                    indentifier: name,
                    value: Box::new(Node::Literal(Literal::Int(5))),
                };

                Ok(let_statment)
            }
            _ => bail!("Cannot parse an statment starting with {:?}", current),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn parse_let_statement() {
        let input = "
        let x = false;
        let y = 10;
        let foobar = true;";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);

        assert_eq!(
            program.nodes,
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
