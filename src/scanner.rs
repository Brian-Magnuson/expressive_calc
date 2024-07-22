//! Module for scanning an input string and converting it into a vector of tokens.

use crate::calc_error::CalcError;
use std::{iter::Peekable, str::Chars};

/// Enum for the different types of tokens that can be scanned.
///
/// Token types include numbers, operators, and parentheses.
/// All numbers are represented as f64.
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

/// A scanner used to help convert an input string into a vector of tokens.
///
/// Provides a scanning function, [`Scanner::scan`], that converts an input string into a vector of tokens.
pub struct Scanner {}
impl Scanner {
    /// Scans the input string and returns a vector of tokens.
    ///
    /// # Errors
    ///
    /// Returns a [`CalcError`] if an invalid character is encountered, or if a number cannot be parsed.
    pub fn scan(input: &str) -> Result<Vec<Token>, CalcError> {
        let mut input_iter = input.chars().peekable();
        let mut tokens = Vec::new();

        loop {
            match input_iter.peek() {
                None => return Ok(tokens),
                Some(c) => match c {
                    ' ' => {
                        input_iter.next();
                    }
                    '+' => {
                        tokens.push(Token::Plus);
                        input_iter.next();
                    }
                    '-' => {
                        tokens.push(Token::Minus);
                        input_iter.next();
                    }
                    '*' => {
                        tokens.push(Token::Star);
                        input_iter.next();
                    }
                    '/' => {
                        tokens.push(Token::Slash);
                        input_iter.next();
                    }
                    '(' => {
                        tokens.push(Token::LParen);
                        input_iter.next();
                    }
                    ')' => {
                        tokens.push(Token::RParen);
                        input_iter.next();
                    }
                    '0'..='9' => {
                        tokens.push(Token::Number(Scanner::scan_number(&mut input_iter)?));
                    }
                    _ => return Err(CalcError::new("Invalid character", None)),
                },
            }
        }
    }

    /// Scans an f64 from the input iterator.
    ///
    /// Effectively consumes all the characters from the iterator that could be part of the number,
    /// then calls [`str::parse`](https://doc.rust-lang.org/std/primitive.str.html#method.parse) to convert the string to an f64.
    /// The behavior of `parse` is based on [`f64::from_str`](https://doc.rust-lang.org/std/primitive.f64.html#method.from_str).
    /// Number characters include digits, a decimal point, and 'E' or 'e' for scientific notation.
    /// If 'E' or 'e', any '+' or '-' that follows is also consumed as part of the number.
    ///
    /// # Errors
    ///
    /// If the number cannot be parsed, a [`CalcError`] is returned containing the [`std::num::ParseFloatError`].
    fn scan_number(input_iter: &mut Peekable<Chars>) -> Result<f64, CalcError> {
        let mut number = String::new();
        loop {
            match input_iter.peek() {
                None => break,
                Some(c) => match c {
                    '0'..='9' | '.' => {
                        number.push(*c);
                        input_iter.next();
                    }
                    'E' | 'e' => {
                        number.push(*c);
                        input_iter.next();
                        match input_iter.peek() {
                            Some(&'+') | Some(&'-') => {
                                number.push(input_iter.next().unwrap());
                            }
                            _ => {}
                        }
                    }
                    _ => break,
                },
            }
        }

        match number.parse() {
            Ok(n) => Ok(n),
            Err(err) => Err(CalcError::new("Failed to parse number", Some(err.into()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_empty_str() {
        let input = "";
        let expected = vec![];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_whitespace() {
        let input = "  ";
        let expected = vec![];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_plus() {
        let input = "+";
        let expected = vec![Token::Plus];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_minus() {
        let input = "-";
        let expected = vec![Token::Minus];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_digit() {
        let input = "0";
        let expected = vec![Token::Number(0.0)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_number() {
        let input = "123.456";
        let expected = vec![Token::Number(123.456)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_number_scientific_notation() {
        let input = "1.23E4";
        let expected = vec![Token::Number(1.23E4)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_number_negative_exponent() {
        let input = "1.23E-4";
        let expected = vec![Token::Number(1.23E-4)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_scan_number_plus_exponent() {
        let input = "1.23E+4";
        let expected = vec![Token::Number(1.23E4)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_addition() {
        let input = "1 + 2";
        let expected = vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_negation() {
        let input = "-1";
        let expected = vec![Token::Minus, Token::Number(1.0)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_multiplication() {
        let input = "2 * 3";
        let expected = vec![Token::Number(2.0), Token::Star, Token::Number(3.0)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_three_terms() {
        let input = "1 + 2 * 3";
        let expected = vec![
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::Star,
            Token::Number(3.0),
        ];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_parentheses() {
        let input = "(1 + 2) * 3";
        let expected = vec![
            Token::LParen,
            Token::Number(1.0),
            Token::Plus,
            Token::Number(2.0),
            Token::RParen,
            Token::Star,
            Token::Number(3.0),
        ];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_add_scientific_notation() {
        let input = "1.23E4 + 5.67E-8";
        let expected = vec![Token::Number(1.23E4), Token::Plus, Token::Number(5.67E-8)];
        assert_eq!(Scanner::scan(input).unwrap(), expected);
    }

    #[test]
    fn test_err_invalid_char() {
        let input = "1 + a";
        assert!(matches!(Scanner::scan(input), Err(CalcError { .. })));
    }
}
