use cow_vm::CowVM;
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

pub fn do_moo(state: CowVM) -> CowVM {
    let mut level = 0;
    let mut new_position = state.program_position - 2;
    for i in (0..state.program_position).rev(){
        if state.program[i] == 0 {
            level += 1;
        }
        if state.program[i] == 7 {
            if level == 0 {
                new_position = i;
                break;
            }
            else {
                level -= 1;
            }
        }
    }
    CowVM{
        program_position: new_position,
        ..state}
}

#[allow(non_snake_case)]
pub fn do_mOo(state: CowVM) ->CowVM {
    CowVM{
        memory_position: state.memory_position-1,
        program_position: state.program_position+1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_moO(state: CowVM) -> CowVM {
    let mut memory = state.memory;
    if memory.len() == (state.memory_position + 1){
        memory.push(0);
    }
    CowVM{
        memory_position: state.memory_position+1,
        memory: memory,
        program_position: state.program_position+1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_mOO(previous_state: CowVM) -> CowVM {
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
pub fn do_Moo(state: CowVM) -> CowVM {
    let mut memory = state.memory;
    if memory[state.memory_position] == 0 {
        memory[state.memory_position] = getchar() as i32;
    }
    else {
        print!("{}", memory[state.memory_position] as u8 as char);
    }
    CowVM{
        memory: memory,
        program_position: state.program_position+1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MOo(state: CowVM) -> CowVM {
    let mut memory = state.memory;
    memory[state.memory_position] -= 1;
    CowVM{
        memory: memory,
        program_position: state.program_position+1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MoO(state: CowVM) -> CowVM {
    let mut memory = state.memory;
    memory[state.memory_position] += 1;
    CowVM{
        memory: memory,
        program_position: state.program_position+1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MOO(state: CowVM) -> CowVM {
    match state.memory[state.memory_position] {
        0 => {
            let mut new_position = state.program_position+1;
            for command_position in state.program_position+1..state.program.len()-1{
                match state.program[command_position]{
                    0 => new_position = command_position+1,
                    _ => continue
                }
            }
            CowVM{
                program_position: new_position,
                ..state
            }
        },
        _ => CowVM{
            program_position: state.program_position+1,
            ..state
        }
    }
}

#[allow(non_snake_case)]
pub fn do_OOO(state: CowVM) -> CowVM {
    let mut memory = state.memory;
    memory[state.memory_position] = 0;
    CowVM{
        memory: memory,
        program_position: state.program_position+1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MMM(state: CowVM) -> CowVM {
    let mut memory = state.memory;
    let mut register : Option<i32>;
    if let Some(value) = state.register{
        memory[state.memory_position] = value;
        register = None;
    } else {
        register = Some(memory[state.memory_position]);
    }
    CowVM{
        register: register,
        program_position: state.program_position+1,
        memory: memory,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_OOM(state: CowVM) -> CowVM {
    print!("{}\n", state.memory[state.memory_position]);
    CowVM{
        program_position: state.program_position+1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_omm(previous_state: CowVM) -> CowVM {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("failed to read from stdin");

    let num = s.trim_right().parse::<i32>().expect("failed parsing int from stdin");

    let mut new_memory = previous_state.memory;
    new_memory[previous_state.memory_position] = num;
    CowVM{memory: new_memory, ..previous_state}

}

pub fn getchar() -> u8 {
    stdin()
    .bytes()
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as u8)
    .expect("Error reading character")
}

pub fn getnum() -> i32 {
    let mut n = String::new();
    stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    n
}
