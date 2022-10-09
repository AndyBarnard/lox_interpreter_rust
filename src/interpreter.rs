use crate::Expr::*;

struct Interpreter {}

//TODO: using Object as a return type placeholder here
//this is the visitor pattern to get around the expression problem in OOP
//I believe this is best alleviated w/ traits in Rust so gotta change later

impl Interpreter {
    pub fn literal_expr(expr: Expr::Literal) -> Expr::Literal {
        // expr.value
        expr
    }

    pub fn unary_expr(expr: Expr::Unary) -> Expr::Unary {

    }

    //TODO: trying pattern matching in fn signature here
    pub fn grouping_expr(expr(expression): Expr::Grouping) -> Expr::Grouping {
        evaluate(expression)
    }

    fn evaluate<T>(&self, expr: Expr) -> T {
        expr.accept(self)
    }
}
