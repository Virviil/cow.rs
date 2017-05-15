use cow_vm::CowVM;
use std::io::stdin;
use std::io::Read;

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
pub fn do_mOO(state: CowVM) -> CowVM {
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
    let register : Option<i32>;
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
pub fn do_oom(state: CowVM) -> CowVM {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("failed to read from stdin");

    let num = s.trim_right().parse::<i32>().expect("failed parsing int from stdin");

    let mut new_memory = state.memory;
    new_memory[state.memory_position] = num;
    CowVM{
        memory: new_memory,
        program_position: state.program_position+1,
        ..state}

}

pub fn getchar() -> u8 {
    stdin()
    .bytes()
    .next()
    .and_then(|result| result.ok())
    .map(|byte| byte as u8)
    .expect("Error reading character")
}
