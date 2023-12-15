use crate::bytecode::{CompiledProgram, Instruction};

pub struct VM {
    instructions: Vec<Instruction>,
    pc: usize,
    counter: usize,
}

impl VM {
    pub fn new(program: CompiledProgram) -> VM {
        VM {
            instructions: program.instructions,
            pc: 0,
            counter: program.initial_counter,
        }
    }

    pub fn run(&mut self) {
        loop {
            let inst = &self.instructions[self.pc];
            self.pc += 1;

            match inst {
                Instruction::Newline => {
                    println!();
                }
                Instruction::IncrementCounter => {
                    self.counter += 1;
                }
                Instruction::PrintCounter => {
                    print!("{:4}:  ", self.counter);
                }
                Instruction::Char(ch) => {
                    print!("{}", ch);
                }
                Instruction::Halt => {
                    break;
                }
            }
        }
    }
}
