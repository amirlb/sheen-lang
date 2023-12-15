use std::path::Path;
use std::io;
use std::io::ErrorKind;

mod ast;
mod parse_result;
mod lexer;
mod parser;
mod bytecode;
mod compiler;
mod vm;
mod interpreter;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn run() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err(io::Error::new(ErrorKind::InvalidInput, "Please provide a single argument"));
    }

    match args[1].as_str() {
        "-h" | "--help" => {
            println!("Usage: {} [-h|--help] [-v|--version] filename.shn", args[0]);
        }
        "-v" | "--version" => {
            println!("Sheen compiler, version {}", VERSION);
        }
        file_name => {
            let path = Path::new(file_name);
            interpreter::execute_script(path)?;
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
