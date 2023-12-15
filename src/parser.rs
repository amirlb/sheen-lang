use std::io::Read;

use crate::lexer::Lexer;
use crate::parse_result::{ParseResult, ParseError};
use crate::ast::{Program, Command};

pub fn parse_file(reader: impl Read) -> ParseResult<Program> {
    let lexer = Lexer::new(reader);
    let mut program = Program { commands: vec![] };
    for (ch, location) in lexer {
        match ch {
            '\n' => { program.commands.push(Command::Newline); }
            'X' => { return Err(ParseError::new("X", location)); }
            _ => program.commands.push(Command::Char(ch)),
        }
    }
    Ok(program)
}
