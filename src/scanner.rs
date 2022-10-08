use std::collections::HashMap;

use crate::lox::*;
use crate::token::*;
use crate::tokentype::*;

pub struct Scanner<'a> {
    source: &'a str,
    tokens: Vec<Token<'a>>,
    start: usize,
    current: usize,
    line: u32,
    lox: Lox,
    keywords: HashMap<&'a str, TokenType>,
}

//TODO: fix indexing

// TODO: pass correct literals...how are they different from lexemes?

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
            keywords,
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

    pub fn scan_tokens(mut self) -> Vec<Token<'a>> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens
            .push(Token::new(Box::new(TokenType::Eof), &"", &"", self.line));

        self.tokens
    }
    //TODO: should this be self.current + 1? otherwise it's always less than len()? bc 0 index while len >= 1
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(Box::new(TokenType::LeftParen), &""),
            ')' => self.add_token(Box::new(TokenType::RightParen), &""),
            '{' => self.add_token(Box::new(TokenType::LeftBrace), &""),
            '}' => self.add_token(Box::new(TokenType::RightBrace), &""),
            ',' => self.add_token(Box::new(TokenType::Comma), &""),
            '.' => self.add_token(Box::new(TokenType::Dot), &""),
            '-' => self.add_token(Box::new(TokenType::Minus), &""),
            '+' => self.add_token(Box::new(TokenType::Plus), &""),
            ';' => self.add_token(Box::new(TokenType::Semicolon), &""),
            '*' => self.add_token(Box::new(TokenType::Star), &""),
            '!' => {
                if self.matching('=') {
                    self.add_token(Box::new(TokenType::BangEqual), &"")
                } else {
                    self.add_token(Box::new(TokenType::Bang), &"")
                }
            }
            '=' => {
                if self.matching('=') {
                    self.add_token(Box::new(TokenType::EqualEqual), &""[..])
                } else {
                    self.add_token(Box::new(TokenType::Equal), &""[..])
                }
            }
            '<' => {
                if self.matching('=') {
                    self.add_token(Box::new(TokenType::LessEqual), &""[..])
                } else {
                    self.add_token(Box::new(TokenType::Less), &""[..])
                }
            }
            '>' => {
                if self.matching('=') {
                    self.add_token(Box::new(TokenType::GreaterEqual), &""[..])
                } else {
                    self.add_token(Box::new(TokenType::Greater), &""[..])
                }
            }
            '/' => {
                // if we have double '//' then it's a comment, scan the whole line
                //but don't consume anything.

                //TODO: below there's support for multi line comments. seems to loop infintely

                if self.matching('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.matching('*') {
                    // if self.peek() == '\n' {
                    //     self.line += 1;
                    // }

                    while (self.peek() != '*' && self.peek_next() != '/') && !self.is_at_end() {
                        self.advance();
                    }

                    if self.peek() == '*' && self.peek_next() == '/' {
                        self.advance();
                        self.advance();
                    }
                } else {
                    self.add_token(Box::new(TokenType::Slash), &""[..])
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
                if Self::is_digit(c) {
                    self.number();
                } else if Self::is_alpha(c) {
                    self.identifier();
                } else {
                    self.lox.error(self.line, &"Unexpected character"[..]);
                }
            }
        }
    }

    fn identifier(&mut self) {
        while Self::is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];

        match self.keywords.get(text) {
            Some(token_type) => self.add_token(Box::new(*token_type), &""),
            None => self.add_token(Box::new(TokenType::Identifier), &""),
        }
    }

    fn number(&mut self) {
        while Self::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() != '.' && Self::is_digit(self.peek_next()) {
            self.advance();

            while Self::is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            Box::new(TokenType::Number),
            &self.source[self.start..self.current],
        );
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.as_bytes()[self.current + 1] as char
    }

    fn is_alpha(c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alphanumeric(c: char) -> bool {
        Self::is_alpha(c) || Self::is_digit(c)
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        //TODO: pass token, here I'm passing self.line as token. WRONG
        if self.is_at_end() {
            // self.lox.error(self.advance(), "unterminated string");
        }

        self.advance();
        //Trim surrounding quotes
        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token(Box::new(TokenType::String), value);
    }

    fn matching(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        //trying to index a string at idx current
        if self.source.as_bytes()[self.current] as char != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.as_bytes()[self.current] as char
    }

    fn is_digit(c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        self.source.as_bytes()[self.current - 1] as char
    }

    fn add_token(&mut self, token_type: Box<TokenType>, literal: &'a str) {
        let text = &self.source[self.start as usize..self.current as usize];

        self.tokens
            .push(Token::new(token_type, &text, &literal, self.line));
    }
}
