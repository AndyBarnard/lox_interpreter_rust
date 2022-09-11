use crate::tokentype::*;
use crate::scanner;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: String,
    pub line: u32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: String, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line
        }
    }

    fn to_string(&self) -> String {
        String::from("{self.token_type} {lexeme} {literal}")
    }
}
