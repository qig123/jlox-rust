use crate::token_type::TokenType;
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    NULL,
    Number(f64),
    Boolean(bool),
}
#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}
impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Literal, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }
}
