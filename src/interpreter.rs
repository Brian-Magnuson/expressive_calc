use crate::calc_error::CalcError;
use crate::parser::Expr;

pub struct Interpreter {}
impl Interpreter {
    pub fn interpret(input: Box<Expr>) -> Result<f64, CalcError> {
        Ok(0.0)
    }
}
