use crate::parser::{Expr, Visitor};
use crate::scanner::Token;
use std::collections::HashMap;

/// An interpreter for evaluating an abstract syntax tree.
///
/// The `interpret` method will traverse the AST and evaluate the expression.
/// State information may be stored in the struct.
pub struct Interpreter {
    table: HashMap<String, f64>,
    variable_count: usize,
}
impl Interpreter {
    /// Create a new interpreter.
    pub fn new() -> Self {
        Interpreter {
            table: HashMap::new(),
            variable_count: 0,
        }
    }

    /// Interpret an expression and return a variable name and result.
    ///
    /// This method will visit each node in the AST and evaluate the expression.
    /// The result will be stored in a variable name that can be used in future expressions.
    /// Variables are named based on the order: `$0`, `$1`, `$2`, etc.
    pub fn interpret(&mut self, input: Box<Expr>) -> (String, f64) {
        let result = self.visit(&input);
        let name = format!("${}", self.variable_count);
        self.table.insert(name.clone(), result);
        self.variable_count += 1;
        (name, result)
    }

    /// Interpret an expression without storing the result.
    ///
    /// This method will visit each node in the AST and evaluate the expression.
    /// Variables previously stored in the interpreter may still be used,
    /// but no new variables will be created.
    pub fn quick_interpret(&self, input: Box<Expr>) -> f64 {
        self.visit(&input)
    }
}
impl Visitor<f64> for Interpreter {
    fn visit(&self, expr: &Expr) -> f64 {
        match expr {
            Expr::Number(n) => *n,
            Expr::UnaryOp { op, operand } => {
                let operand = self.visit(operand);
                match op {
                    Token::Minus => -operand,
                    _ => 0.0,
                }
            }
            Expr::BinaryOp { op, left, right } => {
                let left = self.visit(left);
                let right = self.visit(right);
                match op {
                    Token::Plus => left + right,
                    Token::Minus => left - right,
                    Token::Star => left * right,
                    Token::Slash => left / right,
                    _ => 0.0,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret() {
        let input = Box::new(Expr::BinaryOp {
            op: Token::Plus,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_interpret_unary() {
        let input = Box::new(Expr::UnaryOp {
            op: Token::Minus,
            operand: Box::new(Expr::Number(42.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input);
        assert_eq!(result, -42.0);
    }

    #[test]
    fn test_interpret_complex() {
        let input = Box::new(Expr::BinaryOp {
            op: Token::Plus,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::BinaryOp {
                op: Token::Star,
                left: Box::new(Expr::Number(2.0)),
                right: Box::new(Expr::Number(3.0)),
            }),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input);
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_interpret_grouping() {
        let input = Box::new(Expr::BinaryOp {
            op: Token::Slash,
            left: Box::new(Expr::Number(3.0)),
            right: Box::new(Expr::BinaryOp {
                op: Token::Plus,
                left: Box::new(Expr::Number(1.0)),
                right: Box::new(Expr::Number(2.0)),
            }),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input);
        assert_eq!(result, 1.0);
    }
}
