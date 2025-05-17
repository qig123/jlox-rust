use crate::{
    environment::EnvironmentTree,
    expr::{Expr, Stmt},
    lox_callable::{LoxCallable, LoxFunction},
    token::Object,
    token_type::TokenType,
};
#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
    pub line: usize,
    pub value: Option<Object>,
}

pub struct Interpreter {
    pub environment: EnvironmentTree,
}
impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: EnvironmentTree::new(),
        }
    }
    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        // println!("{:?}", statements);
        for stmt in statements {
            let r = self.interpret_stmt(&stmt);
            match r {
                Ok(_e) => {}
                Err(e) => match e.value {
                    Some(_v) => {}
                    None => {
                        eprintln!("Runtime error: {} at line {}", e.message, e.line)
                    }
                },
            }
        }
    }

    pub fn interpret_expr(&mut self, expr: &Expr) -> Result<Object, RuntimeError> {
        match expr {
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Grouping(expr) => self.interpret_expr(&*expr),
            Expr::Unary { operator, right } => {
                let right = self.interpret_expr(&*right)?;
                match operator.token_type {
                    TokenType::Minus => {
                        if let Object::Number(value) = right {
                            Ok(Object::Number(-value))
                        } else {
                            Err(RuntimeError {
                                message: "Operands must be a number".to_string(),
                                line: operator.line,
                                value: None,
                            })
                        }
                    }
                    TokenType::Bang => Ok(Object::Boolean(!Self::is_truthy(&right))),
                    _ => Err(RuntimeError {
                        message: "Operands must be a number".to_string(),
                        line: operator.line,
                        value: None,
                    }),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left = self.interpret_expr(&*left)?;
                let right = self.interpret_expr(&*right)?;
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
                                value: None,
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
                                value: None,
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
                                value: None,
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
                                value: None,
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
                                value: None,
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
                                value: None,
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
                                value: None,
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
                                value: None,
                            }) //"Operands must be two numbers"
                        }
                    }
                    TokenType::EqualEqual => Ok(Object::Boolean(Self::is_equal(&left, &right))),
                    TokenType::BangEqual => Ok(Object::Boolean(!Self::is_equal(&left, &right))),
                    _ => {
                        Err(RuntimeError {
                            message: "Operands must be two numbers".to_string(),
                            line: operator.line,
                            value: None,
                        }) //"Operands must be two numbers"
                    }
                }
            }
            Expr::Variable(name) => self.environment.get(name.clone()),
            Expr::Assign { name, value } => {
                let value = self.interpret_expr(&*value)?;
                self.environment.assign(name.clone(), &value)?;
                Ok(value) // 返回值无意义
            }
            Expr::Logical {
                left,
                operator,
                right,
            } => {
                let left = self.interpret_expr(&*left)?;
                // println!("左边:{:?}", left);
                match operator.token_type {
                    TokenType::Or => {
                        if Self::is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                    _ => {
                        if !Self::is_truthy(&left) {
                            return Ok(left);
                        }
                    }
                }
                // println!("右边:{:?}", *right);
                self.interpret_expr(&*right)
            }
            Expr::Call {
                callee,
                paren,
                arguments,
            } => {
                // 1. 解析被调用对象（函数）
                let callee_obj = self.interpret_expr(&*callee)?;

                // 2. 解析所有参数表达式
                let mut args = Vec::with_capacity(arguments.len());
                for arg in arguments {
                    args.push(self.interpret_expr(arg)?);
                }

                // 3. 检查是否为可调用对象
                match callee_obj {
                    Object::LoxFunction(func) => {
                        let mut f = func; // 获取可变引用
                        if args.len() != f.arity() {
                            return Err(RuntimeError {
                                message: format!(
                                    "Exprect {} arguments but got {}.",
                                    f.arity(),
                                    args.len()
                                ),
                                line: paren.line,
                                value: None,
                            });
                        }
                        f.call(self, args)
                    }
                    _ => Err(RuntimeError {
                        message: "Can only call functions and classes.".to_string(),
                        line: paren.line,
                        value: None,
                    }),
                }
            }
        }
    }
    pub fn interpret_stmt(&mut self, stmt: &Stmt) -> Result<Object, RuntimeError> {
        match stmt {
            Stmt::Expression(expr) => self.interpret_expr(&expr),
            Stmt::Print(expr) => {
                let e = self.interpret_expr(&expr)?;
                println!("{}", Self::stringify(&e));
                Ok(Object::NULL)
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.interpret_expr(&expr)?,
                    None => Object::NULL,
                };
                self.environment.define(name.lexeme.clone(), value);
                Ok(Object::NULL)
            }
            Stmt::Block(stmts) => {
                self.execute_block(stmts)?;
                Ok(Object::NULL)
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                let c = self.interpret_expr(&condition)?;
                if Self::is_truthy(&c) {
                    self.interpret_stmt(&*then_branch)?;
                } else {
                    match else_branch {
                        Some(e) => {
                            self.interpret_stmt(&*e)?;
                        }
                        None => {}
                    }
                }
                Ok(Object::NULL)
            }
            Stmt::While { condition, body } => {
                // 只在循环开始前创建一个新的作用域
                self.environment.enter_child_scope();
                // 循环条件检查
                while Self::is_truthy(&self.interpret_expr(&condition)?) {
                    // 执行循环体
                    self.interpret_stmt(&*body)?;
                }
                // 循环结束后退出作用域
                self.environment.exit_scope();
                Ok(Object::NULL)
            }
            Stmt::Function { name, params, body } => {
                // 解析时创建FunctionDecl
                let f = LoxFunction::new(params.clone(), body.clone());

                self.environment
                    .define(name.lexeme.clone(), Object::LoxFunction(Box::new(f)));
                Ok(Object::NULL)
            }
            Stmt::Return { keyword, value } => {
                let value_re;
                match value {
                    Some(e) => {
                        value_re = Some(self.interpret_expr(e)?);
                    }
                    None => {
                        value_re = None;
                    }
                }
                //println!("return的值是 {:?}", value_re);
                return Err(RuntimeError {
                    message: "".to_string(),
                    line: 0,
                    value: value_re,
                });
            }
        }
    }
    pub fn execute_block(&mut self, stmts: &[Stmt]) -> Result<Object, RuntimeError> {
        self.environment.enter_child_scope();

        for stmt in stmts {
            // println!("当前执行语句{:?}", stmt);
            match self.interpret_stmt(stmt) {
                Ok(_) => continue,
                Err(e) => {
                    //这个e有可能带return
                    self.environment.exit_scope();
                    return Err(e);
                }
            }
        }
        self.environment.exit_scope();
        Ok(Object::NULL)
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
            Object::LoxFunction(_f) => format!("<fn >"),
        }
    }
}
