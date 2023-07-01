use crate::ast::{
    BlockStatement, CallExpression, Expression, FunctionExpression, IfExpression, Literal,
    Statement,
};
use crate::object::{Environment, Function, Object, NIL};
use crate::parser::Parser;
use crate::token::Identifier;
use anyhow::{anyhow, Result};

impl Environment {
    pub fn eval_program(&mut self, parser: Parser) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in parser.nodes {
            result = statement.eval(self)?;
            if let Object::Return(inner) = result {
                return Ok(*inner);
            }
        }
        Ok(result)
    }
}

impl Statement {
    fn eval(self: Statement, environment: &mut Environment) -> Result<Object> {
        match self {
            Statement::Let { identifier, value } => {
                let val = value.eval(environment)?;
                environment.try_insert(&identifier, val)?;
                Ok(NIL)
            }
            Statement::Return(exp) => Ok(Object::Return(Box::new(exp.eval(environment)?))),
            Statement::Expression(exp) => Ok(exp.eval(environment)?),
        }
    }
}

impl BlockStatement {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in self.0 {
            result = statement.eval(environment)?;
            if let Object::Return(_) = result {
                break;
            }
        }
        Ok(result)
    }
}

impl Expression {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        Ok(match self {
            Expression::Literal(literal) => match literal {
                Literal::Int(integer) => Object::Int(integer),
                Literal::True => Object::Bool(true),
                Literal::False => Object::Bool(false),
                Literal::String(string) => Object::String(string),
                Literal::Nil => Object::Nil,
            },
            Expression::Identifier(ident) => ident.eval(environment)?,
            Expression::Oposite(exp) => exp.0.eval(environment)?.oposite()?,
            Expression::Not(exp) => exp.0.eval(environment)?.not()?,
            Expression::Eq(exp) => exp
                .lhs
                .eval(environment)?
                .eq(exp.rhs.eval(environment)?)
                .into(),
            Expression::NotEq(exp) => exp
                .lhs
                .eval(environment)?
                .not_eq(exp.rhs.eval(environment)?)
                .into(),
            Expression::Lt(exp) => exp
                .lhs
                .eval(environment)?
                .lt(exp.rhs.eval(environment)?)
                .into(),
            Expression::Lte(exp) => exp
                .lhs
                .eval(environment)?
                .lte(exp.rhs.eval(environment)?)
                .into(),
            Expression::Gt(exp) => exp
                .lhs
                .eval(environment)?
                .gt(exp.rhs.eval(environment)?)
                .into(),
            Expression::Gte(exp) => exp
                .lhs
                .eval(environment)?
                .gte(exp.rhs.eval(environment)?)
                .into(),
            Expression::Add(exp) => exp.lhs.eval(environment)?.add(exp.rhs.eval(environment)?)?,
            Expression::Sub(exp) => exp.lhs.eval(environment)?.sub(exp.rhs.eval(environment)?)?,
            Expression::Mul(exp) => exp.lhs.eval(environment)?.mul(exp.rhs.eval(environment)?)?,
            Expression::Div(exp) => exp.lhs.eval(environment)?.div(exp.rhs.eval(environment)?)?,
            Expression::If(if_exp) => if_exp.eval(environment)?,
            Expression::Function(fn_exp) => fn_exp.eval(environment)?,
            Expression::Call(call_exp) => call_exp.eval(environment)?,
        })
    }
}

impl Identifier {
    //FIXME: remove clone
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        Ok(environment
            .get(&self)
            .ok_or(anyhow!("Identifier {} not found", self.inner()))?
            .to_owned())
    }
}

impl CallExpression {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        let object = match self.function {
            either::Either::Left(ident) => ident.eval(environment)?,
            either::Either::Right(func) => func.eval(environment)?,
        };

        match object {
            Object::Function(function) => {
                let args = self
                    .arguments
                    .into_iter()
                    .map(|x| x.eval(environment))
                    .collect::<Result<Vec<_>, _>>()?;

                let mut extended_env = Environment::new_enclosed(
                    //FIXME: remove clone
                    function.env.clone(),
                    function
                        .parameters
                        .into_iter()
                        .map(|x| x.inner())
                        .zip(args)
                        .collect(),
                );
                function.body.eval(&mut extended_env)
            }
            value => Err(anyhow!("expected a function, found: {value:?}")),
        }
    }
}

impl Function {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        Ok(Object::Function(self))
    }
}

impl FunctionExpression {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        Ok(Object::Function(Function::new(
            self.parameters,
            self.body,
            //FIXME: remove clone
            environment.clone(),
        )))
    }
}

impl IfExpression {
    fn eval(self, environment: &mut Environment) -> Result<Object> {
        let condition = self.condition.eval(environment)?.into_bool()?;

        Ok(if condition {
            self.consequence.eval(environment)?
        } else if let Some(alternative) = self.alternative {
            alternative.eval(environment)?
        } else {
            NIL
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{lexer::Lexer, parser::Parser};
    use insta::assert_yaml_snapshot;

    fn parse_program(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let parser = Parser::new(lexer);
        let mut environment = Environment::new();

        environment.eval_program(parser).unwrap()
    }

    fn parse_test_input(input: &[&str]) -> Vec<Object> {
        input.iter().map(|x| parse_program(x)).collect()
    }

    #[test]
    fn eval_numeric_expression() {
        let input = [
            "5;",
            "-10;",
            "3 * (3 + 3) + 10;",
            "(5 + 10 * 2 + 15 / 3) * 2 + -10;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_boolean_expression() {
        let input = [
            "true;",
            "false;",
            "1 >= 2;",
            "1 >= 1;",
            "1 <= 2;",
            "1 <= 1;",
            "1 > 2;",
            "1 > 1;",
            "1 < 2;",
            "1 < 1;",
            "true == true;",
            "false != false;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_not_expression() {
        let input = [
            "!5;", "!!5;", "!0;", "!!0;", "!true;", "!!true;", "!false;", "!!false;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_if_expression() {
        let input = [
            "if (true) { 10; };",
            "if (false) { 10; };",
            "if (1) { 10; };",
            "if (1 < 2) { 10; };",
            "if (1 > 2) { 10; };",
            "if (1 > 2) { 10; } else { 20; };",
            "if (1 < 2) { 10; } else { 20; };",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_return_statement() {
        let input = [
            "return 10;",
            "return 10; 9;",
            "return 2 * 5; 9;",
            "9; return 2 * 5; 9;",
            "if (true) { return 10; }; return 9;",
            "if (true) { if (true) { return 10; }; return 1;};",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_let_statements() {
        let input = [
            "let a = 5; a;",
            "let a = 5 * 5; a;",
            "let a = 5; let b = a; b;",
            "let a = 5; let b = a; let c = a + b + 5; c;",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_function_declaration() {
        let input = ["fn(x) { x + 2; };"];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_function_call() {
        let input = [
            "let identity = fn(x) { x; }; identity(5);",
            "let identity = fn(x) { return x; }; identity(5);",
            "let double = fn(x) { x * 2; }; double(5);",
            "let add = fn(x, y) { x + y; }; add(5, 5);",
            "let add = fn(x, y) { x + y; }; add(5 + 5, add(5, 5));",
            "fn(x) { x; }(5)",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_closure() {
        let input = [
            "
            let newAdder = fn(x) {fn(y) { x + y };};
            let addTwo = newAdder(2);
            addTwo(2);
            ",
            "
            let newAdder = fn(x) {fn(y) { x + y };};
            let addTwo = newAdder(2);
            let addEight = newAdder(8);
            let addTen = fn(x) { addTwo(addEight(x)) };
            addTen(5);
            ",
            "
            let add = fn(a, b) { a + b };
            let applyFunc = fn(a, b, func) { func(a, b) };
            applyFunc(10, 2, add);
            ",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_type_errors() {
        let input = [
            "5 + true;",
            "5 + true; 5;",
            "-true;",
            "true + false;",
            "true + false + true + false;",
            "5; true + false; 5",
            "if (10 > 1) {
                if (10 > 1) {
                    return true + false;
                };
                return 1;
            };",
            "foobar;",
            "let foo = 3; let foo = 4;",
        ];

        let result: Vec<String> = input
            .iter()
            .flat_map(|x| {
                let lexer = Lexer::new(x);
                let parser = Parser::new(lexer);
                let mut environment = Environment::new();
                let val = environment.eval_program(parser);
                match val {
                    Ok(val) => None,
                    Err(err) => Some(err.to_string()),
                }
            })
            .collect();

        assert_yaml_snapshot!(result);
    }
}