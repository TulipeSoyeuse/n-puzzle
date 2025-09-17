use colored::{ColoredString, Colorize};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::io;

#[derive(Debug)]
pub struct AppError {
    kind: ColoredString,
    message: ColoredString,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        AppError {
            kind: "Application Error".bold().red(),
            message: message.white(),
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
            kind: String::from("io").bold().red(),
            message: error.to_string().white(),
        }
    }
}
