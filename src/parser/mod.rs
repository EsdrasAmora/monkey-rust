mod ast;

use std::default;
use std::iter::Peekable;

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use ast::Node;
use id_arena::Arena;

use self::ast::Literal;

struct Program {
    nodes: Vec<Node>,
}

impl Program {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Vec::with_capacity(32);
        let mut tokens = lexer.tokens.into_iter();

        Program { nodes }
    }

    fn new_helper(
        current: &Token,
        arena: &mut Vec<Node>,
        tokens: &mut Peekable<impl Iterator<Item = Token>>,
        //return an result
    ) -> Option<Node> {
        match current {
            Token::Let => {
                //currently does not autoformat lmao: https://github.com/rust-lang/rustfmt/issues/4914
                let Some(Token::Identifier(name)) = tokens.peek().cloned() else { return None; };
                tokens.next();

                if tokens.peek() == Some(&Token::Assign) {
                    return None;
                };
                tokens.next();

                while tokens.next().filter(|x| x != &Token::Semicolon).is_some() {}

                //TODO: remove clone
                let let_statment = Node::Let {
                    name,
                    value: {
                        arena.push(Node::Literal(Literal::Int(1)));
                        arena.len() - 1
                    },
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
    fn parse_let_statment() {
        let input = "
        let x = 5;
        let y = 10;
        let foobar = 838383;";

        let lexer = Lexer::new(input);
        let program = Program::new(lexer);

        assert_eq!(
            program.nodes,
            // Arena::from(vec![Node::Literal(Literal::Int(1))])
        );
    }
}
