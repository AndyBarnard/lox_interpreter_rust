use std::collections::HashMap;

use crate::lox::*;
use crate::token::*;
use crate::tokentype::*;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: u32,
    current: u32,
    line: u32,
    lox: Lox,
    keywords: HashMap<&'a str, TokenType> 
}

//TODO: fix indexing

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut keywords = HashMap::new();
        Self::build_keywords(&mut keywords);

        Self {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            lox: Lox::new(),
            keywords
        }
    }

    fn build_keywords(keywords: &mut HashMap<&str, TokenType>) {
        keywords.insert(&"and", TokenType::And);
        keywords.insert(&"class", TokenType::Class);
        keywords.insert(&"else", TokenType::Else);
        keywords.insert(&"false", TokenType::False);
        keywords.insert(&"for", TokenType::For);
        keywords.insert(&"fun", TokenType::Fun);
        keywords.insert(&"if", TokenType::If);
        keywords.insert(&"nil", TokenType::Nil);
        keywords.insert(&"or", TokenType::Or);
        keywords.insert(&"print", TokenType::Print);
        keywords.insert(&"return", TokenType::Return);
        keywords.insert(&"super", TokenType::Super);
        keywords.insert(&"this", TokenType::This);
        keywords.insert(&"true", TokenType::True);
        keywords.insert(&"var", TokenType::Var);
        keywords.insert(&"while", TokenType::While);
    }

    pub fn scan_tokens(&self) -> Vec<Token> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(TokenType::Eof, &"", &"", self.line));
        self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() as u32
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen, &""),
            ')' => self.add_token(TokenType::RightParen, &""),
            '{' => self.add_token(TokenType::LeftBrace, &""),
            '}' => self.add_token(TokenType::RightBrace, &""),
            ',' => self.add_token(TokenType::Comma, &""),
            '.' => self.add_token(TokenType::Dot, &""),
            '-' => self.add_token(TokenType::Minus, &""),
            '+' => self.add_token(TokenType::Plus, &""),
            ';' => self.add_token(TokenType::Semicolon, &""),
            '*' => self.add_token(TokenType::Star, &""),
            '!' => {
                if self.matching('=') {
                    self.add_token(TokenType::BangEqual, &"")
                } else {
                    self.add_token(TokenType::Bang, &"")
                }
            }
            '=' => {
                if self.matching('=') {
                    self.add_token(TokenType::EqualEqual, &""[..])
                } else {
                    self.add_token(TokenType::Equal, &""[..])
                }
            }
            '<' => {
                if self.matching('=') {
                    self.add_token(TokenType::LessEqual, &""[..])
                } else {
                    self.add_token(TokenType::Less, &""[..])
                }
            }
            '>' => {
                if self.matching('=') {
                    self.add_token(TokenType::GreaterEqual, &""[..])
                } else {
                    self.add_token(TokenType::Greater, &""[..])
                }
            }
            '/' => {
                // if we have double '//' then it's a comment, scan the whole line
                //but don't consume anything.
                if self.matching('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, &""[..])
                }
            }
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line += 1,
            '"' => {
                self.string();
            }

            _ => {
                if Self::is_digit(_) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    self.lox.error(self.line, &"Unexpected character"[..]);
                }
            }
        }
    }

    fn identifier(&self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(&text);

        if let None = token_type {
            let assigned_token_type = TokenType::Identifier;
            self.add_token(assigned_token_type, &"");
        }

        self.add_token(TokenType::Identifier, &"");
    }

    fn number(&self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() != '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(TokenType::Number, &self.source[self.start..self.current]);
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() as u32 {
            '\0'
        }
        &self.source[self.current + 1]
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn string(&self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.lox.error(self.line, "unterminated string");
        }

        self.advance();
        //Trim surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(TokenType::String, value);
    }

    fn matching(&self, expected: char) -> bool {
        if self.is_at_end() {
            false
        }
        //trying to index a string at idx current
        if self.source.as_bytes()[self.current as u8] as char != expected {
            false
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        }
        self.source.as_bytes()[self.current] as char
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source
            .chars()
            .nth((self.current - 1) as usize)
            .unwrap()
    }

    fn add_token(&self, token_type: TokenType, literal: &str) {
        let text = self.source[self.start as usize..self.current as usize];

        self.tokens
            .push(Token::new(token_type, &text, &literal, self.line));
    }
}

// let s = String::from("hello world");

//     let hello = &s[0..5];
//     let world = &s[6..11];
