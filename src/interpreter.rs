use crate::parser::{Expr, Visitor};
use crate::scanner::{Token, Word};
use crate::CalcError;
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
    pub fn interpret(&mut self, input: Box<Expr>) -> Result<(String, f64), CalcError> {
        let result = self.visit(&input)?;
        let name = format!("${}", self.variable_count);
        self.table.insert(name.clone(), result);
        self.variable_count += 1;
        Ok((name, result))
    }

    /// Interpret an expression without storing the result.
    ///
    /// This method will visit each node in the AST and evaluate the expression.
    /// Variables previously stored in the interpreter may still be used,
    /// but no new variables will be created.
    pub fn quick_interpret(&self, input: Box<Expr>) -> Result<f64, CalcError> {
        self.visit(&input)
    }
}
impl Visitor<f64> for Interpreter {
    fn visit(&self, expr: &Expr) -> Result<f64, CalcError> {
        match expr {
            Expr::Number(n) => Ok(*n),
            Expr::UnaryOp { op, operand } => {
                let operand = self.visit(operand)?;
                match op {
                    Token::Minus => Ok(-operand),
                    Token::Keyword(Word::Sqrt) => Ok(operand.sqrt()),
                    _ => Ok(0.0),
                }
            }
            Expr::BinaryOp { op, left, right } => {
                let left = self.visit(left)?;
                let right = self.visit(right)?;
                match op {
                    Token::Plus => Ok(left + right),
                    Token::Minus => Ok(left - right),
                    Token::Star => Ok(left * right),
                    Token::Slash => Ok(left / right),
                    _ => Ok(0.0),
                }
            }
            Expr::Variable(name) => match self.table.get(name) {
                Some(value) => Ok(*value),
                None => Err(CalcError::new("Variable not found", None)),
            },
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
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_interpret_unary() {
        let input = Box::new(Expr::UnaryOp {
            op: Token::Minus,
            operand: Box::new(Expr::Number(42.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input).unwrap();
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
        let (_, result) = interpreter.interpret(input).unwrap();
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
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_interpret_sqrt() {
        let input = Box::new(Expr::UnaryOp {
            op: Token::Keyword(Word::Sqrt),
            operand: Box::new(Expr::Number(9.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 3.0);
    }
}
