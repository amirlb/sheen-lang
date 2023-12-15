use std::io::Read;

use crate::parse_result::SourceLocation;

pub struct Lexer {
    contents: String,
    position: usize,
    location: SourceLocation,
}

impl Lexer {
    pub fn new(mut reader: impl Read) -> Lexer {
        let mut contents = String::new();
        reader.read_to_string(&mut contents).unwrap();
        Lexer {
            contents,
            position: 0,
            location: SourceLocation { line: 1, column: 1 },
        }
    }
}

impl Iterator for Lexer {
    type Item = (char, SourceLocation);

    fn next(&mut self) -> Option<Self::Item> {
        match self.contents.chars().nth(self.position) {
            None => None,
            Some('\n') => {
                let location = self.location;
                self.position += 1;
                self.location.line += 1;
                self.location.column = 1;
                Some(('\n', location))
            },
            Some(ch) => {
                let location = self.location;
                self.position += 1;
                self.location.column += 1;
                Some((ch, location))
            }
        }
    }
}
