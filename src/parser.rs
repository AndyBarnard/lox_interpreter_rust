use anyhow::{anyhow, Context, Result};

use crate::expr::*;
use crate::lox::*;
use crate::token::*;
use crate::tokentype::*;

pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&self) -> Result<Expr> {
        self.expression()
    }

    fn expression(&self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&self) -> Result<Expr> {
        let expr = self.comparison()?;

        while self.matching(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let new_expr = self.previous().and_then(|operator| {
                let right = self.comparison()?;
                expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));
                return Ok(expr);
            })?;
        }

        Ok(expr)
    }

    fn comparison(&self) -> Result<Expr> {
        let expr = self.term()?;

        while self.matching(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let new_expr = self.previous().and_then(|operator| {
                let right = self.term()?;
                expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));
                return Ok(expr);
            })?;
        }

        Ok(expr)
    }

    fn term(&self) -> Result<Expr> {
        let expr = self.factor()?;

        while self.matching(vec![TokenType::Minus, TokenType::Plus]) {
            let new_expr = self.previous().and_then(|operator| {
                let right = self.factor()?;
                expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));
                return Ok(expr);
            });
        }

        Ok(expr)
    }

    fn factor(&self) -> Result<Expr> {
        let expr = self.unary()?;

        while self.matching(vec![TokenType::Slash, TokenType::Star]) {
            let new_expr = self.previous().and_then(|operator| {
                let right = self.unary()?;
                expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));
                return Ok(expr);
            })?;
        }

        Ok(expr)
    }

    fn unary(&self) -> Result<Expr> {
        if self.matching(vec![TokenType::Bang, TokenType::Minus]) {
            let new_expr = self.previous().and_then(|operator| {
                let right = self.unary()?;
                let expr = Expr::Unary(Operator::from(operator), Box::new(right));
                return Ok(expr);
            })?;
        }

        self.primary()
    }

    fn primary(&self) -> Result<Expr> {
        if self.matching(vec![TokenType::False]) {
            return Ok(Expr::Literal(Box::new(Literal::False)));
        }
        if self.matching(vec![TokenType::True]) {
            return Ok(Expr::Literal(Box::new(Literal::True)));
        }
        if self.matching(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Box::new(Literal::Nil)));
        }
        if self.matching(vec![TokenType::Number, TokenType::String]) {
            // match self.previous() {
            //     //TODO: here i'm passing token.literal which is a string but I need to convert to Literal type
            //     Some(token) => return Expr::Literal(Box::new(Literal::from(token.literal))),
            //     None => eprint!("Error calling self.previous() in primary()"),
            // }
            let lit = self.previous()?.literal;
            return Ok(Expr::Literal(Box::new(Literal::from(lit))));
        }
        if self.matching(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;
            //TODO: so consume must return Option<T> based on my implementation
            // expect(
            //     self.consume(&TokenType::RightParen),
            //     "Expected ')' after expression",
            // );
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        // self.error(self.peek(), "Expect expression.");
    }

    fn matching(&self, token_types: Vec<TokenType>) -> bool {
        for token_type in &token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&self, token_type: &TokenType, message: String) -> Result<&Token<'a>> {
        if self.check(token_type) {
            return Ok(self.advance()?);
        } else {
            return Err(anyhow!("Error in consume()"));
        }

        //return anyhow error
        //TODO: see how to return errors with anyhow
        //his code:
        //throw error(peek(), message);
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.peek() {
            Ok(token) => **token.token_type == *token_type,
            Err(_) => false,
        }
        // self.peek().unwrap_or_else(|| return false);
        // **self.peek()?.token_type == *token_type
    }

    fn advance(&self) -> Result<&Token<'a>> {
        if !self.is_at_end() {
            self.current += 1;
        }

        // match self.previous() {
        //     Some(token) => token,
        //     None => eprint!("Error calling self.previous() in advance()"),
        // }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        match self.peek() {
            Ok(token) => **token.token_type == TokenType::Eof,
            Err(_) => false,
        }
    }

    fn peek(&self) -> Result<&Token<'a>> {
        self.tokens
            .get(self.current)
            .context("Error in call to peek()")
    }

    fn previous(&self) -> Result<&Token<'a>> {
        self.tokens
            .get(self.current - 1)
            .context("Error in call to previous()")
    }

    fn error(&self, token: Token, message: &str) -> ParseError {
        //TODO: I just picked a random line number.
        let lox = Lox::new();
        lox.error(&token, message);

        ParseError::new()
    }

    //TODO: coming back to this fn later, it's not absolutely necessary.
    //it synchronizes error output
    // fn synchronize(&self) -> Result<&Token<'a>> {
    //     let token = self.advance()?;

    //     while !self.is_at_end() {
    //         // match self.previous() {
    //         //     Some(token) => {
    //         //         if *token.token_type == TokenType::Semicolon {
    //         //             return;
    //         //         }
    //         //     }
    //         //     None => (),
    //         // }
    //         if *self.previous()?.token_type == TokenType::Semicolon {
    //             return;
    //         }

    //         // match self.peek() {
    //         //     Some(token) => match *token.token_type {
    //         //         TokenType::Return => return,
    //         //         _ => (),
    //         //     },
    //         //     None => eprint!("Error calling self.peek() in synchronize()"),
    //         // }
    //         if *self.peek()?.token_type == TokenType::Return {
    //             return;
    //         }

    //         self.advance();
    //     }
    // }
}
