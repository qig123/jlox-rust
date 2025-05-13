use crate::{
    expr::Expr,
    report,
    token::{Object, Token},
    token_type::TokenType,
};
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
}

pub struct Parser {
    current: usize,
    tokens: Vec<Token>,
}
impl Parser {
    pub fn new(t: Vec<Token>) -> Self {
        Parser {
            current: 0,
            tokens: t,
        }
    }
    //
    pub fn parse(&mut self) -> Result<Expr, bool> {
        self.expression().map_err(|err| {
            report::error(err.line, &err.message);
            self.synchronize();
            false  
        })
    }
   //遇到解析错误（ParseError）后，让解析器“对齐”到下一个合理的语句起点，从而继续解析剩余代码，而不是整个程序直接中断。
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }
            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For | TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
                    return;
                }
                _ => {
                    break;
                }
            }
        }
    }   
    fn expression(&mut self) -> Result<Expr, ParseError> {
       self.equality()
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
                _ => {
                    report::error(prev.line, "Expected number or string literal at line ");
                    Err(ParseError { message: "Expected number or string literal at line".to_string(), line: prev.line })
                }
            };
        }
        // 处理分组表达式
        if self.match_token(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        // 如果没有匹配到任何情况，返回错误
        let token = self.peek();
        report::error(token.line, "Expected expression at line");
        Err(ParseError { message: "Expected expression at line".to_string(), line: token.line })
    }

    // 需要添加的 consume 方法
    fn consume(&mut self, t: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(t) {
            Ok(self.advance())
        } else {
            Err(ParseError { message: message.to_string(), line: self.peek().line })
        }
    }
}
