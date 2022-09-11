use crate::token::*;
use crate::tokentype::*;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token>,
    start: u32,
    current: u32,
    line: u32
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self { //vec slice for tokens
        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::Eof, String::from(""), String::from(""), self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, String::from("")),
            ')' => self.add_token(TokenType::RightParen, String::from("")),
            '{' => self.add_token(TokenType::LeftBrace, String::from("")),
            '}' => self.add_token(TokenType::RightBrace, String::from("")),
            ',' => self.add_token(TokenType::Comma, String::from("")),
            '.' => self.add_token(TokenType::Dot, String::from("")),
            '-' => self.add_token(TokenType::Minus, String::from("")),
            '+' => self.add_token(TokenType::Plus, String::from("")),
            ';' => self.add_token(TokenType::Semicolon, String::from("")),
            '*' => self.add_token(TokenType::Star, String::from("")),
        }
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.char_at(current - 1)
    }

    fn add_token(&self, token_type: TokenType, literal: String) {
        let text = self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, text, literal, line));
    }
}

// let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
