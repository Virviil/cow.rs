mod cow_vm;
mod commands;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use cow_vm::CowVM;

fn main() {
    let mut state = init_vm();
    println!("Initial state:\n\t{:?}\r", state);
    loop {
        state = execute(state);
        println!("New state:\n\t{:?}\r", state);
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
        11 => commands::do_omm(state),
        _ => state,
    }
}

fn init_vm() -> CowVM {
    let filepath = env::args().skip(1).next().expect("No file path specified");
    let path = Path::new(filepath.as_str());
    let mut file = File::open(path).expect("Could not open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read the file");
    let commands : Vec<&str> = content.trim().split(" ").collect();
    CowVM::new(commands)
}
