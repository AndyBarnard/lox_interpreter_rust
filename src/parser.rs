use anyhow::Result;

use crate::lox::*;
use crate::token::*;
use crate::scanner::*;
use crate::tokentype::*;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

//TODO: define Expr
//figure out how to get equivalent to his inheritance/polymorphism with sub-Expr expressions (Grouping, etc)

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&self) -> Expr {
        match self.expression() {
            Ok(res) => res,
            Err(e) => e,
        }
    }

    fn expression(&self) -> Result<Expr> {
        self.equality()
    }

    /*
    TODO: find out what data structure to represent the Expr that I return.
    below for equality(), it's the following rule:

    expr ((!= | ==) expr)*
    */

    fn equality(&self) -> Expr {
        let expr = Expr::new();

        while self.matching(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Binary::new(expr, operator, right); //TODO: i'm changing type of expr
        }

        expr
    }

    fn comparison(&self) -> Expr {
        let expr = self.term();

        while self.matching(
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ) {
            let operator = self.previous();
            let right = self.term();
            expr = Binary::new(expr, operator, right);
        }

        expr
    }

    fn term(&self) -> Expr {
        let expr = self.factor();

        while self.matching(TokenType::Minus, TokenType::Plus) {
            let operator = self.previous();
            let right = self.factor();
            expr = Binary::new(expr, operator, right);
        }

        expr
    }

    fn factor(&self) -> Expr {
        let expr = self.unary();

        while self.matching(TokenType::Slash, TokenType::Star) {
            let operator = self.previous();
            let right = self.unary();
            expr = Binary::new(expr, operator, right);
        }
    }

    fn unary(&self) -> Expr {
        if self.matching(TokenType::Bang, TokenType::Minus) {
            let operator = self.previous();
            let right = self.unary();
            return Unary::new(operator, right);
        }

        self.primary()
    }

    //TODO: should return result?
    fn primary(&self) -> Expr {
        if self.matching(TokenType::False) {
            return Literal::new(false);
        }
        if self.matching(TokenType::True) {
            return Literal::new(true);
        }
        if self.matching(TokenType::Nil) {
            return Literal::new("null"); //here he passes null...use Option<T> and pass None?
        }
        if self.matching(TokenType::Number, TokenType::String) {
            return Literal::new(self.previous().literal);
        }
        if self.matching(TokenType::LeftParen) {
            let expr = self.expression();
            //TODO: so consume must return Option<T> based on my implementation
            expect(
                self.consume(TokenType::RightParen),
                "Expected ')' after expression",
            );
            return Grouping::new(expr);
        }

        self.error(self.peek(), "Expect expression.");
    }

    fn matching(&self, token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&self, token_type: TokenType, message: String) -> Result<Token<'a>> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        //return anyhow error
        //TODO: see how to return errors with anyhow
        //his code:
        //throw error(peek(), message);
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    fn advance(&self) -> Token<'a> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> Token<'a> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Token<'a> {
        self.tokens.get(self.current - 1)
    }

    fn error(&self, token: Token, message: &str) -> ParseError {
        //TODO: I just picked a random line number.
        //his code:
        //Lox.error(token, message);
        Lox::error(token, 0, message);

        ParseError::new()
    }

    fn synchronize(&self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            match self.peek().token_type {
                TokenType::Return => return,
                // TokenType::Class => (),
                // TokenType::Fun => (),
                // TokenType::Var => (),
                // TokenType::For => (),
                // TokenType::If => (),
                // TokenType::While => (),
                // TokenType::Print => (),
                _ => (),
            }

            self.advance();
        }
    }
}

// struct Binary;
// #[derive()]
struct Expr;

// impl Binary {
//     fn new() -> Expr {
//         Expr
//     }
// }
