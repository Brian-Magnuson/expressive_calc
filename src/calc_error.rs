use std::{error, fmt};

#[derive(Debug)]
pub enum CalcErrorKind {}

#[derive(Debug)]
pub struct CalcError {
    kind: CalcErrorKind,
    message: String,
}
impl CalcError {
    pub fn new(kind: CalcErrorKind, message: &str) -> Self {
        Self {
            kind,
            message: message.to_string(),
        }
    }
}
impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}
impl error::Error for CalcError {}
