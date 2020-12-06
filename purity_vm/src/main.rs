mod runtime;

use runtime::*;

fn main() {
    println!("Hello, world!");
    let prog: Vec<u8> = vec![0,0,0,255];
    let program = Program::load_from_bytes(&prog);
}
