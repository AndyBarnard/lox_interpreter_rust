// use crate::scanner;
use crate::tokentype::*;

// use display;

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: Box<TokenType>,
    pub lexeme: &'a str,
    pub literal: &'a str,
    pub line: u32,
}

impl<'a> Token<'a> {
    pub fn new(token_type: Box<TokenType>, lexeme: &'a str, literal: &'a str, line: u32) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        String::from("{self.token_type} {lexeme} {literal}")
    }
}
