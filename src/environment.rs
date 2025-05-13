use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{
    interpreter::RuntimeError,
    token::{Object, Token},
};
#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Rc<RefCell<Environment>>>, //
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }
    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }
    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: Token) -> Result<Object, RuntimeError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            match value {
                Object::Uninitialized => Err(RuntimeError {
                    message: format!("Variable '{}' must be initialized before use", name.lexeme),
                    line: name.line,
                }),
                _ => Ok(value.clone()),
            }
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow().get(name)
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'", name.lexeme),
                line: name.line,
            })
        }
    }
    pub fn assign(&mut self, name: Token, value: Object) -> Result<Object, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.clone());
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            enclosing.borrow_mut().assign(name, value)
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'", name.lexeme),
                line: name.line,
            })
        }
    }
}
