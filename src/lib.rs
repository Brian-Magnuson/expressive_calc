mod calc_error;
pub use calc_error::CalcError;

pub mod interpreter;
pub mod parser;
pub mod scanner;

pub fn evaluate(input: &str) -> Result<f64, CalcError> {
    let tokens = scanner::Scanner::scan(input)?;

    let mut parser = parser::Parser::new(&tokens);
    let expr = parser.parse()?;

    let interpreter = interpreter::Interpreter::new();

    interpreter.interpret(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = "1";
        let result = evaluate(input).unwrap();
        assert_eq!(result, 1.0);
    }

    #[test]
    fn test_evaluate_addition() {
        let input = "1 + 2";
        let result = evaluate(input).unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_evaluate_subtraction() {
        let input = "1 - 2";
        let result = evaluate(input).unwrap();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_evaluate_multiplication() {
        let input = "2 * 3";
        let result = evaluate(input).unwrap();
        assert_eq!(result, 6.0);
    }

    #[test]
    fn test_evaluate_division() {
        let input = "6 / 4";
        let result = evaluate(input).unwrap();
        assert_eq!(result, 1.5);
    }

    #[test]
    fn test_evaluate_unary_minus() {
        let input = "-1";
        let result = evaluate(input).unwrap();
        assert_eq!(result, -1.0);
    }

    #[test]
    fn test_evaluate_parentheses() {
        let input = "(1 + 2) * 3";
        let result = evaluate(input).unwrap();
        assert_eq!(result, 9.0);
    }
}
