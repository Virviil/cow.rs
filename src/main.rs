mod commands;
mod cow_vm;

use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::cow_vm::{CowCode, CowVm};

fn main() {
    let mut state = init_vm();
    // println!("Initial state:\n\t{:?}\r", state);
    loop {
        if state.program_position == state.program.len() {
            break;
        }
        state = execute(state);
        // println!("New state:{:?}\r", state);
    }
}

fn execute(state: CowVm) -> CowVm {
    let command = state.program[state.program_position];
    match command {
        CowCode::moo => commands::do_moo(state),
        CowCode::mOo => commands::do_mOo(state),
        CowCode::moO => commands::do_moO(state),
        CowCode::mOO => commands::do_mOO(state),
        CowCode::Moo => commands::do_Moo(state),
        CowCode::MOo => commands::do_MOo(state),
        CowCode::MoO => commands::do_MoO(state),
        CowCode::MOO => commands::do_MOO(state),
        CowCode::OOO => commands::do_OOO(state),
        CowCode::MMM => commands::do_MMM(state),
        CowCode::OOM => commands::do_OOM(state),
        CowCode::oom => commands::do_oom(state),
    }
}

fn init_vm() -> CowVm {
    let filepath = env::args().nth(1).expect("No file path specified");
    let path = Path::new(filepath.as_str());
    let mut file = File::open(path).expect("Could not open the file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read the file");
    new_vm(content)
}

fn new_vm(program_string: String) -> CowVm {
    let mut buff: [u8; 3] = [0; 3];

    let commands: Vec<CowCode> = program_string
        .into_bytes()
        .into_iter()
        .filter_map(|e| {
            buff[0] = buff[1];
            buff[1] = buff[2];
            buff[2] = e;

            let val = match (buff[0], buff[1], buff[2]) {
                (b'm', b'o', b'o') => Some(CowCode::moo),
                (b'm', b'O', b'o') => Some(CowCode::mOo),
                (b'm', b'o', b'O') => Some(CowCode::moO),
                (b'm', b'O', b'O') => Some(CowCode::mOO),
                (b'M', b'o', b'o') => Some(CowCode::Moo),
                (b'M', b'O', b'o') => Some(CowCode::MOo),
                (b'M', b'o', b'O') => Some(CowCode::MoO),
                (b'M', b'O', b'O') => Some(CowCode::MOO),
                (b'O', b'O', b'O') => Some(CowCode::OOO),
                (b'M', b'M', b'M') => Some(CowCode::MMM),
                (b'O', b'O', b'M') => Some(CowCode::OOM),
                (b'o', b'o', b'm') => Some(CowCode::oom),
                _ => None, //invalid command
            };
            if val.is_some() {
                buff = [0; 3];
            }
            val
        })
        .collect();

    CowVm {
        program: commands,
        memory: vec![0],
        program_position: 0,
        memory_position: 0,
        register: None,
    }
}
