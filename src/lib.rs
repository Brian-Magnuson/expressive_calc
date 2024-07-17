mod calc_error;
pub use calc_error::CalcError;

pub mod interpreter;
pub mod parser;
pub mod scanner;

pub fn evaluate(_input: &str) -> Result<f64, CalcError> {
    Ok(0.0)
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
