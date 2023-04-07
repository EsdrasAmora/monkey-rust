use self::ast::Statement;
use self::token_parser::TokenParser;
use crate::ast;
use crate::lexer::Lexer;
use crate::token_parser;
use anyhow::Error;

#[derive(Debug)]
pub struct Parser {
    pub nodes: Vec<Statement>,
    pub errors: Vec<Error>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
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

//TODO: missing Assign support, only able to declare variables with let but not assign them afterwards;
//TODO: create error types and think about when to buble them up
#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_yaml_snapshot;

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
        assert_yaml_snapshot!(program.nodes);
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
        assert_yaml_snapshot!(program.nodes);
    }

    #[test]
    fn parse_grouped_expression() {
        let input = "(5 + 5) * 5; -(5 + 5);";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);

        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_yaml_snapshot!(program.nodes);
    }

    #[test]
    fn parse_if_expression() {
        let input = "if (x < y) { return x; };";

        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_yaml_snapshot!(program.nodes);
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
        assert_yaml_snapshot!(program.nodes);
    }

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
        assert_yaml_snapshot!(program.nodes);
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
        assert_yaml_snapshot!(program.nodes);
    }

    #[test]
    fn parse_expression_statement_identifier() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_yaml_snapshot!(program.nodes);
    }

    #[test]
    fn parse_expression_statement_integer_literal() {
        let input = "3;";
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_yaml_snapshot!(program.nodes);
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
        assert_yaml_snapshot!(program.nodes);
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
        assert_yaml_snapshot!(program.nodes);
    }

    #[test]
    fn check_parser_precedence() {
        let input = r#"
        -a * b;
        !-a;
        a + b + c;
        a + b - c;
        a * b * c;
        a * b / c;
        a + b / c;
        a + b * c + d / e - f;
        3 + 4; -5 * 5;
        5 > 4 == 3 < 4;
        5 < 4 != 3 > 4;
        3 + 4 * 5 == 3 * 1 + 4 * 5;
        true;
        false;
        3 > 5 == false;
        3 < 5 == true;
        1 + (2 + 3) + 4;
        (5 + 5) * 2;
        2 / (5 + 5);
        (5 + 5) * 2 * (5 + 5);
        -(5 + 5);
        !(true == true);
        a + add(b * c) + d;
        add(a, b, 1, 2 * 3, 4 + 5, add(6, 7 * 8));
        add(a + b + c * d / f + g);"#;
        let lexer = Lexer::new(input);
        let program = Parser::new(lexer);
        assert!(program.errors.is_empty(), "errors: {:#?}", program.errors);
        assert_yaml_snapshot!(program.nodes);
    }
}
