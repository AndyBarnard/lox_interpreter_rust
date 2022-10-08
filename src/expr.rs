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

pub enum Literal {
    Number(usize),
    String(String),
    True,
    False,
    Nil,
    Expr(Expr),
}

//TODO: this parses everything as a Number, obviously wrong, just getting it to build now
//  bc I'm still not sure how to structure this with literals
impl From<&str> for Literal {
    fn from(s: &str) -> Self {
        match s.to_lowercase() {
            // "number" => Literal::Number(1),
            // "string" => Literal::String("abc"),
            _ => Literal::Number(1),
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
    Invalid, //TODO: this is a placeholder for invalid conversion in the from() fn. will remove later
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
            _ => Operator::Invalid,
        }
    }
}

// impl Expr<'_> {
    // pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
    //     Self {
    //         left: Box::new(left),
    //         operator,
    //         right: Box::new(right),
    //     }
    // }
// }

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
