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
}
