use crate::calc_error::CalcError;
use crate::parser::{Expr, Visitor};
use crate::scanner::Token;

/// An interpreter for evaluating an abstract syntax tree.
///
/// The `interpret` method will traverse the AST and evaluate the expression.
/// State information may be stored in the struct.
pub struct Interpreter {}
impl Interpreter {
    /// Create a new interpreter.
    pub fn new() -> Self {
        Interpreter {}
    }

    /// Interpret an expression.
    ///
    /// This method will visit each node in the AST and evaluate the expression.
    pub fn interpret(&self, input: Box<Expr>) -> Result<f64, CalcError> {
        Ok(self.visit(&input))
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
        let interpreter = Interpreter {};
        let result = interpreter.interpret(input).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_interpret_unary() {
        let input = Box::new(Expr::UnaryOp {
            op: Token::Minus,
            operand: Box::new(Expr::Number(42.0)),
        });
        let interpreter = Interpreter {};
        let result = interpreter.interpret(input).unwrap();
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
        let interpreter = Interpreter {};
        let result = interpreter.interpret(input).unwrap();
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
        let interpreter = Interpreter {};
        let result = interpreter.interpret(input).unwrap();
        assert_eq!(result, 1.0);
    }
}
