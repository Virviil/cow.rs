use commands::Command;

#[derive(Debug, Default, Clone)]
pub struct CowVM {
    pub program: Vec<Command>,
    pub memory: Vec<u32>,
    pub program_position: usize,
    pub memory_position: usize,
    pub register: (bool, u32)
}