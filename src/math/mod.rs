use std::error::Error;
use std::fmt::{Display, Formatter};

pub mod matrix;
pub mod square_matrix;

#[derive(Debug)]
pub struct MathError {
    message: String,
}

impl Display for MathError where Self: Sized {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.message.as_str())
    }
}

impl Error for MathError {}

impl From<String> for MathError {
    fn from(message: String) -> Self { MathError { message } }
}

impl From<&str> for MathError {
    fn from(message: &str) -> Self { MathError { message: message.to_owned() } }
}
