use indextree::NodeId;

use crate::{
    expr::Stmt,
    interpreter::{Interpreter, RuntimeError},
    token::{Object, Token},
};
pub trait LoxCallable {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;
    fn arity(&self) -> usize;
}
#[derive(Debug, Clone)]
pub struct LoxFunction {
    pub params: Vec<Token>,
    pub body: Box<Vec<Stmt>>,
    pub closure_env: NodeId, // 只保存闭包创建时的环境节点ID
}

impl LoxFunction {
    pub fn new(p: Vec<Token>, b: Box<Vec<Stmt>>, closure_env: NodeId) -> Self {
        Self {
            params: p,
            body: b,
            closure_env, // 存储闭包创建时的环境节点
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        // 1. 进入新作用域
        // 保存旧环境
        let old_env = interpreter.environment.current;

        // 切换到闭包环境（使用解释器原有的 arena）
        interpreter.environment.current = self.closure_env;

        // 进入新作用域（在闭包环境的基础上）
        interpreter.environment.enter_child_scope();

        // 2. 绑定参数
        for (i, param) in self.params.iter().enumerate() {
            interpreter.environment.define(
                param.lexeme.clone(),
                arguments
                    .get(i)
                    .ok_or_else(|| RuntimeError {
                        message: format!("Missing argument for '{}'", param.lexeme),
                        line: param.line,
                        value: None,
                    })?
                    .clone(),
            );
        }
        // 执行函数体
        let result = interpreter.execute_block(&self.body);

        interpreter.environment.exit_scope();
        interpreter.environment.current = old_env;

        match result {
            Ok(value) => Ok(value), // 有 return 值
            Err(e) => {
                if e.value.is_some() {
                    Ok(e.value.unwrap()) // return 传播的错误
                } else {
                    Err(e) // 真正的错误
                }
            }
        }
    }

    fn arity(&self) -> usize {
        self.params.len()
    }
}
