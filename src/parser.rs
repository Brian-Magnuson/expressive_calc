//! Module for parsing a vector of tokens into an abstract syntax tree.

use crate::scanner::Token;
use crate::{calc_error::CalcError, scanner::Word};

use std::{iter::Peekable, slice::Iter};

const PHI: f64 = 1.618033988749894848204586834365638118_f64;

/// An expression in the form of an abstract syntax tree.
#[derive(Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Variable(String),
    UnaryOp {
        op: Token,
        operand: Box<Expr>,
    },
    BinaryOp {
        op: Token,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

/// A visitor trait for traversing an abstract syntax tree.
///
/// Although the trait is named `Visitor`, it is not a true visitor pattern.
/// Because expressions are enums, implementing the `visit` method means
/// handling each variant of the enum in a single location.
pub trait Visitor<T> {
    /// Visit an expression.
    ///
    /// When traversing an AST, this method will be called for each node.
    /// Because `Expr` is an enum, the implementor will be responsible for
    /// handling each variant of the enum.
    fn visit(&self, expr: &Expr) -> Result<T, CalcError>;
}

/// A parser used for generating an abstract syntax tree from a vector of tokens.
pub struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    /// Create a new parser with a slice of tokens.
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            iter: tokens.iter().peekable(),
        }
    }

    /// Parse the tokens into an abstract syntax tree.
    ///
    /// This function will call the first part of the recursive descent parser.
    /// If the iterator is not empty after parsing, an error is returned, even if
    /// the preceding tokens were valid.
    pub fn parse(&mut self) -> Result<Box<Expr>, CalcError> {
        let result = self.expr();
        // Ensure that the iterator is empty after parsing
        match self.iter.peek() {
            Some(_) => Err(CalcError::new("Unexpected token", None)),
            None => result,
        }
    }

    /// Check if the next token is the expected token and consume it if it is.
    ///
    /// If the next token is the expected token, consume it and return true.
    /// Otherwise, return false, leaving the iterator unchanged.
    fn optional(&mut self, token: Token) -> bool {
        match self.iter.peek() {
            Some(t) if *t == &token => {
                self.iter.next();
                true
            }
            _ => false,
        }
    }

    /// Require a token to be the next token in the iterator.
    ///
    /// Calls `next` on the iterator and compares the result to the expected token.
    /// If the token is not the next token, an error is returned.
    fn require(&mut self, token: Token, msg: &str) -> Result<(), CalcError> {
        match self.iter.next() {
            Some(t) if t == &token => Ok(()),
            _ => Err(CalcError::new(msg, None)),
        }
    }

    /// Parse an expression.
    ///
    /// This function will call the first part of the recursive descent parser.
    fn expr(&mut self) -> Result<Box<Expr>, CalcError> {
        self.term()
    }

    /// Parse a term binary expression.
    ///
    /// Term operations include addition and subtraction.
    fn term(&mut self) -> Result<Box<Expr>, CalcError> {
        let expr = self.factor()?;
        loop {
            match self.iter.peek() {
                Some(Token::Plus) => {
                    self.iter.next();
                    let right = self.factor()?;
                    return Ok(Box::new(Expr::BinaryOp {
                        op: Token::Plus,
                        left: expr,
                        right,
                    }));
                }
                Some(Token::Minus) => {
                    self.iter.next();
                    let right = self.factor()?;
                    return Ok(Box::new(Expr::BinaryOp {
                        op: Token::Minus,
                        left: expr,
                        right,
                    }));
                }
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    /// Parse a factor binary expression.
    ///
    /// Factor operations include multiplication and division.
    fn factor(&mut self) -> Result<Box<Expr>, CalcError> {
        let expr = self.unary()?;
        loop {
            match self.iter.peek() {
                Some(Token::Star) => {
                    self.iter.next();
                    let right = self.unary()?;
                    return Ok(Box::new(Expr::BinaryOp {
                        op: Token::Star,
                        left: expr,
                        right,
                    }));
                }
                Some(Token::Slash) => {
                    self.iter.next();
                    let right = self.unary()?;
                    return Ok(Box::new(Expr::BinaryOp {
                        op: Token::Slash,
                        left: expr,
                        right,
                    }));
                }
                _ => {
                    return Ok(expr);
                }
            }
        }
    }

    /// Parse a unary expression.
    ///
    /// A unary expression is either a primary expression or a unary operator followed by a primary expression.
    fn unary(&mut self) -> Result<Box<Expr>, CalcError> {
        match self.iter.peek() {
            Some(Token::Minus) => {
                self.iter.next();
                let operand = self.primary()?;
                Ok(Box::new(Expr::UnaryOp {
                    op: Token::Minus,
                    operand,
                }))
            }
            _ => self.primary(),
        }
    }

    /// Parse a primary expression.
    ///
    /// A primary expression is either a number, variable, or an expression enclosed in parentheses.
    fn primary(&mut self) -> Result<Box<Expr>, CalcError> {
        match self.iter.next() {
            Some(Token::Number(n)) => Ok(Box::new(Expr::Number(*n))),
            Some(Token::Variable(s)) => Ok(Box::new(Expr::Variable(s.clone()))),
            Some(Token::Keyword(w)) => self.call(w),
            Some(Token::LParen) => {
                let expr = self.expr()?;
                match self.iter.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(CalcError::new("Expected closing parenthesis", None)),
                }
            }
            _ => Err(CalcError::new("Not a valid expression", None)),
        }
    }

    fn call(&mut self, w: &Word) -> Result<Box<Expr>, CalcError> {
        match w {
            Word::Inf => Ok(Box::new(Expr::Number(f64::INFINITY))),
            Word::Pi => Ok(Box::new(Expr::Number(std::f64::consts::PI))),
            Word::Tau => Ok(Box::new(Expr::Number(std::f64::consts::TAU))),
            Word::E => Ok(Box::new(Expr::Number(std::f64::consts::E))),
            Word::Phi => Ok(Box::new(Expr::Number(PHI))),
            Word::Sqrt
            | Word::Cbrt
            | Word::Exp
            | Word::Log2
            | Word::Log10
            | Word::Ln
            | Word::Sin
            | Word::Cos
            | Word::Tan
            | Word::Asin
            | Word::Acos
            | Word::Atan
            | Word::Sinh
            | Word::Cosh
            | Word::Tanh
            | Word::Asinh
            | Word::Acosh
            | Word::Atanh
            | Word::Rad
            | Word::Deg
            | Word::Abs
            | Word::Floor
            | Word::Ceil
            | Word::Trunc
            | Word::Round => {
                self.require(Token::LParen, "Expected opening parenthesis")?;
                let expr = self.expr()?;
                self.optional(Token::Comma);
                self.require(Token::RParen, "Expected closing parenthesis")?;
                Ok(Box::new(Expr::UnaryOp {
                    op: Token::Keyword(w.clone()),
                    operand: expr,
                }))
            }
            Word::Pow
            | Word::Log
            | Word::Hypot
            | Word::Atan2
            | Word::Mod
            | Word::Max
            | Word::Min => {
                self.require(Token::LParen, "Expected opening parenthesis")?;
                let left = self.expr()?;
                self.require(Token::Comma, "Expected comma")?;
                let right = self.expr()?;
                self.optional(Token::Comma);
                self.require(Token::RParen, "Expected closing parenthesis")?;
                Ok(Box::new(Expr::BinaryOp {
                    op: Token::Keyword(w.clone()),
                    left,
                    right,
                }))
            }
        }
    }
}

// MARK: Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        let input = vec![];
        let mut parser = Parser::new(&input);
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_parse_number() {
        let input = vec![Token::Number(42.0)];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::Number(42.0));
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_unary_op() {
        let input = vec![Token::Minus, Token::Number(42.0)];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::UnaryOp {
            op: Token::Minus,
            operand: Box::new(Expr::Number(42.0)),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_parse_addition() {
        let input = vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::BinaryOp {
            op: Token::Plus,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_parse_subtraction() {
        let input = vec![Token::Number(1.0), Token::Minus, Token::Number(2.0)];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::BinaryOp {
            op: Token::Minus,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::Number(2.0)),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_order_of_operations() {
        let input = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Star,
            Token::Number(3.0),
        ];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::BinaryOp {
            op: Token::Plus,
            left: Box::new(Expr::Number(1.0)),
            right: Box::new(Expr::BinaryOp {
                op: Token::Star,
                left: Box::new(Expr::Number(2.0)),
                right: Box::new(Expr::Number(3.0)),
            }),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_grouping() {
        let input = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
            Token::Star,
            Token::Number(3.0),
        ];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::BinaryOp {
            op: Token::Star,
            left: Box::new(Expr::BinaryOp {
                op: Token::Plus,
                left: Box::new(Expr::Number(1.0)),
                right: Box::new(Expr::Number(2.0)),
            }),
            right: Box::new(Expr::Number(3.0)),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_variable() {
        let input = vec![Token::Variable("$x".to_string())];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::Variable("$x".to_string()));
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_unexpected_token() {
        let input = vec![Token::Plus];
        let mut parser = Parser::new(&input);
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_missing_closing_paren() {
        let input = vec![Token::LParen, Token::Number(1.0)];
        let mut parser = Parser::new(&input);
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_excess_tokens() {
        let input = vec![Token::Number(1.0), Token::Number(2.0)];
        let mut parser = Parser::new(&input);
        assert!(parser.parse().is_err());
    }

    #[test]
    fn test_sqrt() {
        let input = vec![
            Token::Keyword(Word::Sqrt),
            Token::LParen,
            Token::Number(4.0),
            Token::RParen,
        ];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::UnaryOp {
            op: Token::Keyword(Word::Sqrt),
            operand: Box::new(Expr::Number(4.0)),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_sqrt_trailing_comma() {
        let input = vec![
            Token::Keyword(Word::Sqrt),
            Token::LParen,
            Token::Number(4.0),
            Token::Comma,
            Token::RParen,
        ];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::UnaryOp {
            op: Token::Keyword(Word::Sqrt),
            operand: Box::new(Expr::Number(4.0)),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_pow() {
        let input = vec![
            Token::Keyword(Word::Pow),
            Token::LParen,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.0),
            Token::RParen,
        ];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::BinaryOp {
            op: Token::Keyword(Word::Pow),
            left: Box::new(Expr::Number(2.0)),
            right: Box::new(Expr::Number(3.0)),
        });
        assert_eq!(*parser.parse().unwrap(), *expected);
    }

    #[test]
    fn test_inf() {
        let input = vec![Token::Keyword(Word::Inf)];
        let mut parser = Parser::new(&input);
        let expected = Box::new(Expr::Number(f64::INFINITY));
        assert_eq!(*parser.parse().unwrap(), *expected);
    }
}
