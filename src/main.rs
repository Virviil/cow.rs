mod cow_vm;
mod commands;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use cow_vm::CowVM;

fn main() {
    let mut state = init_vm();
    // println!("Initial state:\n\t{:?}\r", state);
    loop {
        if state.program_position == state.program.len() {
            break;
        }
        state = execute(state);
        // println!("New state:\n\t{:?}\r", state);
    }
}

fn execute(state: CowVM) -> CowVM {
    let command = state.program[state.program_position];
    match command {
        0 => commands::do_moo(state),
        1 => commands::do_mOo(state),
        2 => commands::do_moO(state),
        3 => commands::do_mOO(state),
        4 => commands::do_Moo(state),
        5 => commands::do_MOo(state),
        6 => commands::do_MoO(state),
        7 => commands::do_MOO(state),
        8 => commands::do_OOO(state),
        9 => commands::do_MMM(state),
        10 => commands::do_OOM(state),
        11 => commands::do_oom(state),
        _ => state,
    }
}

fn init_vm() -> CowVM {
    let filepath = env::args().skip(1).next().expect("No file path specified");
    let path = Path::new(filepath.as_str());
    let mut file = File::open(path).expect("Could not open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read the file");
    new_vm(content)
}

fn new_vm(program_string : String) -> CowVM{
    let mut buff : [char; 3] = [0 as char; 3];

    let commands : Vec<u32> =
    program_string
    .chars()
    .map(|e| {
        buff[0] = buff[1];
        buff[1] = buff[2];
        buff[2] = e;

        let val = match (buff[0], buff[1], buff[2]){
            ('m', 'o', 'o') => 0,
            ('m', 'O', 'o') => 1,
            ('m', 'o', 'O') => 2,
            ('m', 'O', 'O') => 3,
            ('M', 'o', 'o') => 4,
            ('M', 'O', 'o') => 5,
            ('M', 'o', 'O') => 6,
            ('M', 'O', 'O') => 7,
            ('O', 'O', 'O') => 8,
            ('M', 'M', 'M') => 9,
            ('O', 'O', 'M') => 10,
            ('o', 'o', 'm') => 11,
            (_, _, _) => 99 //invalid command
        };
        if val != 99 {
            buff = [0 as char; 3];
        }
        val
    })
    .filter(|e| *e != 99)
    .collect();

    CowVM{
        program: commands,
        memory: vec![0],
        program_position: 0,
        memory_position: 0,
        register: None
    }
}
