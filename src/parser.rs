use crate::{
    expr::{Expr, Stmt},
    report,
    token::{Object, Token},
    token_type::TokenType,
};
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
    source_lines: Vec<String>,
}
impl Parser {
    pub fn new(t: Vec<Token>, source: String) -> Self {
        Parser {
            current: 0,
            tokens: t,
            source_lines: source.lines().map(String::from).collect(),
        }
    }
    pub fn parse(&mut self) -> Result<Vec<Stmt>, ()> {
        // 用 `()` 表示"有错误"，无额外信息
        let mut statements = Vec::new();
        let mut had_error = false; // 只需记录是否出错

        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    report::report_error(err.line, &self.source_lines, err.column, &err.message);
                    self.synchronize(); // 同步恢复
                    had_error = true; // 标记出错
                }
            }
        }

        if had_error {
            Err(()) // 返回简单错误标志
        } else {
            Ok(statements) // 全部成功
        }
    }

    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Var]) {
            return self.var_declaration();
        }
        self.statement()
    }
    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();
        let initializer = if self.match_token(&[TokenType::Equal]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var { name, initializer })
    }
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_token(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::LeftBrace]) {
            return self.block_statement();
        }
        if self.match_token(&[TokenType::If]) {
            return self.if_statement();
        }
        self.expression_statement()
    }
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;
        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.match_token(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }
    fn block_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut statements = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;
        Ok(Stmt::Block { statements })
    }
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        // 1. 先解析等号左边的表达式（可能是变量或其他表达式）
        let expr = self.equality()?;

        // 2. 检查当前token是否是等号（表示这是一个赋值语句）
        if self.match_token(&[TokenType::Equal]) {
            // 3. 递归解析等号右边的表达式
            let value = self.assignment()?;
            let equals = self.previous(); // 获取等号token用于错误定位

            // 4. 检查左边表达式是否是变量（唯一合法的赋值目标）
            match expr {
                Expr::Variable(name) => {
                    // 合法情况：创建赋值表达式节点
                    return Ok(Expr::Assign {
                        name,                   // 变量名
                        value: Box::new(value), // 要赋的值
                    });
                }
                _ => {
                    // 非法情况：左边不是变量（如 `1+1 = 2` 这种非法语法）
                    return Err(ParseError {
                        message: "Invalid assignment target.".to_string(),
                        line: equals.line,
                        column: equals.column,
                    });
                }
            }
        }

        // 5. 如果不是赋值语句，直接返回解析的表达式
        Ok(expr)
    }
    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }
    //遇到解析错误（ParseError）后，让解析器"对齐"到下一个合理的语句起点，从而继续解析剩余代码，而不是整个程序直接中断。
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {
                    break;
                }
            }
        }
    }
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn match_token(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, t: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == t
    }
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn is_at_end(&self) -> bool {
        let p = self.peek();
        p.token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?; // 初始解析一元表达式
        // 持续处理 * 和 / 运算符
        while self.match_token(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone(); // 获取操作符
            let right = self.unary()?; // 解析右侧表达式
            expr = Expr::Binary {
                left: Box::new(expr), // 包装左表达式
                operator,
                right: Box::new(right), // 包装右表达式
            };
        }

        Ok(expr) // 返回最终表达式
    }
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_token(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right_expr = self.comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr), // 包装左表达式
                operator,
                right: Box::new(right_expr), // 包装右表达式
            };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_token(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right_expr = self.term()?;
            expr = Expr::Binary {
                left: Box::new(expr), // 包装左表达式
                operator,
                right: Box::new(right_expr), // 包装右表达式
            };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_token(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right_expr = self.factor()?;
            expr = Expr::Binary {
                left: Box::new(expr), // 包装左表达式
                operator,
                right: Box::new(right_expr), // 包装右表达式
            };
        }
        Ok(expr)
    }
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone(); // 需要克隆 Token
            let right = self.unary()?; // 使用 ? 处理可能的错误

            return Ok(Expr::Unary {
                operator,
                right: Box::new(right), // 用 Box 包装
            });
        }

        self.primary() // 直接返回 primary() 的结果
    }
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token(&vec![TokenType::False]) {
            return Ok(Expr::Literal(crate::token::Object::Boolean(false)));
        }
        if self.match_token(&vec![TokenType::True]) {
            return Ok(Expr::Literal(crate::token::Object::Boolean(true)));
        }
        if self.match_token(&vec![TokenType::Nil]) {
            return Ok(Expr::Literal(crate::token::Object::NULL));
        }
        if self.match_token(&vec![TokenType::Number, TokenType::String]) {
            let prev = self.previous();
            return match &prev.literal {
                Object::Number(n) => Ok(Expr::Literal(Object::Number(*n))),
                Object::String(s) => Ok(Expr::Literal(Object::String(s.clone()))),
                _ => Err(ParseError {
                    message: "Expected number or string literal at line".to_string(),
                    line: prev.line,
                    column: prev.column,
                }),
            };
        }
        if self.match_token(&[TokenType::Identifier]) {
            let name = self.previous().clone();
            return Ok(Expr::Variable(name));
        }
        // 处理分组表达式
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?; //语法分析，这里能否也显示一下出错字符串的位置
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        // 如果没有匹配到任何情况，返回错误
        let token = self.peek();

        Err(ParseError {
            message: "Expected expression at line".to_string(),
            line: token.line,
            column: token.column,
        })
    }

    // 检查当前 token 是否是我们期望的类型
    // 如果是，就消费掉这个 token（移动到下一个）
    // 如果不是，就报告一个语法错误
    // 举个例子，对于这样的表达式：(1 + 2
    // 当解析到 2 后，我们会调用 consume(TokenType::RightParen, "Expect ')' after expression.")
    // 如果当前 token 是 TokenType::RightParen，就消费掉这个 token，并返回 Ok(token)
    // 如果当前 token 不是 TokenType::RightParen，就报告一个语法错误，并返回 Err(ParseError)
    fn consume(&mut self, expected_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        let current_token = self.peek();
        if self.check(expected_type) {
            Ok(self.advance())
        } else {
            // 获取上一个token的位置，这个位置更准确地表示了错误发生的地方
            let prev_token = self.previous();

            // 构造更有信息量的错误消息
            let error_message = format!("{} (found '{}' instead)", message, current_token.lexeme);

            Err(ParseError {
                message: error_message,
                line: prev_token.line, // 使用上一个token的位置
                column: prev_token.column + prev_token.lexeme.len(), // 在上一个token之后
            })
        }
    }
}
