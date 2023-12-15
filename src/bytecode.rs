#[derive(Debug, Clone)]
pub enum Instruction {
    Newline,
    IncrementCounter,
    PrintCounter,
    Char(char),
    Halt,
}

#[derive(Debug, Clone)]
pub struct CompiledProgram {
    pub initial_counter: usize,
    pub instructions: Vec<Instruction>,
}
