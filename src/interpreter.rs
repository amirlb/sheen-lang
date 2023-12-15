use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufReader;

use crate::parser::parse_file;
use crate::compiler::compile;
use crate::vm::VM;

/*
    Skeleton of an interpreter: the laguage currently implemented simply prints
    the program file, with line numbering and skipping empty lines.
    Still, this illustrates reporting of parse errors, compilation, and execution.
 */

pub fn execute_script(file_name: &Path) -> io::Result<()> {
    let file = File::open(file_name)?;
    let mut reader = BufReader::new(file);
    let program = parse_file(&mut reader).map_err(|e| e.to_io_error())?;
    let bytecode = compile(program);
    VM::new(bytecode).run();
    Ok(())
}
