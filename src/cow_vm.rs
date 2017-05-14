use commands::Command;

#[derive(Debug, Default, Clone)]
pub struct CowVM {
    // pub program: Vec<Command>,
    pub program: Vec<u32>,
    pub memory: Vec<i32>,
    pub program_position: usize,
    pub memory_position: usize,
    pub register: Option<i32>
}
impl CowVM{
    pub fn new(command_strings : Vec<&str>) -> CowVM{
        let mut commands : Vec<u32> = Vec::with_capacity(command_strings.len());
        for command in command_strings{
            match command {
                "moo" => commands.push(0),
                "mOo" => commands.push(1),
                "moO" => commands.push(2),
                "mOO" => commands.push(3),
                "Moo" => commands.push(4),
                "MOo" => commands.push(5),
                "MoO" => commands.push(6),
                "MOO" => commands.push(7),
                "OOO" => commands.push(8),
                "MMM" => commands.push(9),
                "OOM" => commands.push(10),
                "omm" => commands.push(11),
                _ => panic!("Couldn't parse command")
            }
        }
        CowVM{
            program: commands,
            memory: vec![0],
            program_position: 0,
            memory_position: 0,
            register: None
        }
    }
}
