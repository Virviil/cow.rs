use std::io::Read;
use std::env;
use std::path::Path;

#[derive(Debug, Default, Clone)]
struct CowVM {
    program: Vec<i32>,
    memory: Vec<i32>,
    program_position: i32,
    memory_position: u32,
    register: (bool, i32)
}

#[derive(Debug)]
enum Command {
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

fn main() {
    init_vm();
}

fn init_vm() -> () {
    let filepath = env::args().skip(1).next().unwrap();
    let path = Path::new(filepath.as_str());
    println!("{:?}", path);
}

fn execute(instruction: Command, state: CowVM) -> CowVM {
    println!("Executing command {:?}", &instruction);
    match instruction {
        Command::moo => do_moo(state),
        Command::mOo => do_mOo(state),
        _ => state.clone(),
    }
}

fn do_moo(previous_state: CowVM) -> CowVM {
    // TODO
    println!("moo");
    previous_state.clone()
}

fn do_mOo(previous_state: CowVM) ->CowVM {
    CowVM{memory_position: previous_state.memory_position-1, ..previous_state}
}

fn do_moO(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    if memory.len() == (previous_state.memory_position - 1) as usize {
        memory.push(0);
    }
    CowVM{
        memory_position: previous_state.memory_position+1,
        memory: memory,
        ..previous_state
    }
}

fn do_mOO(previous_state: CowVM) -> CowVM {
    match previous_state.memory[previous_state.memory_position as usize] {
        0 => do_moo(previous_state),
        1 => do_mOo(previous_state),
        2 => do_moo(previous_state),
        _ => exit()
    }
}

fn do_Moo(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    if memory[previous_state.memory_position as usize] == 0 {
        memory[previous_state.memory_position as usize] = getchar() as i32;
    }
    else {
        print!("{}", memory[previous_state.memory_position as usize] as u8 as char);
    }
    CowVM{
        memory: memory,
        ..previous_state
    }
}

fn do_MOo(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    memory[previous_state.memory_position as usize] -= 1;
    CowVM{
        memory: memory,
        ..previous_state
    }
}

fn do_MoO(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    memory[previous_state.memory_position as usize] += 1;
    CowVM{
        memory: memory,
        ..previous_state
    }
}

fn do_MOO(previous_state: CowVM) -> CowVM {
    // implement
    previous_state
}

fn do_OOO(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    memory[previous_state.memory_position as usize] = 0;
    CowVM{
        memory: memory,
        ..previous_state
    }
}

fn do_MMM(previous_state: CowVM) -> CowVM {
    let mut memory = previous_state.memory;
    let register = match previous_state.register {
        (false, _) => (true, memory[previous_state.memory_position as usize]),
        (true, value) => {
            memory[previous_state.memory_position as usize] = value;
            (false, 0)
        }
    };
    CowVM{
        register: register,
        memory: memory,
        ..previous_state
    }
}

fn do_OOM(previous_state: CowVM) -> CowVM {
    print!("{}", previous_state.memory[previous_state.memory_position as usize]);
    previous_state
}

fn exit() -> ! {
    panic!("EXIT");
}


fn getchar() -> char {
    std::io::stdin()
        .bytes()
        .next()
        .and_then(|result| result.ok())
        .unwrap() as char
}

fn getnum() -> i32 {
    let mut n = String::new();
    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input.");
    let n: i32 = n.trim().parse().expect("invalid input");
    n
}