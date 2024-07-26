use std::{error, fmt};

/// Error type for the calculator.
#[derive(Debug)]
pub struct CalcError {
    message: String,
    source: Option<Box<dyn error::Error>>,
}
impl CalcError {
    pub fn new(message: &str, source: Option<Box<dyn error::Error>>) -> Self {
        Self {
            message: message.to_string(),
            source,
        }
    }
}
impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CalcError: {}", self.message)
    }
}
impl error::Error for CalcError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref())
    }
}
