use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::io::ErrorKind;
use std::path::Path;

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
}

impl Error for ParseError {}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} at position {}", self.message, self.location)
    }
}

impl From<ParseError> for io::Error {
    fn from(e: ParseError) -> Self {
        io::Error::new(ErrorKind::Other, e)
    }
}

pub type ParseResult<T> = Result<T, ParseError>;

#[derive(Debug, Clone)]
struct FileContext<T> {
    file_name: Box<Path>,
    error: T,
}

impl<T> Error for FileContext<T> where T: Error {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.error.source()
    }
}

impl<T> Display for FileContext<T> where T: Display {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error with file {:?}:\n{}", self.file_name, self.error)
    }
}

pub fn in_file_context(file_name: &Path, error: io::Error) -> io::Error {
    let kind = error.kind();
    let context = FileContext {
        file_name: file_name.to_path_buf().into_boxed_path(),
        error,
    };
    io::Error::new(kind, context)
}
