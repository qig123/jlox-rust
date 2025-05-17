use std::collections::HashMap;

use indextree::{Arena, NodeId};

use crate::{
    interpreter::RuntimeError,
    token::{Object, Token},
};
#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, Object>,
}
#[derive(Debug)]
pub struct EnvironmentTree {
    arena: Arena<Environment>,
    pub current: NodeId,
}
impl EnvironmentTree {
    pub fn new() -> Self {
        let mut arena = Arena::new();
        let global = arena.new_node(Environment {
            values: HashMap::new(),
        });
        Self {
            arena,
            current: global,
        }
    }
    pub fn define(&mut self, name: String, value: Object) {
        self.arena[self.current]
            .get_mut()
            .values
            .insert(name, value);
    }

    pub fn get(&self, name: Token) -> Result<Object, RuntimeError> {
        for node_id in self.current.ancestors(&self.arena) {
            let env = &self.arena[node_id].get();
            if let Some(val) = env.values.get(&name.lexeme) {
                return Ok(val.clone());
            }
        }
        Err(RuntimeError {
            message: format!("Undefined variable '{}'", name.lexeme),
            line: name.line,
            value: None,
        })
    }
    pub fn assign(&mut self, name: Token, value: &Object) -> Result<(), RuntimeError> {
        // 遍历作用域链寻找变量定义
        let node_ids: Vec<_> = self.current.ancestors(&self.arena).collect();

        for node_id in node_ids {
            let env = &mut self.arena[node_id].get_mut();

            // 如果找到变量则更新其值
            if env.values.contains_key(&name.lexeme) {
                env.values.insert(name.lexeme, value.clone());
                return Ok(());
            }
        }

        // 未找到变量返回错误
        Err(RuntimeError {
            message: format!("Undefined variable '{}'", name.lexeme),
            line: name.line,
            value: None,
        })
    }
    /// 进入一个新的子作用域
    pub fn enter_child_scope(&mut self) {
        let child = self.arena.new_node(Environment {
            values: HashMap::new(),
        });
        self.current.append(child, &mut self.arena);
        self.current = child;
    }

    /// 回到父作用域
    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.current.ancestors(&self.arena).nth(1) {
            self.current = parent;
        }
    }
}
