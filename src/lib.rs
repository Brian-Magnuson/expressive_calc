mod calc_error;
mod interpreter;
mod parser;
mod scanner;

pub use calc_error::CalcError;

/// A simple calculator that can evaluate expressions.
pub struct Calculator {
    interpreter: interpreter::Interpreter,
}
impl Calculator {
    /// Create a new calculator.
    pub fn new() -> Self {
        Self {
            interpreter: interpreter::Interpreter::new(),
        }
    }

    /// Evaluate an expression, storing state between calls.
    ///
    /// This function will scan the input string, parse the tokens, and interpret the expression.
    /// The interpreter will store the result in a variable that can be used in future expressions.
    /// Variables are named based on the order: `$0`, `$1`, `$2`, etc.
    /// The variable name and result are returned as a tuple.
    ///
    /// # Errors
    ///
    /// Returns a [`CalcError`] if an invalid character is encountered, or if an expression cannot be parsed.
    pub fn evaluate(&mut self, input: &str) -> Result<(String, f64), CalcError> {
        let tokens = scanner::Scanner::scan(input)?;

        let mut parser = parser::Parser::new(&tokens);
        let expr = parser.parse()?;

        Ok(self.interpreter.interpret(expr)?)
    }

    /// Evaluate an expression without storing state.
    ///
    /// This function will scan the input string, parse the tokens, and interpret the expression.
    /// The parser and interpreter are dropped after the function returns, meaning
    /// no state is stored between calls.
    ///
    /// # Errors
    ///
    /// Returns a [`CalcError`] if an invalid character is encountered, or if an expression cannot be parsed.
    pub fn quick_evaluate(&self, input: &str) -> Result<f64, CalcError> {
        let tokens = scanner::Scanner::scan(input)?;

        let mut parser = parser::Parser::new(&tokens);
        let expr = parser.parse()?;

        Ok(self.interpreter.quick_interpret(expr)?)
    }
}

// MARK: Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = "1";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_evaluate_addition() {
        let input = "1 + 2";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_subtraction() {
        let input = "1 - 2";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let input = "2 * 3";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_evaluate_division() {
        let input = "6 / 4";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 1.5);
    }

    #[test]
    fn test_evaluate_multiple_operations() {
        let input = "1 + 2 * 3";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_evaluate_parentheses() {
        let input = "(1 + 2) * 3";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 9.0);
    }

    #[test]
    fn test_inf() {
        let input = "inf";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, f64::INFINITY);
    }

    #[test]
    fn test_unary_func() {
        let input = "sqrt(9)";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_binary_func() {
        let input = "pow(2, 3)";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 8.0);
    }
}
