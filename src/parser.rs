use anyhow::{anyhow, Context, Result};

use crate::expr::*;
use crate::token::*;
use crate::tokentype::*;

#[derive(Clone)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        let mut expr = self.clone().comparison()?;

        if self.matching(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let right = self.comparison()?;
            let operator = self.previous()?;

            expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));

            return Ok(expr);
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr> {
        let expr = self.term()?;

        if self.matching(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let right = self.term()?;
            let operator = self.previous()?;

            let new_expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));

            return Ok(new_expr);
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr> {
        let mut expr = self.factor()?;

        if self.matching(vec![TokenType::Minus, TokenType::Plus]) {
            let right = self.factor()?;
            let operator = self.previous()?;

            expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));

            return Ok(expr);
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr> {
        let mut expr = self.clone().unary()?;

        if self.matching(vec![TokenType::Slash, TokenType::Star]) {
            let right = self.unary()?;
            let operator = self.previous()?;

            expr = Expr::Binary(Box::new(expr), Operator::from(operator), Box::new(right));

            return Ok(expr);
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr> {
        if self.matching(vec![TokenType::Bang, TokenType::Minus]) {
            let right = self.unary()?;
            let operator = self.previous()?;

            let expr = Expr::Unary(Operator::from(operator), Box::new(right));

            return Ok(expr);
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.matching(vec![TokenType::False]) {
            return Ok(Expr::Literal(Box::new(Literal::False)));
        } else if self.matching(vec![TokenType::True]) {
            return Ok(Expr::Literal(Box::new(Literal::True)));
        } else if self.matching(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Box::new(Literal::Nil)));
        } else if self.matching(vec![TokenType::Number, TokenType::String]) {
            let lit = self.previous()?.literal;

            return Ok(Expr::Literal(Box::new(Literal::from(lit))));
        } else if self.matching(vec![TokenType::LeftParen]) {
            let expr = self.expression()?;

            self.consume(&TokenType::RightParen, "Expected ')' after expression");

            return Ok(Expr::Grouping(Box::new(expr)));
        } else {
            return Err(anyhow!("Error from primary(): Expected expression"));
        }
    }

    //TODO: just iterate over token_types and use Rust iter API instead of this fn?
    //this fn couples state and iteration, so i need &mut self, and many of my fns
    //above require multiple mutable references to self
    //so I can use some kind of poitner/reference API in those fns, or (at least for matching()),
    //I can separate statefullness from iteration

    fn matching(&mut self, token_types: Vec<TokenType>) -> bool {
        for token_type in &token_types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token<'a>> {
        if self.check(token_type) {
            return self.advance();
        } else {
            // let peeked = self.peek()?;

            return Err(anyhow!("{message}"));
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        match self.peek() {
            Ok(token) => **token.token_type == *token_type,
            Err(_) => false,
        }
    }

    fn advance(&mut self) -> Result<&Token<'a>> {
        if !self.is_at_end() {
            self.current += 1;
        }

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

    // fn error(&self, token: Token, message: &str) -> ParseError {
    //     //TODO: I just picked a random line number.
    //     let lox = Lox::new();
    //     lox.error(&token, message);

    //     ParseError::new()
    // }

    //TODO: coming back to this fn later, it's not absolutely necessary.
    //it synchronizes error output
    // fn synchronize(&self) {
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

pub struct ParseError;