use crate::{
    token::{Literal, Token},
    token_type::TokenType,
};
#[derive(Debug)]
pub struct Scanner {
    line: usize,
}

impl Scanner {
    fn new() -> Self {
        Scanner { line: 1 }
    }
    fn scan_tokens(&mut self, source: String) -> Vec<Token> {
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
                _ => {}
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
