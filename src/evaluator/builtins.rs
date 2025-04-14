use crate::evaluator::object::{BuiltinFunction, Object};
use crate::parser::ast::Ident;

pub struct BuiltinFunctions;

impl BuiltinFunctions {
    pub fn new() -> Self {
        BuiltinFunctions{}
    }

    pub fn get_builtins(&self) -> Vec<(Ident, Object)> {
        vec![
            add_builtin("print", 1, bprint_fn),
            add_builtin("len", 1, blen_fn),
            add_builtin("head", 1, bhead_fn),
            add_builtin("tail", 1, btail_fn),
            add_builtin("cons", 2, bcons_fn),
        ]
    }
}

impl Default for BuiltinFunctions {
    fn default() -> Self {
        Self::new()
    }
}

fn add_builtin(name: &str, param_num: usize, func: BuiltinFunction) -> (Ident, Object) {
    let name = name.to_string();
    (Ident(name.clone()), Object::Builtin(name, param_num, func))
}

fn bprint_fn(args: Vec<Object>) -> Result<Object, String> {
    match args.get(0) {
        Some(Object::String(s)) => {
            println!("{}", s);
            Ok(Object::Null)
        },
        Some(o) => {
            println!("{}", o);
            Ok(Object::Null)
        }
        _ => Err(String::from("invalid arguments for print")),
    }
}

fn blen_fn(args: Vec<Object>) -> Result<Object, String> {
    match args.get(0) {
        Some(Object::String(s)) => Ok(Object::Integer(s.len() as i64)),
        Some(Object::Array(arr)) => Ok(Object::Integer(arr.len() as i64)),
        _ => Err(String::from("invalid arguments for len")),
    }
}

fn bhead_fn(args: Vec<Object>) -> Result<Object, String> {
    match args.into_iter().next() {
        Some(Object::Array(arr)) => match arr.into_iter().next() {
            None => Err(String::from("empty array")),
            Some(x) => Ok(x),
        },
        _ => Err(String::from("invalid arguments for head")),
    }
}

fn btail_fn(args: Vec<Object>) -> Result<Object, String> {
    match args.into_iter().next() {
        Some(Object::Array(mut arr)) => match arr.len() {
            0 => Err(String::from("empty array")),
            _ => {
                arr.remove(0);
                Ok(Object::Array(arr))
            }
        }
        _ => Err(String::from("invalid arguments for tail")),
    }
}
fn bcons_fn(args: Vec<Object>) -> Result<Object, String> {
    let mut args = args.into_iter();
    match (args.next(), args.next()) {
        (Some(o), Some(Object::Array(mut os))) => {
            os.insert(0, o);
            Ok(Object::Array(os))
        }
        _ => Err(String::from("invalid arguments for cons")),
    }
}