pub(crate) mod ast;
pub(crate) mod token_parser;

use self::{ast::Statement, token_parser::TokenParser};
use crate::lexer::Lexer;
use anyhow::Error;

#[derive(Debug)]
struct Parser {
    nodes: Vec<Statement>,
    errors: Vec<Error>,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut nodes = Vec::new();
        let mut errors = Vec::new();
        let mut tokens = TokenParser::new(lexer.tokens);

        while let Some(current) = tokens.next() {
            tokens
                .parse_statement(current)
                .map_or_else(|err| errors.push(err), |val| nodes.push(val))
        }

        Parser { nodes, errors }
    }
}

//TODO: not sure if i should keep the expected json's here or in the mocks folder. Maybe I should just use insta (snapshot)?
//TODO: missing Assign support, only able to declare variables with let but not assign them afterwards;
//TODO: create error types and think about when to buble them up
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::ast::{Expression, Literal, UnaryExpression};
    use pretty_assertions::assert_eq;
    use serde_json::json;
    use smol_str::SmolStr;

    #[test]
    fn parse_call_expression() {
        let input = "
        myFn();
        myFn(1,2);
        myFn(1, 2 * 3, -4 + 5);
        myFn(fn() {});
        fn() {}();
        fn(x) { return x; }(1);
        -myFn(1,2);
        myFn(fn(a,b) {return a + b;}(1,2));";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        println!("{:#?}", program.nodes);
    }

    #[test]
    fn parse_fn_expression() {
        let input = "
        fn() {};
        fn(x) { x; };
        fn(x, y, z) { return x + y + z; };";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        println!("{:#?}", program.nodes);
    }

    #[test]
    fn parse_grouped_expression() {
        let input = "(5 + 5) * 5; -(5 + 5);";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        println!("{:#?}", program.nodes);
    }

    #[test]
    fn parse_if_expression() {
        let input = "if (x < y) { return x; };";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        println!("{:#?}", program.nodes);
    }

    #[test]
    fn parse_if_else_expression() {
        let input = "
        if (x < y) { x; } else { y; };
        if (x < y) { return x; } else { return y; };
        if (x < y) { x == 3; } else { x != 1; };";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        println!("{:#?}", program.nodes);
    }

    #[test]
    fn parse_function_call_expression() {}

    #[test]
    fn parse_infix_expression() {
        let input = "
        5 + 5;
        5 - 5;
        5 * 5;
        5 / 5;
        5 > 5;
        5 < 5;
        5 == 5;
        5 != 5;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);

        let right = json!([
            {"Expression": {"Add":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}},
            {"Expression":{"Sub":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}},
            {"Expression":{"Mul":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}},
            {"Expression":{"Div":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}},
            {"Expression":{"Gt":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}},
            {"Expression":{"Lt":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}},
            {"Expression":{"Eq":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}},
            {"Expression":{"NotEq":{"lhs":{"Literal":{"Int":5}},"rhs":{"Literal":{"Int":5}}}}}
        ]);
        assert_eq!(serde_json::to_value(&program.nodes).unwrap(), right)
    }

    #[test]
    fn parse_prefix_expression() {
        let input = "
        -1;
        !5;
        !!-2;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [
                Statement::Expression(Box::new(Expression::Oposite(UnaryExpression(Box::new(
                    Literal::Int(1).into()
                ))))),
                Statement::Expression(Box::new(Expression::Not(UnaryExpression(Box::new(
                    Literal::Int(5).into()
                ))))),
                Statement::Expression(Box::new(Expression::Not(UnaryExpression(Box::new(
                    Expression::Not(UnaryExpression(Box::new(Expression::Oposite(
                        UnaryExpression(Box::new(Literal::Int(2).into()))
                    ))))
                )))))
            ]
        )
    }

    #[test]
    fn parse_expression_statement_identifier() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [Statement::Expression(Box::new(Expression::Identifier(
                SmolStr::new("foobar")
            )))]
        )
    }

    #[test]
    fn parse_expression_statement_integer_literal() {
        let input = "3;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [Statement::Expression(Box::new(Literal::Int(3).into()))]
        )
    }

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
                Statement::Let {
                    identifier: SmolStr::new("x").into(),
                    value: Box::new(Literal::False.into())
                },
                Statement::Let {
                    identifier: SmolStr::new("y").into(),
                    value: Box::new(Literal::Int(10).into())
                },
                Statement::Let {
                    identifier: SmolStr::new("foobar").into(),
                    value: Box::new(Literal::True.into())
                }
            ]
        )
    }

    #[test]
    fn parse_return_statement() {
        let input = "
        return 5;
        return 10;
        return 993322;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_eq!(
            program.nodes,
            [
                Statement::Return(Box::new(Literal::Int(5).into())),
                Statement::Return(Box::new(Literal::Int(10).into())),
                Statement::Return(Box::new(Literal::Int(993322).into()))
            ]
        )
    }

    #[test]
    fn check_parser_precedence_2() {
        let input = r#"
        -a * b
        !-a
        a + b + c
        a + b - c
        a * b * c
        a * b / c
        a + b / c
        a + b * c + d / e - f
        3 + 4; -5 * 5
        5 > 4 == 3 < 4
        5 < 4 != 3 > 4
        3 + 4 * 5 == 3 * 1 + 4 * 5
        true
        false
        3 > 5 == false
        3 < 5 == true
        1 + (2 + 3) + 4
        (5 + 5) * 2
        2 / (5 + 5)
        (5 + 5) * 2 * (5 + 5)
        -(5 + 5)
        !(true == true)
        a + add(b * c) + d
        add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8))
        add(a + b + c * d / f + g);"#;
        // let lexer = Lexer::new(input);
        // let program = Parser::new(lexer);
        // println!("{:#?}", program.nodes);
    }
}
