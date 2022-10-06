use anyhow::Result;

use crate::lox::*;
use crate::token::*;
use crate::tokentype::*;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn expression() -> Expr {
        equality()
    }

    /*
    TODO: find out what data structure to represent the Expr that I return.
    below for equality(), it's the following rule:

    expr ((!= | ==) expr)*
    */

    fn equality() -> Expr {
        let expr = Expr::new();

        while matches(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = previous();
            let right = comparison();
            expr = Binary::new(expr, operator, right); //TODO: i'm changing type of expr
        }

        expr
    }

    fn comparison() -> Expr {
        let expr = term();

        while matches(
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ) {
            let operator = previous();
            let right = term();
            expr = Binary::new(expr, operator, right);
        }

        expr
    }

    fn term() -> Expr {
        let expr = factor();

        while matches(TokenType::Minus, TokenType::Plus) {
            let operator = previous();
            let right = factor();
            expr = Binary::new(expr, operator, right);
        }

        expr
    }

    fn factor() -> Expr {
        let expr = unary();

        while matches(TokenType::Slash, TokenType::Star) {
            let operator = previous();
            let right = unary();
            expr = Binary::new(expr, operator, right);
        }
    }

    fn unary() -> Expr {
        if matches(TokenType::Bang, TokenType::Minus) {
            let operator = previous();
            let right = unary();
            return Unary::new(operator, right);
        }

        primary()
    }

    fn primary() -> Expr {
        if mathes(TokenType::False) {
            return Literal::new(false);
        }
        if matches(TokenType::True) {
            return Literal::new(true);
        }
        if matches(TokenType::Nil) {
            return Literal::new("null"); //here he passes null...use Option<T> and pass None?
        }
        if matches(TokenType::Number, TokenType::String) {
            return Literal::new(previous().literal);
        }
        if matches(TokenType::LeftParen) {
            let expr = expression();
            //TODO: so consume must return Option<T> based on my implementation
            expect(
                consume(TokenType::RightParen),
                "Expected ')' after expression",
            );
            return Grouping::new(expr);
        }
    }

    fn matches(token_types: Vec<TokenType>) -> bool {
        for token_type in token_types {
            if check(token_type) {
                advance();
                return true;
            }
        }

        false
    }

    fn consume(token_type: TokenType, message: String) -> Result<Token> {
        if check(token_type) {
            return Some(advance());
        }

        //return anyhow error 
        //TODO: see how to return errors with anyhow
        //his code:
        //throw error(peek(), message);
    }

    fn check(token_type: TokenType) -> bool {
        if is_at_end() {
            return false;
        }

        peek().token_type == token_type
    }

    fn advance() -> Token {
        if !is_at_end() {
            current += 1;
        }

        previous()
    }

    fn is_at_end() -> bool {
        peek().token_type == TokenType::Eof
    }

    fn peek() -> Token {
        tokens.get(current)
    }

    fn previous() -> Token {
        tokens.get(current - 1)
    }

    fn error(token: Token, message: &str) -> ParseError {
        //TODO: I just picked a random line number.
        //his code:
        //Lox.error(token, message);
        Lox::error(token, 0, message);

        ParseError::new()
    }
}

// struct Binary;
// struct Expr;

// impl Binary {
//     fn new() -> Expr {
//         Expr
//     }
// }
