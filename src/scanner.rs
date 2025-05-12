use crate::{
    report,
    token::{Literal, Token},
    token_type::TokenType,
};
#[derive(Debug)]
pub struct Scanner {
    line: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Scanner { line: 1 }
    }
    pub fn scan_tokens(&mut self, source: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = source.chars().peekable();

        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    tokens.push(Token::new(
                        TokenType::LeftParen,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                ')' => {
                    tokens.push(Token::new(
                        TokenType::RightParen,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                '{' => {
                    tokens.push(Token::new(
                        TokenType::LeftBrace,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                '}' => {
                    tokens.push(Token::new(
                        TokenType::RightBrace,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                ',' => {
                    tokens.push(Token::new(
                        TokenType::Comma,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                '.' => {
                    tokens.push(Token::new(
                        TokenType::Semicolon,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                '-' => {
                    tokens.push(Token::new(
                        TokenType::Minus,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                '+' => {
                    tokens.push(Token::new(
                        TokenType::Plus,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                ';' => {
                    tokens.push(Token::new(
                        TokenType::Semicolon,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                '*' => {
                    tokens.push(Token::new(
                        TokenType::Star,
                        c.to_string(),
                        Literal::NULL,
                        self.line,
                    ));
                }
                //处理前瞻一个字符的情况
                _ => {
                    report::error(self.line, "Unexpected character.");
                }
            }
        }
        tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Literal::NULL,
            self.line,
        ));
        tokens
    }
}
