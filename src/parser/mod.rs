mod ast;

use std::iter::Peekable;

//TODO: how to rexport this?
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use ast::Node;
use id_arena::Arena;

struct Program {
    statements: Arena<Node>,
}

impl Program {
    fn new(lexer: Lexer) -> Self {
        let statements = Arena::with_capacity(32);
        let mut tokens = lexer.tokens.iter().peekable();

        while let Some(token) = tokens.next() {
            // let statement = parse_statement(tokens);
            // statements.alloc(statement);
        }

        Program { statements }
    }

    fn new_helper(tokens: &mut Peekable<impl Iterator<Item = Token>>) {}
}
