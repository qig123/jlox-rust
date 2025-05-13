use crate::{
    report,
    token::{Object, Token},
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
    pub fn scan_tokens(&mut self, source: String) -> Result<Vec<Token>, ()> {
        let mut tokens = Vec::new();
        let mut chars = source.chars().peekable();
        let mut had_error = false;
        while let Some(c) = chars.next() {
            match c {
                '(' => {
                    tokens.push(Token::new(
                        TokenType::LeftParen,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                ')' => {
                    tokens.push(Token::new(
                        TokenType::RightParen,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                '{' => {
                    tokens.push(Token::new(
                        TokenType::LeftBrace,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                '}' => {
                    tokens.push(Token::new(
                        TokenType::RightBrace,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                ',' => {
                    tokens.push(Token::new(
                        TokenType::Comma,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                '.' => {
                    tokens.push(Token::new(
                        TokenType::Dot,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                '-' => {
                    tokens.push(Token::new(
                        TokenType::Minus,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                '+' => {
                    tokens.push(Token::new(
                        TokenType::Plus,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                ';' => {
                    tokens.push(Token::new(
                        TokenType::Semicolon,
                        c.to_string(),
                        Object::NULL,
                        self.line,
                    ));
                }
                '*' => {
                    tokens.push(Token::new(
                        TokenType::Star,
                        c.to_string(),
                        Object::NULL,
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
                                Object::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Bang,
                                c.to_string(),
                                Object::NULL,
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
                                Object::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Equal,
                                c.to_string(),
                                Object::NULL,
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
                                Object::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Greater,
                                c.to_string(),
                                Object::NULL,
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
                                Object::NULL,
                                self.line,
                            ));
                            chars.next();
                        } else {
                            tokens.push(Token::new(
                                TokenType::Less,
                                c.to_string(),
                                Object::NULL,
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
                            Object::NULL,
                            self.line,
                        ));
                    }
                }
                ' ' | '\r' | '\t' => {}
                '\n' => {
                    self.line += 1;
                }
                '"' => {
                    let mut string_content = String::new();
                    // 处理字符串内容
                    while let Some(&next_char) = chars.peek() {
                        if next_char == '"' {
                            break; // 找到闭合引号
                        }
                        if next_char == '\n' {
                            self.line += 1;
                        }
                        string_content.push(chars.next().unwrap()); // 消费字符并添加到内容
                    }
                    // 检查是否到达文件末尾而未闭合
                    if chars.peek().is_none() {
                        report::error(self.line, "Unterminated string.");
                        had_error = true;
                    } else {
                        // 消费闭合引号
                        chars.next();

                        tokens.push(Token::new(
                            TokenType::String,
                            string_content.clone(),         // 实际字符串内容
                            Object::String(string_content), // 存储为字面量
                            self.line,
                        ));
                    }
                }
                c if c.is_ascii_digit() => {
                    let mut number_literal = c.to_string();

                    // 收集整数部分
                    while let Some(&next_char) = chars.peek() {
                        if next_char.is_ascii_digit() {
                            number_literal.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    // 查看是否存在小数部分
                    if let Some(&'.') = chars.peek() {
                        if let Some(next_next_char) = chars.clone().nth(1) {
                            if next_next_char.is_ascii_digit() {
                                number_literal.push(chars.next().unwrap()); // 消费 '.'

                                // 收集小数部分
                                while let Some(&next_char) = chars.peek() {
                                    if next_char.is_ascii_digit() {
                                        number_literal.push(chars.next().unwrap());
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    // 尝试解析为 f64
                    match number_literal.parse::<f64>() {
                        Ok(value) => {
                            tokens.push(Token::new(
                                TokenType::Number,
                                number_literal.clone(),
                                Object::Number(value),
                                self.line,
                            ));
                        }
                        Err(_) => {
                            report::error(self.line, "Invalid number literal.");
                            had_error = true;
                        }
                    }
                }
                c if Scanner::is_alpha(c) => {
                    let mut identifier = c.to_string();

                    // 收集后续的字母数字字符
                    while let Some(&next_char) = chars.peek() {
                        if Scanner::is_alpha_numeric(next_char) {
                            identifier.push(chars.next().unwrap());
                        } else {
                            break;
                        }
                    }
                    // 检查是否是关键字
                    let token_type = match identifier.as_str() {
                        "and" => TokenType::And,
                        "class" => TokenType::Class,
                        "else" => TokenType::Else,
                        "false" => TokenType::False,
                        "fun" => TokenType::Fun,
                        "for" => TokenType::For,
                        "if" => TokenType::If,
                        "nil" => TokenType::Nil,
                        "or" => TokenType::Or,
                        "print" => TokenType::Print,
                        "return" => TokenType::Return,
                        "super" => TokenType::Super,
                        "this" => TokenType::This,
                        "true" => TokenType::True,
                        "var" => TokenType::Var,
                        "while" => TokenType::While,
                        _ => TokenType::Identifier,
                    };

                    tokens.push(Token::new(token_type, identifier, Object::NULL, self.line));
                }

                _ => {
                    report::error(self.line, "Unexpected character.");
                    had_error = true;
                }
            }
        }
        tokens.push(Token::new(
            TokenType::Eof,
            "".to_string(),
            Object::NULL,
            self.line,
        ));
        if had_error { Err(()) } else { Ok(tokens) }
    }

    fn is_alpha(c: char) -> bool {
        return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
    }

    fn is_alpha_numeric(c: char) -> bool {
        return Self::is_alpha(c) || c.is_ascii_digit();
    }
}
