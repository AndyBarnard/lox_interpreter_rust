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

pub enum Expr<'a> {
    Equality(),
    Comparison(),
    Term(),
    Factor(),
    Binary(Box<Expr<'a>>, Operator, Box<Expr<'a>>),
    Unary(Operator, Box<Expr<'a>>),
    // Literal(Box<Expr>),
    Literal(Box<Literal<'a>>),
    Grouping(Box<Expr<'a>>),
}

pub enum Literal<'a> {
    Number(usize),
    String(&'a str),
    True,
    False,
    Nil,
    Expr(Expr<'a>),
}

impl From<&str> for Literal<'_> {
    fn from(s: &str) -> Self {
        match &s.to_lowercase() {
            "number" => Literal::Number(1),
            "string" => Literal::String("abc"),

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

impl Expr<'_> {
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
