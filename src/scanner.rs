//! Module for scanning an input string and converting it into a vector of tokens.

use crate::calc_error::CalcError;
use std::{iter::Peekable, str::Chars};

/// Enum for the different reserved words in the calculator.
///
/// Keywords are special tokens that have a specific meaning in the calculator.
/// These include functions like `sqrt`.
#[derive(Clone, Debug, PartialEq)]
pub enum Word {
    // Numbers
    Inf,
    Pi,
    Tau,
    E,
    Phi,

    // Unary operations
    Sqrt,
    Cbrt,
    Exp,
    Log2,
    Log10,
    Ln,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Sinh,
    Cosh,
    Tanh,
    Asinh,
    Acosh,
    Atanh,
    Rad,
    Deg,
    Abs,
    Floor,
    Ceil,
    Trunc,
    Round,

    // Binary operations
    Pow,
    Log,
    Hypot,
    Atan2,
    Mod,
    Max,
    Min,
}

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
    Comma,
    Variable(String),
    Keyword(Word),
}

/// A scanner used to help convert an input string into a vector of tokens.
///
/// First, create a new scanner with [`Scanner::new`], then call [`Scanner::scan`] to convert the input string into tokens.
pub struct Scanner<'a> {
    iter: Peekable<Chars<'a>>,
}
impl<'a> Scanner<'a> {
    /// Create a new scanner with the input string.
    pub fn new(input: &'a str) -> Self {
        Self {
            iter: input.chars().peekable(),
        }
    }

    /// Scans the input string and returns a vector of tokens.
    ///
    /// Consumes the Scanner to iterate over the input string.
    ///
    /// # Errors
    ///
    /// Returns a [`CalcError`] if an invalid character is encountered, or if a number cannot be parsed.
    pub fn scan(mut self) -> Result<Vec<Token>, CalcError> {
        let mut tokens = Vec::new();

        loop {
            match self.iter.peek() {
                None => return Ok(tokens),
                Some(c) => match c {
                    ' ' => {
                        self.iter.next();
                    }
                    '+' => {
                        tokens.push(Token::Plus);
                        self.iter.next();
                    }
                    '-' => {
                        tokens.push(Token::Minus);
                        self.iter.next();
                    }
                    '*' => {
                        tokens.push(Token::Star);
                        self.iter.next();
                    }
                    '/' => {
                        tokens.push(Token::Slash);
                        self.iter.next();
                    }
                    '(' => {
                        tokens.push(Token::LParen);
                        self.iter.next();
                    }
                    ')' => {
                        tokens.push(Token::RParen);
                        self.iter.next();
                    }
                    ',' => {
                        tokens.push(Token::Comma);
                        self.iter.next();
                    }
                    'a'..='z' | 'A'..='Z' => {
                        tokens.push(Token::Keyword(self.scan_word()?));
                    }
                    '$' => {
                        self.iter.next();
                        tokens.push(Token::Variable(self.scan_variable()?));
                    }
                    '0'..='9' => {
                        tokens.push(Token::Number(self.scan_number()?));
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
    fn scan_number(&mut self) -> Result<f64, CalcError> {
        let mut number = String::new();
        loop {
            match self.iter.peek() {
                None => break,
                Some(c) => match c {
                    '0'..='9' | '.' => {
                        number.push(*c);
                        self.iter.next();
                    }
                    'E' | 'e' => {
                        number.push(*c);
                        self.iter.next();
                        match self.iter.peek() {
                            Some(&'+') | Some(&'-') => {
                                number.push(self.iter.next().unwrap());
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

    /// Scans a variable from the input iterator.
    ///
    /// All variables must start with a '$' and can contain any alphanumeric character.
    /// The exact characters that are accepted can be expressed as `[0-9a-zA-Z_]` in regex.
    /// If an invalid character is encountered, the variable is considered complete.
    /// An exception is when there are no characters following the '$'.
    ///
    /// # Errors
    ///
    /// Returns a [`CalcError`] if there were no alphanumeric characters following the '$'.
    /// For example, scanning `$v#` will not return an error immediately, but `$#` will.
    fn scan_variable(&mut self) -> Result<String, CalcError> {
        let mut variable = String::from("$");
        let mut has_char = false;

        loop {
            match self.iter.peek() {
                None => break,
                Some(c) => match c {
                    '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => {
                        variable.push(*c);
                        has_char = true;
                        self.iter.next();
                    }
                    _ => break,
                },
            }
        }

        if !has_char {
            return Err(CalcError::new("Invalid variable", None));
        }

        Ok(variable)
    }

    /// Scans a reserved word from the input iterator.
    ///
    /// Returns a [`Word`] enum representing the reserved word.
    /// Reserved words include special functions like `sqrt`.
    /// Reserved words also include constants like `pi` and special values like `inf`.
    /// This function consumes all characters that could be part of the keyword.
    /// This happens to include uppercase letters despite all reserved words being lowercase.
    ///
    /// # Errors
    ///
    /// If an unknown keyword is encountered, a [`CalcError`] is returned.
    fn scan_word(&mut self) -> Result<Word, CalcError> {
        let mut keyword = String::new();
        loop {
            match self.iter.peek() {
                None => break,
                Some(c) => match c {
                    'a'..='z' | 'A'..='Z' => {
                        keyword.push(*c);
                        self.iter.next();
                    }
                    _ => break,
                },
            }
        }

        match keyword.as_str() {
            "inf" => Ok(Word::Inf),
            "pi" => Ok(Word::Pi),
            "tau" => Ok(Word::Tau),
            "e" => Ok(Word::E),
            "phi" => Ok(Word::Phi),

            "sqrt" => Ok(Word::Sqrt),
            "cbrt" => Ok(Word::Cbrt),
            "exp" => Ok(Word::Exp),
            "log2" => Ok(Word::Log2),
            "log10" => Ok(Word::Log10),
            "ln" => Ok(Word::Ln),
            "sin" => Ok(Word::Sin),
            "cos" => Ok(Word::Cos),
            "tan" => Ok(Word::Tan),
            "asin" => Ok(Word::Asin),
            "acos" => Ok(Word::Acos),
            "atan" => Ok(Word::Atan),
            "sinh" => Ok(Word::Sinh),
            "cosh" => Ok(Word::Cosh),
            "tanh" => Ok(Word::Tanh),
            "asinh" => Ok(Word::Asinh),
            "acosh" => Ok(Word::Acosh),
            "atanh" => Ok(Word::Atanh),
            "rad" => Ok(Word::Rad),
            "deg" => Ok(Word::Deg),
            "abs" => Ok(Word::Abs),
            "floor" => Ok(Word::Floor),
            "ceil" => Ok(Word::Ceil),
            "trunc" => Ok(Word::Trunc),
            "round" => Ok(Word::Round),

            "pow" => Ok(Word::Pow),
            "log" => Ok(Word::Log),
            "hypot" => Ok(Word::Hypot),
            "atan2" => Ok(Word::Atan2),
            "mod" => Ok(Word::Mod),
            "max" => Ok(Word::Max),
            "min" => Ok(Word::Min),
            _ => Err(CalcError::new("Unknown keyword", None)),
        }
    }
}

// MARK: Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_empty_str() {
        let input = "";
        let expected = vec![];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_whitespace() {
        let input = "  ";
        let expected = vec![];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_plus() {
        let input = "+";
        let expected = vec![Token::Plus];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_minus() {
        let input = "-";
        let expected = vec![Token::Minus];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_digit() {
        let input = "0";
        let expected = vec![Token::Number(0.0)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_number() {
        let input = "123.456";
        let expected = vec![Token::Number(123.456)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_number_scientific_notation() {
        let input = "1.23E4";
        let expected = vec![Token::Number(1.23E4)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_number_negative_exponent() {
        let input = "1.23E-4";
        let expected = vec![Token::Number(1.23E-4)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_scan_number_plus_exponent() {
        let input = "1.23E+4";
        let expected = vec![Token::Number(1.23E4)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_addition() {
        let input = "1 + 2";
        let expected = vec![Token::Number(1.0), Token::Plus, Token::Number(2.0)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_negation() {
        let input = "-1";
        let expected = vec![Token::Minus, Token::Number(1.0)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_multiplication() {
        let input = "2 * 3";
        let expected = vec![Token::Number(2.0), Token::Star, Token::Number(3.0)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
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
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
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
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_add_scientific_notation() {
        let input = "1.23E4 + 5.67E-8";
        let expected = vec![Token::Number(1.23E4), Token::Plus, Token::Number(5.67E-8)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_err_invalid_char() {
        let input = "1 + a";
        let scanner = Scanner::new(input);
        assert!(matches!(scanner.scan(), Err(CalcError { .. })));
    }

    #[test]
    fn test_variable() {
        let input = "$var";
        let expected = vec![Token::Variable(String::from("$var"))];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_keyword() {
        let input = "sqrt";
        let expected = vec![Token::Keyword(Word::Sqrt)];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }

    #[test]
    fn test_keyword_with_args() {
        let input = "pow(2, 3)";
        let expected = vec![
            Token::Keyword(Word::Pow),
            Token::LParen,
            Token::Number(2.0),
            Token::Comma,
            Token::Number(3.0),
            Token::RParen,
        ];
        let scanner = Scanner::new(input);
        assert_eq!(scanner.scan().unwrap(), expected);
    }
}
