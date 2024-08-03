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
        let scanner = scanner::Scanner::new(input);
        let tokens = scanner.scan()?;

        let parser = parser::Parser::new(&tokens);
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
        let scanner = scanner::Scanner::new(input);
        let tokens = scanner.scan()?;

        let parser = parser::Parser::new(&tokens);
        let expr = parser.parse()?;

        Ok(self.interpreter.quick_interpret(expr)?)
    }

    /// Reset the calculator, clearing all stored state.
    ///
    /// This function resets the interpreter.
    /// All stored variables are cleared, and the variable count is reset to zero.
    pub fn reset(&mut self) {
        self.interpreter.reset();
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

    #[test]
    fn test_abs_bars() {
        let input = "|-42|";
        let calculator = Calculator::new();
        let result = calculator.quick_evaluate(input).unwrap();
        assert_eq!(result, 42.0);
    }

    #[test]
    fn test_state() {
        let input = "1 + 2";
        let mut calculator = Calculator::new();
        let result = calculator.evaluate(input).unwrap();
        assert_eq!(result, ("$0".to_string(), 3.0));

        let input = "$0 * 3";
        let result = calculator.evaluate(input).unwrap();
        assert_eq!(result, ("$1".to_string(), 9.0));
    }

    #[test]
    fn test_reset() {
        let input = "1 + 2";
        let mut calculator = Calculator::new();
        let result = calculator.evaluate(input).unwrap();
        assert_eq!(result, ("$0".to_string(), 3.0));

        calculator.reset();
        assert!(matches!(calculator.quick_evaluate("$0"), Err(_)));

        let input = "1 + 3";
        let result = calculator.evaluate(input).unwrap();
        assert_eq!(result, ("$0".to_string(), 4.0));
    }
}
