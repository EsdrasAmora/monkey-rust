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
    nodes: Arena<Node>,
}

impl Program {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Arena::with_capacity(32);
        let mut tokens = lexer.tokens.into_iter();

        Program { nodes }
    }

    fn new_helper(
        current: &Token,
        arena: &mut Arena<Node>,
        tokens: &mut Peekable<std::slice::Iter<Token>>,
        //return an result
    ) -> Option<Node> {
        match current {
            Token::Let => {
                // let x = "value".to_string();

                // if let Some(Token::Identifier(name)) = tokens.peek() {
                //     let x = name;
                // }

                // let batata = tokens.peek()?;
                // if let Token::Identifier(name) = batata {}

                let (Some(bb), Some(aaaa)) = (tokens.next(), tokens.next()) else {
                            // panic!("Can't segment count item pair: '{s}'");
                    return None;
                };

                let let_statment = Node::Let {
                    name: "".to_string(),
                    value: arena.alloc(Node::Literal(Literal::Int(1))),
                };
                Some(let_statment)
            }
            _ => None,
        }
    }
}

fn variant_eq<T>(a: &T, b: &T) -> bool {
    std::mem::discriminant(a) == std::mem::discriminant(b)
}
