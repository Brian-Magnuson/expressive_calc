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
    /// The last result is also stored in the variable `$ans`.
    pub fn interpret(&mut self, input: Box<Expr>) -> Result<(String, f64), CalcError> {
        let result = self.visit(&input)?;
        let name = format!("${}", self.variable_count);
        self.table.insert(name.clone(), result);
        self.table.insert("$ans".to_string(), result);
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

    /// Reset the interpreter, clearing all stored variables.
    ///
    /// This method will clear all stored variables and reset the variable count.
    pub fn reset(&mut self) {
        self.table.clear();
        self.variable_count = 0;
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
                    Token::Keyword(Word::Cbrt) => Ok(operand.cbrt()),
                    Token::Keyword(Word::Exp) => Ok(operand.exp()),
                    Token::Keyword(Word::Log2) => Ok(operand.log2()),
                    Token::Keyword(Word::Log10) => Ok(operand.log10()),
                    Token::Keyword(Word::Ln) => Ok(operand.ln()),
                    Token::Keyword(Word::Sin) => Ok(operand.sin()),
                    Token::Keyword(Word::Cos) => Ok(operand.cos()),
                    Token::Keyword(Word::Tan) => Ok(operand.tan()),
                    Token::Keyword(Word::Asin) => Ok(operand.asin()),
                    Token::Keyword(Word::Acos) => Ok(operand.acos()),
                    Token::Keyword(Word::Atan) => Ok(operand.atan()),
                    Token::Keyword(Word::Sinh) => Ok(operand.sinh()),
                    Token::Keyword(Word::Cosh) => Ok(operand.cosh()),
                    Token::Keyword(Word::Tanh) => Ok(operand.tanh()),
                    Token::Keyword(Word::Asinh) => Ok(operand.asinh()),
                    Token::Keyword(Word::Acosh) => Ok(operand.acosh()),
                    Token::Keyword(Word::Atanh) => Ok(operand.atanh()),
                    Token::Keyword(Word::Rad) => Ok(operand.to_radians()),
                    Token::Keyword(Word::Deg) => Ok(operand.to_degrees()),
                    Token::Keyword(Word::Abs) => Ok(operand.abs()),
                    Token::Keyword(Word::Floor) => Ok(operand.floor()),
                    Token::Keyword(Word::Ceil) => Ok(operand.ceil()),
                    Token::Keyword(Word::Trunc) => Ok(operand.trunc()),
                    Token::Keyword(Word::Round) => Ok(operand.round()),
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
                    Token::Caret | Token::Keyword(Word::Pow) => Ok(left.powf(right)),
                    Token::Percent | Token::Keyword(Word::Mod) => Ok(left % right),
                    Token::Keyword(Word::Log) => Ok(left.log(right)),
                    Token::Keyword(Word::Hypot) => Ok(left.hypot(right)),
                    Token::Keyword(Word::Atan2) => Ok(left.atan2(right)),
                    Token::Keyword(Word::Max) => Ok(left.max(right)),
                    Token::Keyword(Word::Min) => Ok(left.min(right)),
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

// MARK: Tests
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

    #[test]
    fn test_interpret_exp() {
        let input = Box::new(Expr::UnaryOp {
            op: Token::Keyword(Word::Exp),
            operand: Box::new(Expr::Number(1.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 2.718281828459045);
    }

    #[test]
    fn test_interpret_ln() {
        let input = Box::new(Expr::UnaryOp {
            op: Token::Keyword(Word::Ln),
            operand: Box::new(Expr::Number(2.718281828459045)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_interpret_pow() {
        let input = Box::new(Expr::BinaryOp {
            op: Token::Keyword(Word::Pow),
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 8.0);
    }

    #[test]
    fn test_interpret_log() {
        let input = Box::new(Expr::BinaryOp {
            op: Token::Keyword(Word::Log),
            left: Box::new(Expr::Number(8.0)),
            right: Box::new(Expr::Number(2.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_interpret_mod() {
        let input = Box::new(Expr::BinaryOp {
            op: Token::Keyword(Word::Mod),
            left: Box::new(Expr::Number(8.0)),
            right: Box::new(Expr::Number(3.0)),
        });
        let mut interpreter = Interpreter::new();
        let (_, result) = interpreter.interpret(input).unwrap();
        assert_eq!(result, 2.0);
    }
}
