use crate::evaluator::environment::Environment;
use crate::parser::ast::{Ident, Program};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Array(Vec<Object>),
    Hash(HashMap<Object, Object>),
    Function(Vec<Ident>, Program, Rc<RefCell<Environment>>),
    Builtin(String, usize, BuiltinFunction),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
}

impl Object {
    pub fn is_returned(&self) -> bool {
        matches!(*self, Object::ReturnValue(_))
    }

    pub fn returned(self) -> Self {
        match self {
            Object::ReturnValue(o) => *o,
            o => o,
        }
    }
}

impl Eq for Object {}

#[allow(clippy::all)]
impl Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Object::Integer( i) => i.hash(state),
            Object::Boolean( b) => b.hash(state),
            Object::String( s) => s.hash(state),
            _ => "".hash(state),
        }
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => {
                if *b {
                    write!(f, "true")
                } else {
                    write!(f, "false")
                }
            }
            Object::String(s) => write!(f, "{}", s),
            Object::Array(v) => {
                let mut fmt_string = String::new();
                fmt_string.push('[');
                for (i, o) in v.iter().enumerate()  {
                    fmt_string.push_str(format!("{}", o).as_str());
                    if i < v.len() - 1 {
                        fmt_string.push_str(", ");
                    }
                }
                fmt_string.push(']');
                write!(f, "{}", fmt_string)
            },
            Object::Hash(hashmap) => {
                let mut  fmt_string = String::new();
                fmt_string.push('{');
                for (i, (k, v)) in hashmap.iter().enumerate() {
                    fmt_string.push_str(format!("{}: {}", k, v).as_str());
                    if i < hashmap.len() - 1 {
                        fmt_string.push_str(", ");
                    }
                }
                fmt_string.push('}');
                write!(f, "{}", fmt_string)
            },
            Object::Function(_, _, _) => write!(f, "[function]"),
            Object::Builtin(name, _, _) => write!(f, "[built-in function: {}]", *name),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(o) => write!(f, "{}", *o),
            Object::Error(s) => write!(f, "Error: {}", s),
        }
    }
}

pub type BuiltinFunction = fn(Vec<Object>) -> Result<Object, String>;
