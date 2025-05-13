

use crate::{expr::Expr, token::Object, token_type::TokenType};
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
}


pub fn interpret(expr: Expr) -> Result<Object, RuntimeError> {
    match expr {
        Expr::Literal(value) => Ok(value),
        Expr::Grouping(expr) => interpret(*expr),
        Expr::Unary { operator, right } => {
            let right = interpret(*right)?;
            match operator.token_type {
                TokenType::Minus => {
                    if let Object::Number(value) = right {
                        Ok(Object::Number(-value))
                    } else {
                        Err(RuntimeError { message: "Operands must be a number".to_string(), line: operator.line })
                    }
                }
                TokenType::Bang => {
                    Ok(Object::Boolean(!is_truthy(&right)))
                }
                _ => Err(RuntimeError { message: "Operands must be a number".to_string(), line: operator.line }),
            }
        }
        Expr::Binary { left, operator, right } => {
            let left = interpret(*left)?;
            let right = interpret(*right)?;
            match operator.token_type {
                TokenType::Plus => {
                    // 处理数字相加或字符串连接
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right  ) {
                        Ok(Object::Number(a + b))
                    } else if let (Object::String(a), Object::String(b)) = (&left, &right) {
                        Ok(Object::String(format!("{}{}", a, b)))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers or two strings".to_string(), line: operator.line }) //"Operands must be two numbers or two strings"
                    }
                }
                TokenType::Minus => {
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                        Ok(Object::Number(a - b))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: operator.line }) //"Operands must be two numbers"
                    }
                }
                TokenType::Star => {
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                        Ok(Object::Number(a * b))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: operator.line }) //"Operands must be two numbers"
                    }
                }
                TokenType::Slash => {
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                        Ok(Object::Number(a / b))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: operator.line }) //"Operands must be two numbers"
                    }
                }
                TokenType::Greater => {
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                        Ok(Object::Boolean(a > b))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: operator.line }) //"Operands must be two numbers"
                    }
                }
                TokenType::GreaterEqual => {
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                        Ok(Object::Boolean(a >= b))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: 0 }) //"Operands must be two numbers"
                    }
                }   
                TokenType::Less => {
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                        Ok(Object::Boolean(a < b))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: operator.line }) //"Operands must be two numbers"
                    }
                }
                TokenType::LessEqual => {
                    if let (Object::Number(a), Object::Number(b)) = (&left, &right) {
                        Ok(Object::Boolean(a <= b))
                    } else {
                        Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: operator.line }) //"Operands must be two numbers"
                    }
                }
                TokenType::EqualEqual => {
                    Ok(Object::Boolean(is_equal(&left, &right)))
                }
                TokenType::BangEqual => {
                    Ok(Object::Boolean(!is_equal(&left, &right)))
                }   
                _=>{
                    Err(RuntimeError { message: "Operands must be two numbers".to_string(), line: operator.line }) //"Operands must be two numbers"
                }
            }
        }
       
    }
}
// 辅助函数：判断一个值是否为真
fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::NULL=> false,
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
