#[derive(Debug, Default, Clone)]
pub struct CowVm {
    pub program: Vec<CowCode>,
    pub memory: Vec<i32>,
    pub program_position: usize,
    pub memory_position: usize,
    pub register: Option<i32>,
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy)]
pub enum CowCode {
    moo,
    mOo,
    moO,
    mOO,
    Moo,
    MOo,
    MoO,
    MOO,
    OOO,
    MMM,
    OOM,
    oom,
}
