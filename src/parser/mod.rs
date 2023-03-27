mod ast;

use std::default;
use std::iter::Peekable;

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use ast::Node;
use id_arena::Arena;

struct Program {
    nodes: Arena<Node>,
}

impl Program {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Arena::with_capacity(32);
        let mut tokens = lexer.tokens.into_iter();

        // start.cloned()

        // while let Some(current) = start.next() {
        //     match current {
        //         Token::Let => {
        //             let b = tokens.next_if(|x| x);

        //             // let name = tokens.peek();
        //             // let value = tokens.next();

        //             Some(Node::Let {
        //                 name: "".to_string(),
        //                 value: 13,
        //             })
        //         }
        //         _ => None,
        //     }
        // }

        Program { nodes }
    }

    fn new_helper(
        current: &Token,
        arena: &mut Arena<Node>,
        tokens: &mut Peekable<std::slice::Iter<Token>>,
    ) -> Option<Node> {
        None
        // match current {
        //     Token::Let => {
        //         let b = tokens.next_if(|x| x);

        //         // let name = tokens.peek();
        //         // let value = tokens.next();

        //         Some(Node::Let {
        //             name: "".to_string(),
        //             value: 13,
        //         })
        //     }
        //     _ => None,
        // }
    }
}
