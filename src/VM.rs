use commands::Command;

#[derive(Debug, Default, Clone)]
pub struct CowVM {
    pub program: Vec<Command>,
    pub memory: Vec<u32>,
    pub program_position: usize,
    pub memory_position: usize,
    pub register: Option<u32>
}
impl CowVM{
    pub fn new(command_strings : Vec<&str>) -> CowVM{
        let mut commands : Vec<Command> = Vec::with_capacity(command_strings.len());
        for command in command_strings{
            match command {
                "moo" => commands.push(Command::moo),
                "mOo" => commands.push(Command::mOo),
                "moO" => commands.push(Command::moO),
                "mOO" => commands.push(Command::mOO),
                "Moo" => commands.push(Command::Moo),
                "MOo" => commands.push(Command::MOo),
                "MoO" => commands.push(Command::MoO),
                "MOO" => commands.push(Command::MOO),
                "OOO" => commands.push(Command::OOO),
                "MMM" => commands.push(Command::MMM),
                "omm" => commands.push(Command::omm),
                _ => panic!("Couldn't parse command")
            }
        }
        CowVM{
            program: commands,
            memory: Vec::new(),
            program_position: 0,
            memory_position: 0,
            register: None
        }
    }
}