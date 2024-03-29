use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    fmt::{self, Display},
    ops::{Deref, DerefMut},
    ptr,
    rc::Rc,
};

use serde::{self, Serialize};
use smol_str::SmolStr;
use thiserror::Error;

use crate::{
    ast::{BinaryOperator, BlockStatement, UnaryOperator},
    token::Identifier,
};

#[derive(Debug, Serialize, Clone)]
pub enum Object {
    Nil,
    Int(i64),
    Bool(bool),
    BuiltInFn(BuiltInFn),
    Array(Array),
    HashTable(Box<HashTable>),
    String(SmolStr),
    Function(Box<Function>),
    Return(Box<Object>),
}

impl std::hash::Hash for Object {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Object::Nil => state.write_u8(69),
            Object::Int(int) => int.hash(state),
            Object::Bool(bool) => bool.hash(state),
            Object::BuiltInFn(builtin) => builtin.hash(state),
            Object::Array(array) => array.as_ptr().hash(state),
            Object::HashTable(hashtable) => ptr::addr_of!(*hashtable).hash(state),
            Object::String(string) => string.hash(state),
            Object::Function(function) => ptr::addr_of!(*function).hash(state),
            Object::Return(_) => state.write_u16(1337),
        }
    }
}

impl Eq for Object {}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nil, Self::Nil) => true,
            (Self::Int(lhs), Self::Int(rhs)) => lhs == rhs,
            (Self::Bool(lhs), Self::Bool(rhs)) => lhs == rhs,
            (Self::String(lhs), Self::String(rhs)) => lhs == rhs,
            (Self::BuiltInFn(lhs), Self::BuiltInFn(rhs)) => lhs == rhs,
            (Self::Array(lhs), Self::Array(rhs)) => ptr::eq(lhs, rhs),
            (Self::HashTable(lhs), Self::HashTable(rhs)) => ptr::eq(lhs, rhs),
            (Self::Function(lhs), Self::Function(rhs)) => ptr::eq(lhs, rhs),
            (Self::Return(lhs), Self::Return(rhs)) => false,
            _ => false,
        }
    }
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum BuiltInFn {
    Len,
    First,
    Last,
    Rest,
    Push,
    Puts,
}

#[derive(Debug, Serialize, Clone)]
pub struct Array(pub Vec<Object>);

pub struct HashTableKey(Object);

#[derive(Debug, Serialize, Clone)]
pub struct HashTable(pub HashMap<Object, Object>);

impl HashTable {
    pub fn new(storage: HashMap<Object, Object>) -> Self {
        Self(storage)
    }
}

impl From<Array> for Object {
    fn from(vec: Array) -> Self {
        Self::Array(vec)
    }
}

impl Array {
    pub fn new(storage: Vec<Object>) -> Self {
        Self(storage)
    }

    pub fn push(&mut self, obj: Object) {
        self.0.push(obj)
    }
}

impl Deref for Array {
    type Target = Vec<Object>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Array {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl BuiltInFn {
    pub const fn name(&self) -> &'static str {
        match self {
            BuiltInFn::Len => "len",
            BuiltInFn::First => "first",
            BuiltInFn::Last => "last",
            BuiltInFn::Rest => "rest",
            BuiltInFn::Push => "push",
            BuiltInFn::Puts => "puts",
        }
    }
}

type Result<T> = std::result::Result<T, EvalError>;

pub const TRUE: Object = Object::Bool(true);
pub const FALSE: Object = Object::Bool(false);
pub const NIL: Object = Object::Nil;
pub const ZERO: Object = Object::Int(0);
pub const EMPTY_STRING: Object = Object::String(SmolStr::new_inline(""));

#[derive(Error, Debug)]
pub enum EvalError {
    #[error("operator `{operator:?}` not supported for value {operand:?}")]
    UnaryOpError {
        operator: UnaryOperator,
        operand: String,
    },
    #[error("operator `{operator:?}` not supported between values {lhs:?} and {rhs:?}")]
    BinaryOpError {
        operator: BinaryOperator,
        lhs: String,
        rhs: String,
    },
    #[error("{target} coercion unsuported for {value:?}")]
    CoercionError { target: &'static str, value: String },
    #[error("Identifier {0} already defined")]
    IdentifierAlreadyDefined(Identifier),
}

#[derive(Serialize, Clone, Default)]
pub struct Environment {
    //maybe use a single hashmap with the function name as key
    curr: HashMap<SmolStr, Object>,
    #[serde(skip_serializing)]
    outer: Option<SharedEnv>,
}

impl fmt::Debug for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Environment")
            .field("curr", &self.curr)
            .finish()
    }
}

pub type SharedEnv = Rc<RefCell<Box<Environment>>>;

impl Environment {
    pub fn new() -> Self {
        Self {
            curr: HashMap::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(outer: SharedEnv, curr: HashMap<SmolStr, Object>) -> SharedEnv {
        Rc::new(RefCell::new(Box::new(Self {
            curr,
            outer: Some(outer),
        })))
    }

    pub fn get(&self, name: &Identifier) -> Option<Object> {
        self.curr
            .get(&name.inner())
            .cloned()
            .or_else(|| self.outer.as_ref().and_then(|x| x.borrow().get(name)))
            .or_else(|| match name.inner().as_str() {
                //TODO: find some way to detect when there is a case missing here
                "len" => Some(Object::BuiltInFn(BuiltInFn::Len)),
                "first" => Some(Object::BuiltInFn(BuiltInFn::First)),
                "last" => Some(Object::BuiltInFn(BuiltInFn::Last)),
                "rest" => Some(Object::BuiltInFn(BuiltInFn::Rest)),
                "push" => Some(Object::BuiltInFn(BuiltInFn::Push)),
                "puts" => Some(Object::BuiltInFn(BuiltInFn::Puts)),
                _ => None,
            })
    }

    pub fn try_insert(&mut self, ident: &Identifier, value: Object) -> Result<()> {
        match self.curr.entry(ident.inner()) {
            Entry::Occupied(_) => Err(EvalError::IdentifierAlreadyDefined(ident.clone())),
            Entry::Vacant(entry) => {
                entry.insert(value);
                Ok(())
            }
        }
    }
}

#[derive(Serialize, Clone)]
pub struct Function {
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
    #[serde(skip_serializing)]
    pub env: SharedEnv,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
            .field("parameters", &self.parameters)
            .field("body", &self.body)
            .finish()
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params = self
            .parameters
            .iter()
            .map(|x| x.inner())
            .collect::<Vec<_>>()
            .join(", ");
        write!(f, "fn({}) {{todo()!}}", params)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Nil => write!(f, "nil"),
            Object::Int(int) => write!(f, "{}", int),
            Object::Bool(bool) => write!(f, "{}", bool),
            Object::String(str) => write!(f, "{}", str),
            Object::BuiltInFn(builtin) => write!(f, "{}", builtin.name()),
            Object::Function(function) => write!(f, "{}", function),
            Object::Return(obj) => write!(f, "{}", obj),
            Object::HashTable(hash) => write!(
                f,
                "{{{}}}",
                hash.0
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Object::Array(array) => write!(
                f,
                "[{}]",
                array
                    .0
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

impl Function {
    pub fn new(parameters: Vec<Identifier>, body: BlockStatement, env: SharedEnv) -> Self {
        Self {
            parameters,
            body,
            env,
        }
    }
}

impl Object {
    pub fn as_typeof(&self) -> &'static str {
        match self {
            Object::Nil => "nil",
            Object::Int(_) => "int",
            Object::Bool(_) => "bool",
            Object::String(_) => "string",
            Object::BuiltInFn(builtin) => builtin.name(),
            Object::Function(_) => "function",
            Object::Return(_) => "return",
            Object::Array(_) => "array",
            Object::HashTable(_) => "object",
        }
    }

    pub fn into_string(self) -> Result<SmolStr> {
        Ok(match self {
            Object::String(str) => str,
            Object::Int(int) => int.to_string().into(),
            Object::Bool(bool) => bool.to_string().into(),
            other => {
                return Err(EvalError::CoercionError {
                    target: EMPTY_STRING.as_typeof(),
                    value: other.to_string(),
                })
            }
        })
    }

    pub fn into_int(self) -> Result<i64> {
        Ok(match self {
            Object::Int(int) => int,
            Object::Bool(bool) => bool.into(),
            Object::String(value) => match value.parse() {
                Ok(int) => int,
                Err(_) => Err(EvalError::CoercionError {
                    target: ZERO.as_typeof(),
                    value: Object::String(value).to_string(),
                })?,
            },
            other => {
                return Err(EvalError::CoercionError {
                    target: ZERO.as_typeof(),
                    value: other.to_string(),
                })
            }
        })
    }

    pub fn into_bool(self) -> Result<bool> {
        Ok(match self {
            Object::Nil => false,
            Object::Int(int) => int != 0,
            Object::Bool(bool) => bool,
            Object::String(str) => !str.is_empty(),
            Object::Function(_) => true,
            other => {
                return Err(EvalError::CoercionError {
                    target: TRUE.as_typeof(),
                    value: other.to_string(),
                })
            }
        })
    }

    pub fn not(self) -> Result<Object> {
        Ok((!self.into_bool()?).into())
    }

    pub fn minus(self) -> Result<Object> {
        match self {
            Object::Int(int) => Ok(Object::Int(-int)),
            operand => Err(EvalError::UnaryOpError {
                operator: UnaryOperator::Minus,
                operand: operand.to_string(),
            }),
        }
    }

    pub fn add(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs + rhs),
            (Object::String(lhs), Object::String(rhs)) => {
                Object::String(format!("{}{}", lhs, rhs).into())
            }
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Add,
                    lhs: lhs.to_string(),
                    rhs: rhs.to_string(),
                })
            }
        })
    }

    pub fn sub(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs - rhs),
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Sub,
                    lhs: lhs.to_string(),
                    rhs: rhs.to_string(),
                })
            }
        })
    }

    pub fn mul(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs * rhs),
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Mul,
                    lhs: lhs.to_string(),
                    rhs: rhs.to_string(),
                })
            }
        })
    }

    pub fn div(self, rhs: Object) -> Result<Object> {
        Ok(match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => Object::Int(lhs / rhs),
            (lhs, rhs) => {
                return Err(EvalError::BinaryOpError {
                    operator: BinaryOperator::Div,
                    lhs: lhs.to_string(),
                    rhs: rhs.to_string(),
                })
            }
        })
    }

    pub fn eq(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs == rhs,
            (Object::String(lhs), Object::String(rhs)) => lhs == rhs,
            (Object::Bool(lhs), Object::Bool(rhs)) => lhs == rhs,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }

    pub fn not_eq(self, rhs: Object) -> bool {
        !self.eq(rhs)
    }

    pub fn lt(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs < rhs,
            _ => false,
        }
    }

    pub fn gt(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs > rhs,
            _ => false,
        }
    }

    pub fn lte(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs <= rhs,
            _ => false,
        }
    }

    pub fn gte(self, rhs: Object) -> bool {
        match (self, rhs) {
            (Object::Int(lhs), Object::Int(rhs)) => lhs >= rhs,
            _ => false,
        }
    }
}

impl From<bool> for Object {
    fn from(bool: bool) -> Self {
        if bool {
            TRUE
        } else {
            FALSE
        }
    }
}
