mod VM;
mod commands;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use VM::CowVM;

fn main() {
    let initialVM = init_vm();

    loop{
        initialVM.program[initialVM.program_position].run(initialVM);
    }
}

fn init_vm() -> CowVM {
    let filepath = env::args().skip(1).next().expect("No file path specified");
    let path = Path::new(filepath.as_str());
    let mut file = File::open(path).expect("Could not open the file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Could not read the file");
    let commands : Vec<&str> = content.split(" ").collect();
    CowVM::new(commands)
}
