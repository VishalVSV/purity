use purity_lib::runtime::Program;
use crate::vm_decl::PureVM;

mod p1;
mod vm_decl;

fn main() {
    let prog: Vec<u8> = vec![0,0,0,10,0,0,0,2,0,0,0,0,128,0,0,0,0,2,0,0,0,1,3,255];
    let program = Program::load_from_bytes(&prog);

    p1::P1::run(program);
}
