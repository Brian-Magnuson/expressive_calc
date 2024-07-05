use crate::calc_error::CalcError;
use std::{iter::Peekable, str::Chars};

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

pub struct Scanner {}
impl Scanner {
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
}
