#[derive(Debug, Clone)]
pub enum Command {
    Newline,
    Char(char),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub commands: Vec<Command>,
}
