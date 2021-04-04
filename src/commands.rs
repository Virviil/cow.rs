use std::io::stdin;
use std::io::Read;

use crate::cow_vm::{CowCode, CowVm};

pub fn do_moo(state: CowVm) -> CowVm {
    if state.program_position < 2 {
        end_execution(state)
    } else {
        let mut level = 0;
        let mut new_position = state.program_position - 2;
        for i in (0..state.program_position - 2).rev() {
            match state.program[i] {
                CowCode::moo => level += 1,
                CowCode::MOO => {
                    if level == 0 {
                        new_position = i;
                        break;
                    } else {
                        level -= 1;
                    }
                }
                _ => continue,
            }
        }
        CowVm {
            program_position: new_position,
            ..state
        }
    }
}

#[allow(non_snake_case)]
pub fn do_mOo(state: CowVm) -> CowVm {
    if state.memory_position < 1 {
        end_execution(state)
    } else {
        CowVm {
            memory_position: state.memory_position - 1,
            program_position: state.program_position + 1,
            ..state
        }
    }
}

#[allow(non_snake_case)]
pub fn do_moO(state: CowVm) -> CowVm {
    let mut memory = state.memory;
    if memory.len() == (state.memory_position + 1) {
        memory.push(0);
    }
    CowVm {
        memory_position: state.memory_position + 1,
        memory,
        program_position: state.program_position + 1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_mOO(state: CowVm) -> CowVm {
    match state.memory[state.memory_position] {
        0 => do_moo(state),
        1 => do_mOo(state),
        2 => do_moO(state),
        4 => do_Moo(state),
        5 => do_MOo(state),
        6 => do_MoO(state),
        7 => do_MOO(state),
        8 => do_OOO(state),
        9 => do_MMM(state),
        10 => do_OOM(state),
        11 => do_oom(state),
        _ => panic!("Error: invalid command, potential infinite loop"),
    }
}

#[allow(non_snake_case)]
pub fn do_Moo(state: CowVm) -> CowVm {
    let mut memory = state.memory;
    if memory[state.memory_position] == 0 {
        memory[state.memory_position] = getchar() as i32;
    } else {
        print!("{}", memory[state.memory_position] as u8 as char);
    }
    CowVm {
        memory,
        program_position: state.program_position + 1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MOo(state: CowVm) -> CowVm {
    let mut memory = state.memory;
    memory[state.memory_position] -= 1;
    CowVm {
        memory,
        program_position: state.program_position + 1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MoO(state: CowVm) -> CowVm {
    let mut memory = state.memory;
    memory[state.memory_position] += 1;
    CowVm {
        memory,
        program_position: state.program_position + 1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MOO(state: CowVm) -> CowVm {
    match state.memory[state.memory_position] {
        0 => {
            let mut new_position = state.program_position + 2;
            let mut level = 0;
            for command_position in state.program_position + 2..state.program.len() - 1 {
                match state.program[command_position] {
                    CowCode::MOO => level += 1,
                    CowCode::moo => {
                        if level == 0 {
                            new_position = command_position + 1
                        } else {
                            level -= 1;
                        }
                    }
                    _ => continue,
                }
            }
            CowVm {
                program_position: new_position,
                ..state
            }
        }
        _ => CowVm {
            program_position: state.program_position + 1,
            ..state
        },
    }
}

#[allow(non_snake_case)]
pub fn do_OOO(state: CowVm) -> CowVm {
    let mut memory = state.memory;
    memory[state.memory_position] = 0;
    CowVm {
        memory,
        program_position: state.program_position + 1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_MMM(state: CowVm) -> CowVm {
    let mut memory = state.memory;
    let register: Option<i32>;
    if let Some(value) = state.register {
        memory[state.memory_position] = value;
        register = None;
    } else {
        register = Some(memory[state.memory_position]);
    }
    CowVm {
        register,
        program_position: state.program_position + 1,
        memory,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_OOM(state: CowVm) -> CowVm {
    println!("{}", state.memory[state.memory_position]);
    CowVm {
        program_position: state.program_position + 1,
        ..state
    }
}

#[allow(non_snake_case)]
pub fn do_oom(state: CowVm) -> CowVm {
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("failed to read from stdin");

    let num = s
        .trim_end()
        .parse::<i32>()
        .expect("failed parsing int from stdin");

    let mut new_memory = state.memory;
    new_memory[state.memory_position] = num;
    CowVm {
        memory: new_memory,
        program_position: state.program_position + 1,
        ..state
    }
}

pub fn getchar() -> u8 {
    stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .map(|byte| byte as u8)
        .expect("Error reading character")
}

fn end_execution(state: CowVm) -> CowVm {
    CowVm {
        program_position: state.program.len(),
        ..state
    }
}
