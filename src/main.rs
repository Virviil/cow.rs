mod VM;
mod commands;

use std::env;
use std::path::Path;

fn main() {
    init_vm();
}

fn init_vm() -> () {
    let filepath = env::args().skip(1).next().unwrap();
    let path = Path::new(filepath.as_str());
    println!("{:?}", path);
}
