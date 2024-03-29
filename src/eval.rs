use std::cell::RefCell;
use std::iter;
use std::rc::Rc;

use crate::ast::{
    BinaryExpression, BinaryOperator, BlockStatement, CallExpression, Expression,
    FunctionExpression, IfExpression, IndexExpression, Literal, Statement, UnaryExpression,
};
use crate::object::{Array, BuiltInFn, Environment, Function, HashTable, Object, SharedEnv, NIL};
use crate::parser::Parser;
use crate::token::Identifier;
use anyhow::{anyhow, bail, Result};

#[derive(Default)]
pub struct Program {
    pub env: SharedEnv,
}

impl Program {
    pub fn new() -> Self {
        Self {
            env: Rc::new(RefCell::new(Box::new(Environment::new()))),
        }
    }

    pub fn eval(&mut self, parser: Parser) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in parser.nodes {
            result = statement.eval(self.env.clone())?;
            if let Object::Return(inner) = result {
                return Ok(*inner);
            }
        }
        Ok(result)
    }
}

impl Statement {
    fn eval(self: Statement, env: SharedEnv) -> Result<Object> {
        match self {
            Statement::Let { identifier, value } => {
                let val = value.eval(env.clone())?;
                env.borrow_mut().try_insert(&identifier, val)?;
                Ok(NIL)
            }
            Statement::Return(exp) => Ok(Object::Return(Box::new(exp.eval(env)?))),
            Statement::Expression(exp) => Ok(exp.eval(env)?),
        }
    }
}

impl BlockStatement {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        let mut result = Object::Nil;
        for statement in self.0 {
            result = statement.eval(env.clone())?;
            if let Object::Return(_) = result {
                break;
            }
        }
        Ok(result)
    }
}

impl Expression {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        Ok(match self {
            Expression::Literal(literal) => match literal {
                Literal::Int(integer) => Object::Int(integer),
                Literal::True => Object::Bool(true),
                Literal::False => Object::Bool(false),
                Literal::String(string) => Object::String(string),
                Literal::Nil => Object::Nil,
                Literal::Hash(hash) => Object::HashTable(Box::new(HashTable::new(
                    hash.into_iter()
                        .map(|(key, value)| Ok((key.eval(env.clone())?, value.eval(env.clone())?)))
                        .collect::<Result<_>>()?,
                ))),
                Literal::Array(array) => Object::Array(Array::new(
                    array
                        .into_iter()
                        .map(|x| x.eval(env.clone()))
                        .collect::<Result<_>>()?,
                )),
            },
            Expression::Identifier(ident) => ident.eval(env)?,
            Expression::UnaryExpression(exp) => exp.eval(env)?,
            Expression::BinaryExp(bin_exp) => bin_exp.eval(env)?,
            Expression::If(if_exp) => if_exp.eval(env)?,
            Expression::Function(fn_exp) => fn_exp.eval(env)?,
            Expression::Call(call_exp) => call_exp.eval(env)?,
            Expression::IndexExpression(index_exp) => index_exp.eval(env)?,
        })
    }
}

impl IndexExpression {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        let container = self.container.eval(env.clone())?;
        let index = self.index.eval(env)?;
        Ok(match (container, index) {
            (Object::Array(array), Object::Int(index)) => {
                let value = if index.is_negative() {
                    array.len().checked_sub(index.unsigned_abs() as usize)
                } else {
                    Some(index as usize)
                };
                value.map_or(NIL, |x| array.get(x).cloned().unwrap_or(NIL))
            }
            (Object::HashTable(table), anything) => table.0.get(&anything).cloned().unwrap_or(NIL),
            (container, index) => bail!("index operator not supported: {}[{}]", container, index),
        })
    }
}

impl UnaryExpression {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        let operand = self.value.eval(env)?;
        Ok(match self.operator {
            crate::ast::UnaryOperator::Not => operand.not()?,
            crate::ast::UnaryOperator::Minus => operand.minus()?,
        })
    }
}

impl BinaryExpression {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        let (lhs, rhs) = (self.lhs.eval(env.clone())?, self.rhs.eval(env)?);
        Ok(match self.operator {
            BinaryOperator::Eq => lhs.eq(rhs).into(),
            BinaryOperator::NotEq => lhs.not_eq(rhs).into(),
            BinaryOperator::Lt => lhs.lt(rhs).into(),
            BinaryOperator::Lte => lhs.lte(rhs).into(),
            BinaryOperator::Gt => lhs.gt(rhs).into(),
            BinaryOperator::Gte => lhs.gte(rhs).into(),
            BinaryOperator::Add => lhs.add(rhs)?,
            BinaryOperator::Sub => lhs.sub(rhs)?,
            BinaryOperator::Mul => lhs.mul(rhs)?,
            BinaryOperator::Div => lhs.div(rhs)?,
        })
    }
}

impl Identifier {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        env.borrow()
            .get(&self)
            .ok_or(anyhow!("Identifier {} not found", self.inner()))
    }
}

impl CallExpression {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        match self.function.eval(env.clone())? {
            Object::Function(function) => function.eval(env, self.arguments),
            Object::BuiltInFn(builtin) => builtin.eval(env, self.arguments),
            value => Err(anyhow!("expected a function, found: {value}")),
        }
    }
}

impl BuiltInFn {
    pub fn eval(self, env: SharedEnv, arguments: Vec<Expression>) -> Result<Object> {
        match self {
            //maybe use https://docs.rs/itertools/0.11.0/itertools/trait.Itertools.html#method.tuples
            BuiltInFn::Len => match TryInto::<[Expression; 1]>::try_into(arguments) {
                Ok([val]) => Ok(match val.eval(env)? {
                    Object::String(val) => Object::Int(val.len() as i64),
                    Object::Array(val) => Object::Int(val.len() as i64),
                    val => bail!("expected array or string, found: {}", val),
                }),
                Err(vec) => bail!("expected 1 argument, found: {}", vec.len()),
            },
            BuiltInFn::First => match TryInto::<[Expression; 1]>::try_into(arguments) {
                Ok([val]) => Ok(match val.eval(env)? {
                    Object::Array(val) => val.first().cloned().unwrap_or(NIL),
                    val => bail!("expected array, found: {}", val),
                }),
                Err(vec) => bail!("expected 1 argument, found: {}", vec.len()),
            },
            BuiltInFn::Last => match TryInto::<[Expression; 1]>::try_into(arguments) {
                Ok([val]) => Ok(match val.eval(env)? {
                    Object::Array(val) => val.last().cloned().unwrap_or(NIL),
                    val => bail!("expected array, found: {}", val),
                }),
                Err(vec) => bail!("expected 1 argument, found: {}", vec.len()),
            },
            BuiltInFn::Rest => match TryInto::<[Expression; 1]>::try_into(arguments) {
                Ok([val]) => Ok(match val.eval(env)? {
                    Object::Array(val) => Array::new(val.0.into_iter().skip(1).collect()).into(),
                    val => bail!("expected array, found: {}", val),
                }),
                Err(vec) => bail!("expected 1 argument, found: {}", vec.len()),
            },
            BuiltInFn::Push => match TryInto::<[Expression; 2]>::try_into(arguments) {
                Ok([container, element]) => {
                    Ok(match (container.eval(env.clone())?, element.eval(env)?) {
                        (Object::Array(array), element) => {
                            Array::new(array.0.into_iter().chain(iter::once(element)).collect())
                                .into()
                        }
                        val => bail!("expected array and element, found: {} and {}", val.0, val.1),
                    })
                }
                Err(vec) => bail!("expected 2 argument, found: {}", vec.len()),
            },
            BuiltInFn::Puts => {
                for exp in arguments {
                    println!("{}", exp.eval(env.clone())?)
                }
                Ok(NIL)
            }
        }
    }
}

impl Function {
    fn eval(self, env: SharedEnv, arguments: Vec<Expression>) -> Result<Object> {
        let args = arguments
            .into_iter()
            .map(|x| x.eval(env.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        let extended_env = Environment::new_enclosed(
            self.env.clone(),
            self.parameters
                .into_iter()
                .map(|x| x.inner())
                .zip(args)
                .collect(),
        );
        self.body.eval(extended_env)
    }
}

impl FunctionExpression {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        Ok(Object::Function(Box::new(Function::new(
            self.parameters,
            self.body,
            env,
        ))))
    }
}

impl IfExpression {
    fn eval(self, env: SharedEnv) -> Result<Object> {
        let condition = self.condition.eval(env.clone())?.into_bool()?;

        Ok(if condition {
            self.consequence.eval(env)?
        } else if let Some(alternative) = self.alternative {
            alternative.eval(env)?
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
        let mut program = Program::new();

        program.eval(parser).unwrap()
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
    fn eval_recursive_fn() {
        let input = "
            let fib = fn(x) {
                if (x <= 2){
                    1
                } else {
                    fib(x - 1) + fib(x - 2)
                }
            };
            fib(10);
        ";
        let result = parse_test_input([input].as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_array_map() {
        let input = "
        let map = fn(arr, f) {
            let iter = fn(arr, accumulated) {
                if (len(arr) == 0) {
                    accumulated
                } else {
                    iter(rest(arr), push(accumulated, f(first(arr))));
                }
            };
            iter(arr, []);
        };
        let a = [1, 2, 3, 4];
        let double = fn(x) { x * 2 };
        map(a, double);
        ";

        let result = parse_test_input([input].as_slice());
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
            "!!0",
            "!!fn(x){}",
            "true == true;",
            "false != false;",
            r#"!!"""#,
            r#"!!"something""#,
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
    fn eval_array_literal() {
        let input = [r#"[1, 2 * 2, "foo", fn(x) { return x + 2; }]"#];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_array_index() {
        let input = [
            "[1, 2, 3][0]",
            "[1, 2, 3][1]",
            "[1, 2, 3][2]",
            "let i = 0; [1][i];",
            "[1, 2, 3][1 + 1];",
            "let myArray = [1, 2, 3]; myArray[2];",
            "let myArray = [1, 2, 3]; myArray[0] + myArray[1] + myArray[2];",
            "let myArray = [1, 2, 3]; let i = myArray[0]; myArray[i]",
            "[1, 2, 3][3]",
            "[1, 2, 3][-1]",
            "[1, 2, 3][-3]",
            "[1, 2, 3][-4]",
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_hash_index() {
        let input = [
            r#"{"foo": 5}["foo"]"#,
            r#"{"foo": 4}["bar"]"#,
            r#"let key = "foo"; {"foo": 3}[key]"#,
            r#"{}["foo"]"#,
            r#"{5: 2}[5]"#,
            r#"{true: 5}[true]"#,
            r#"{false: 5}[false]"#,
            r#"{"true": 5}[true]"#,
            r#"{true: 5}["true"]"#,
            r#"{3: 1}["3"]"#,
            r#"{"3": 1}[3]"#,
        ];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_hash_literal() {
        let input = r#"
            let two = "potato";
            {
                "one": 1 + 1,
                two: 13,
                "foo" + "bar": 10,
                4: {1:1},
                true: [1,2],
                "false": 6
            };"#;

        let result = parse_program(input);
        if let Object::HashTable(hash) = result {
            let mut result = hash
                .0
                .into_iter()
                .map(|(key, value)| (key.to_string(), value))
                .collect::<Vec<_>>();
            result.sort_by(|(k1, _), (k2, _)| k1.cmp(k2));
            assert_yaml_snapshot!(result);
        } else {
            panic!("expected hash table");
        }
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
    fn eval_string_concat() {
        let input = [r#""hello" + "world""#];

        let result = parse_test_input(input.as_slice());
        assert_yaml_snapshot!(result);
    }

    #[test]
    fn eval_builtin_len() {
        let input = [
            r#"len("")"#,
            r#"len("four")"#,
            r#"len("hello world")"#,
            r#"len(1)"#,
            r#"len("one", "two")"#,
        ];

        let result: Vec<_> = input
            .iter()
            .map(|x| {
                let lexer = Lexer::new(x);
                let parser = Parser::new(lexer);
                let mut program = Program::new();
                match program.eval(parser) {
                    Ok(ok) => ok.to_string(),
                    Err(err) => err.to_string(),
                }
            })
            .collect();
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
            r#""Hello" - "World""#,
        ];

        let result: Vec<String> = input
            .iter()
            .flat_map(|x| {
                let lexer = Lexer::new(x);
                let parser = Parser::new(lexer);
                let env = Environment::new();
                let mut program = Program::new();
                match program.eval(parser) {
                    Ok(val) => None,
                    Err(err) => Some(err.to_string()),
                }
            })
            .collect();

        assert_yaml_snapshot!(result);
    }
}
