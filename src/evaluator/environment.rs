use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::evaluator::builtins::BuiltinFunctions;
use crate::evaluator::object::{BuiltinFunction, Object};
use crate::parser::ast::Ident;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        let mut hashmap = HashMap::new();
        Self::fill_env_with_builtins(&mut hashmap);
        Environment {
            store: hashmap,
            parent: None,
        }
    }

    pub fn new_with_outer(outer: Rc<RefCell<Environment>>) -> Self {
        let mut hashmap = HashMap::new();
        Self::fill_env_with_builtins(&mut hashmap);
        Environment {
            store: hashmap,
            parent: Some(outer),
        }
    }

    fn fill_env_with_builtins(hashmap: &mut HashMap<String, Object>) {
        let builtins_functions = BuiltinFunctions::new();
        let builtins = builtins_functions.get_builtins();
        for (Ident(name), object) in builtins {
            hashmap.insert(name, object);
        }
    }
    
    pub fn set(&mut self, name: &str, val: Object) {
        self.store.insert(name.to_string(), val);
    }
    
    pub fn get(&self, name: &str) -> Option<Object> {
        match self.store.get(name) {
            Some(o) => Some(o.clone()),
            None => match self.parent {
                Some(ref parent_env) => {
                    let env = parent_env.borrow();
                    env.get(name)
                },
                None => None,
            },
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}