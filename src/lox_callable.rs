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
}

impl LoxFunction {
    pub fn new(p: Vec<Token>, b: Box<Vec<Stmt>>) -> Self {
        Self { params: p, body: b }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        // 1. 进入新作用域
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
