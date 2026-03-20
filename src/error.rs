use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io;

#[derive(Debug)]
pub struct AppError {
    kind: String,
    message: String,
}

impl AppError {
    pub fn new(message: &'static str) -> Self {
        AppError {
            kind: String::from("Application Error"),
            message: String::from(message),
        }
    }
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl From<io::Error> for AppError {
    fn from(error: io::Error) -> Self {
        AppError {
            kind: String::from("io"),
            message: error.to_string(),
        }
    }
}
