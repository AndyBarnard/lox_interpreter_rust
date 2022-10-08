use std::convert::From;

use crate::token::*;
use crate::tokentype::*;

//TODO: have Expr fields as Option? in crate::parser with fns like equality() I call
// Expr::new() with no args then in an if-statement I fill those args
//or I can just use 2 different variable names like expr_left and expr_right? I'll try that

//TODO: make Expr left recursive by always requiring the `left` field but make the other 2 fields Options?
//or since `operator` and `right` are inside () in his AST, have them as a tuple?

//TODO: or have Expr as an enum with all the different types of expressions?
//that allows for polymorphism but how does that affect the recursive structure of our parser
//w.r.t. recursive descent?
//I think the only alternative to that is having all the fields on Expr as Options which might be worse
// pub struct Expr {
//     left: Box<Expr>,
//     operator: Token,
//     right: Box<Expr>,
// }

//TODO: some of the fields on Expr may be enums (Literal, etc.), some not

pub enum Expr {
    Equality(),
    Comparison(),
    Term(),
    Factor(),
    Binary(Box<Expr>, Operator, Box<Expr>),
    Unary(Operator, Box<Expr>),
    // Literal(Box<Expr>),
    Literal(Box<Literal>),
    Grouping(Box<Expr>),
}
//TODO: should Literal be an enum? then what about the actual literal value?

pub enum Literal {
    Number(usize),
    String(String),
    True,
    False,
    Nil,
    Expr(Expr),
}

impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        match &s.to_lowercase() {
            "number" => Literal::Number,
            "string" => Literal::String,

        }
    }
}

pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Bang,
    Equal,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

impl From<&Token<'_>> for Operator {
    fn from(token: &Token) -> Self {
        match *token.token_type {
            TokenType::Plus => Operator::Plus,
            TokenType::Minus => Operator::Minus,
            TokenType::Star => Operator::Times,
            TokenType::Slash => Operator::Divide,
            TokenType::Bang => Operator::Bang,
            TokenType::Equal => Operator::Equal,
            TokenType::Greater => Operator::Greater,
            TokenType::GreaterEqual => Operator::GreaterEqual,
            TokenType::Less => Operator::Less,
            TokenType::LessEqual => Operator::LessEqual,
            _ => eprintln!("Error converting from &Token to Operator"),
        }
    }
}

impl Expr {
    // pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
    //     Self {
    //         left: Box::new(left),
    //         operator,
    //         right: Box::new(right),
    //     }
    // }
}

// trait Binary {
//     fn binary(&self) -> Expr;
// }

// impl Binary for Expr {
//     fn binary(&self, left: Expr, operator: Token, right: Expr) -> Self {
//         //TODO: I think in all these "subclasses" of Expr I can just call Expr::new() with diff args
//         Expr::new(left, operator, right)
//     }
// }

// trait Literal {
//     fn literal(&self) -> Expr;
// }

// impl Literal for Expr {
//     fn literal(&self) -> Self {
//         Expr::new()
//     }
// }

// trait Unary {
//     fn unary(&self, operator: Token, right: Expr) -> Expr;
// }

// impl Unary for Expr {

// }
