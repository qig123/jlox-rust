use std::collections::HashMap;

use crate::{
    interpreter::RuntimeError,
    token::{Object, Token},
};

pub struct Environment {
    values: HashMap<String, Object>,
}
impl Environment {
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }
    pub fn get(&self, name: Token) -> Result<Object, RuntimeError> {
        self.values
            .get(&name.lexeme)
            .cloned()
            .ok_or_else(|| RuntimeError {
                message: format!("Undefined variable '{}'", name.lexeme),
                line: name.line,
            })
    }
    pub fn assign(&mut self, name: Token, value: Object) -> Result<Object, RuntimeError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme, value.clone());
            Ok(value.clone())
        } else {
            Err(RuntimeError {
                message: format!("Undefined variable '{}'", name.lexeme),
                line: name.line,
            })
        }
    }
}
