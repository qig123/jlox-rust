use std::{cell::RefCell, rc::Rc};

use crate::{
    environment::Environment,
    expr::{Expr, Stmt},
    token::Object,
    token_type::TokenType,
};
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}
pub struct Interpreter {
    environment: Environment,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Environment::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Result<Object, RuntimeError> {
        for stmt in statements {
            self.interpret_stmt(stmt)?;
        }
        Ok(Object::NULL)
    }

    pub fn interpret_expr(&mut self, expr: Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Literal(value) => Ok(value),
            Expr::Grouping(expr) => self.interpret_expr(*expr),
            Expr::Unary { operator, right } => {
                let right = self.interpret_expr(*right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        if let Object::Number(value) = right {
                            Ok(Object::Number(-value))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be a number".to_string(),
                                line: operator.line,
                            })
                        }
                    }
                    TokenType::Bang => Ok(Object::Boolean(!Self::is_truthy(&right))),
                    _ => Err(RuntimeError {
                        message: "Operands must be a number".to_string(),
                        line: operator.line,
                    }),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.interpret_expr(*left)?;
                let right = self.interpret_expr(*right)?;
                match operator.token_type {
                    TokenType::Plus => {
                        // 处理数字相加或字符串连接
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Number(a + b))
                        } else if let (Object::String(a), Object::String(b)) = (&left, &right) {
                            Ok(Object::String(format!("{}{}", a, b)))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers or two strings".to_string(),
                                line: operator.line,
                            }) //"Operands must be two numbers or two strings"
                        }
                    }
                    TokenType::Minus => {
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Number(a - b))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers".to_string(),
                                line: operator.line,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::Star => {
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Number(a * b))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers".to_string(),
                                line: operator.line,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::Slash => {
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Number(a / b))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers".to_string(),
                                line: operator.line,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::Greater => {
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Boolean(a > b))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers".to_string(),
                                line: operator.line,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::GreaterEqual => {
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Boolean(a >= b))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers".to_string(),
                                line: 0,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::Less => {
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Boolean(a < b))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers".to_string(),
                                line: operator.line,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::LessEqual => {
                        if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                            Ok(Object::Boolean(a <= b))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be two numbers".to_string(),
                                line: operator.line,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::EqualEqual => Ok(Object::Boolean(Self::is_equal(&left, &right))),
                    TokenType::BangEqual => Ok(Object::Boolean(!Self::is_equal(&left, &right))),
                    _ => {
                        Err(RuntimeError {
                            message: "Operands must be two numbers".to_string(),
                            line: operator.line,
                        }) //"Operands must be two numbers"
                    }
                }
            }
            Expr::Variable(name) => self.environment.get(name),
            Expr::Assign { name, value } => {
                let value = self.interpret_expr(*value)?;
                self.environment.assign(name, value)
            }
        }
    }
    pub fn interpret_stmt(&mut self, stmt: Stmt) -> Result<Object, RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => self.interpret_expr(expr),
            Stmt::Print(expr) => {
                let e = self.interpret_expr(expr)?;
                println!("{}", Self::stringify(&e));
                Ok(Object::NULL)
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.interpret_expr(expr)?,
                    None => Object::Uninitialized,
                };
                self.environment.define(name.lexeme, value);
                Ok(Object::NULL)
            }
            Stmt::Block { statements } => self.execute_block(
                statements,
                Environment::new_with_enclosing(Rc::new(RefCell::new(self.environment.clone()))), //这里应该怎样写
            ),
        }
    }
    fn execute_block(
        &mut self,
        statements: Vec<Stmt>,
        environment: Environment,
    ) -> Result<Object, RuntimeError> {
        let previous = std::mem::replace(&mut self.environment, environment);
        let result = (|| {
            for stmt in statements {
                self.interpret_stmt(stmt)?;
            }
            Ok(Object::NULL)
        })(); // 立即执行闭包
        self.environment = previous; // 无论如何都会恢复
        result
    }
    // 辅助函数：判断一个值是否为真
    fn is_truthy(obj: &Object) -> bool {
        match obj {
            Object::NULL => false,
            Object::Boolean(b) => *b,
            _ => true,
        }
    }
    fn is_equal(a: &Object, b: &Object) -> bool {
        match (a, b) {
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::NULL, Object::NULL) => true,
            _ => false,
        }
    }

    fn stringify(obj: &Object) -> String {
        match obj {
            Object::Number(value) => value.to_string(),
            Object::String(value) => value.clone(),
            Object::Boolean(value) => value.to_string(),
            Object::NULL => "null".to_string(),
            Object::Uninitialized => "uninitialized".to_string(),
        }
    }
}
