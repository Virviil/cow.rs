#[derive(Debug, Default, Clone)]
pub struct CowVM {
    // pub program: Vec<Command>,
    pub program: Vec<u32>,
    pub memory: Vec<i32>,
    pub program_position: usize,
    pub memory_position: usize,
    pub register: Option<i32>
}
