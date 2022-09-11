use crate::tokentype::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: i32,
}
