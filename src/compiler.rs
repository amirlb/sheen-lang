use crate::ast::{Program, Command};
use crate::bytecode::{CompiledProgram, Instruction};

pub fn compile(program: Program) -> CompiledProgram {
    let mut instructions = vec![];
    let mut start_of_line = true;
    for command in program.commands {
        match command {
            Command::Newline => {
                if !start_of_line {
                    instructions.push(Instruction::Newline);
                }
                instructions.push(Instruction::IncrementCounter);
                start_of_line = true;
            }
            Command::Char(ch) => {
                if start_of_line {
                    instructions.push(Instruction::PrintCounter);
                }
                instructions.push(Instruction::Char(ch));
                start_of_line = false;
            }
        }
    }
    instructions.push(Instruction::Halt);

    CompiledProgram {
        initial_counter: 1,
        instructions,
    }
}