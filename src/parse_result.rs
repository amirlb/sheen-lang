use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};

#[derive(Debug, Clone, Copy)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub location: SourceLocation,
}

impl ParseError {
    pub fn new(message: &str, location: SourceLocation) -> ParseError {
        ParseError {
            message: message.to_string(),
            location,
        }
    }

    pub fn to_io_error(self) -> Error {
        Error::new(
            ErrorKind::InvalidData,
            format!("{} at position {}", self.message, self.location)
        )
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
