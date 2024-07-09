//! Module for parsing a vector of tokens into an abstract syntax tree.

use crate::calc_error::CalcError;
use crate::scanner::Token;

enum Expr {
    Number(f64),
    UnaryOp {
        op: fn(f64) -> f64,
        operand: Box<Expr>,
    },
    BinaryOp {
        op: fn(f64, f64) -> f64,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

trait Visitor<T> {
    fn visit(&self, expr: &Expr) -> T;
}

pub struct Parser {}
impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Box<Expr>, CalcError> {
        Ok(Box::new(Expr::Number(0.0)))
    }
}
