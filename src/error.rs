use std::fmt::{self, write};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken,
    ExpectedNumber,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected token"),
            ParseError::ExpectedNumber => write!(f, "Expected number"),
        }
    }
}
