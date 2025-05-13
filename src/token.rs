use crate::token_type::TokenType;
#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    String(String),
    NULL,
    Number(f64),
    Boolean(bool),
    Uninitialized,
}
#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Object,
    pub line: usize,
    pub column: usize,
}
impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Object,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}
