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
                        TokenType::Dot,
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
                '!' => {
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '=' {
                            let mut lexeme = c.to_string();
                            lexeme.push(next_char);
                            tokens.push(Token::new(
                                TokenType::BangEqual,
                                lexeme,
                                Literal::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Bang,
                                c.to_string(),
                                Literal::NULL,
                                self.line,
                            ));
                        }
                    }
                }
                '=' => {
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '=' {
                            let mut lexeme = c.to_string();
                            lexeme.push(next_char);
                            tokens.push(Token::new(
                                TokenType::EqualEqual,
                                lexeme,
                                Literal::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Equal,
                                c.to_string(),
                                Literal::NULL,
                                self.line,
                            ));
                        }
                    }
                }
                '>' => {
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '=' {
                            let mut lexeme = c.to_string();
                            lexeme.push(next_char);
                            tokens.push(Token::new(
                                TokenType::GreaterEqual,
                                lexeme,
                                Literal::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Greater,
                                c.to_string(),
                                Literal::NULL,
                                self.line,
                            ));
                        }
                    }
                }
                '<' => {
                    if let Some(&next_char) = chars.peek() {
                        if next_char == '=' {
                            let mut lexeme = c.to_string();
                            lexeme.push(next_char);
                            tokens.push(Token::new(
                                TokenType::LessEqual,
                                lexeme,
                                Literal::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Less,
                                c.to_string(),
                                Literal::NULL,
                                self.line,
                            ));
                        }
                    }
                }
                '/' => {
                    if let Some(&'/') = chars.peek() {
                        // 单行注释处理
                        while let Some(&next_char) = chars.peek() {
                            if next_char == '\n' {
                                break;
                            }
                            chars.next();
                        }
                    } else {
                        tokens.push(Token::new(
                            TokenType::Slash,
                            c.to_string(),
                            Literal::NULL,
                            self.line,
                        ));
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => {
                    self.line += 1;
                }

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
