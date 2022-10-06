use std::ops::Deref;

use crate::token::Token;

#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    //category A
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    //category B
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    //category C
    Identifier,
    String,
    Number,
    //category D
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    //category E
    Eof,
}

impl Deref for TokenType {
    type Target = TokenType;

    fn deref<'a>(&'a self) -> &'a TokenType {
        self
    }
}