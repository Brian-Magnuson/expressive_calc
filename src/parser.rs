//! Module for parsing a vector of tokens into an abstract syntax tree.

use crate::calc_error::CalcError;
use crate::scanner::Token;

use std::{iter::Peekable, slice::Iter};

enum Expr {
    Number(f64),
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

trait Visitor<T> {
    fn visit(&self, expr: &Expr) -> T;
}

pub struct Parser<'a> {
    iter: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            iter: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, CalcError> {
        let result = self.expr();
        // Ensure that the iterator is empty after parsing
        match self.iter.peek() {
            Some(_) => Err(CalcError::new("Unexpected token", None)),
            None => result,
        }
    }

    fn expr(&mut self) -> Result<Box<Expr>, CalcError> {
        self.term()
    }

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

    fn primary(&mut self) -> Result<Box<Expr>, CalcError> {
        match self.iter.next() {
            Some(Token::Number(n)) => Ok(Box::new(Expr::Number(*n))),
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
}
