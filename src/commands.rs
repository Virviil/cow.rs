use VM::CowVM;
use std::io::stdin;
use std::io::Read;
use std::env;
use std::path::Path;
use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Command {
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
    omm
}

impl Command
{
    pub fn run(&self, state: CowVM) -> CowVM{
        match self {
            moo => do_moo(state),
            mOo => do_mOo(state),
            moO => do_moO(state),
            mOO => do_mOO(state),
            Moo => do_Moo(state),
            MOo => do_MOo(state),
            MoO => do_MoO(state),
            MOO => do_MOO(state),
            OOO => do_OOO(state),
            MMM => do_MMM(state),
            omm => do_omm(state),
            _ => {panic!("invalid command")}
        }
    }
}

fn do_moo(previous_state: CowVM) -> CowVM {
    let mut new_position = previous_state.program_position - 2;
    for i in (0..previous_state.program_position - 1).rev(){
        if previous_state.program[i] == Command::MOO{
            new_position = i;
            break;
        }
    }
    CowVM{ program_position: new_position, ..previous_state}
}

#[allow(non_snake_case)]
fn do_mOo(previous_state: CowVM) ->CowVM {
    CowVM{memory_position: previous_state.memory_position-1, ..previous_state}
}

#[allow(non_snake_case)]
fn do_moO(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    if memory.len() == (previous_state.memory_position - 1){
        memory.push(0);
    }
    CowVM{
        memory_position: previous_state.memory_position+1,
        memory: memory,
        ..previous_state
    }
}

#[allow(non_snake_case)]
fn do_mOO(previous_state: CowVM) -> CowVM {
    match previous_state.memory[previous_state.memory_position] {
        0 => Command::moo.run(previous_state),
        1 => Command::mOo.run(previous_state),
        2 => Command::moO.run(previous_state),
        4 => Command::Moo.run(previous_state),
        5 => Command::MOo.run(previous_state),
        6 => Command::MoO.run(previous_state),
        7 => Command::MOO.run(previous_state),
        8 => Command::OOO.run(previous_state),
        9 => Command::MMM.run(previous_state),
        10 => Command::omm.run(previous_state),
        _ => panic!("Error: invalid command, potential infinite loop")
    }
}

#[allow(non_snake_case)]
fn do_Moo(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    if memory[previous_state.memory_position] == 0 {
        memory[previous_state.memory_position] = getchar() as u32;
    }
    else {
        print!("{}", memory[previous_state.memory_position] as u8 as char);
    }
    CowVM{
        memory: memory,
        ..previous_state
    }
}

#[allow(non_snake_case)]
fn do_MOo(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    memory[previous_state.memory_position] -= 1;
    CowVM{
        memory: memory,
        ..previous_state
    }
}

#[allow(non_snake_case)]
fn do_MoO(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    memory[previous_state.memory_position] += 1;
    CowVM{
        memory: memory,
        ..previous_state
    }
}

#[allow(non_snake_case)]
fn do_MOO(previous_state: CowVM) -> CowVM {
    match previous_state.memory[previous_state.memory_position] {
        0 => {
            let mut new_position = previous_state.program_position+1;
            for command_position in previous_state.program_position+1..previous_state.program.len()-1{
                match previous_state.program[command_position]{
                    Command::moo => new_position = command_position,
                    _ => continue
                }
            }
            CowVM{program_position: new_position, ..previous_state}
        },
        _ => {
            previous_state
        }
    }
}

#[allow(non_snake_case)]
fn do_OOO(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    memory[previous_state.memory_position] = 0;
    CowVM{
        memory: memory,
        ..previous_state
    }
}

#[allow(non_snake_case)]
fn do_MMM(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    let mut register : Option<u32>;
    if let Some(value) = previous_state.register{
        memory[previous_state.memory_position] = value;
        register = None;
    } else {
        register = Some(memory[previous_state.memory_position]);
    }
    CowVM{
        register: register,
        memory: memory,
        ..previous_state
    }
}

#[allow(non_snake_case)]
fn do_OOM(previous_state: CowVM) -> CowVM {
    print!("{}", previous_state.memory[previous_state.memory_position]);
    previous_state
}

#[allow(non_snake_case)]
fn do_omm(previous_state: CowVM) -> CowVM {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("failed to read from stdin");

    let num = s.trim_right().parse::<u32>().expect("failed parsing int from stdin");

    let mut new_memory = previous_state.memory;
    new_memory[previous_state.memory_position] = num;
    CowVM{memory: new_memory, ..previous_state}
    
}

fn getchar() -> u8 {
    stdin()
    .bytes()
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as u8)
    .expect("Error reading character")
}

fn getnum() -> i32 {
    let mut n = String::new();
    stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    n
}