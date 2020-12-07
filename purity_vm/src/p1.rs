use purity_lib::types::PureTypes;
use purity_lib::runtime::Program;
use purity_lib::runtime::p1;

use crate::vm_decl::{PureVM,PureVMResults};

pub struct P1;

impl PureVM for P1 {
    fn run(program: Program) -> PureVMResults {
        let mut iter = program.bytecode.iter();

        let mut stack: Vec<PureTypes> = Vec::new();

        loop {
            if let Some(opcode) = iter.next() {
                match *opcode {
                    p1::opcodes::OP_LOAD_CONSTANT_8 => {
                        let index = *iter.next().expect("Couldn't find operand for OP_LOAD_CONSTANT") as usize;
                        stack.push(program.constant_pool[index].clone());
                    },
                    p1::opcodes::OP_LOAD_CONSTANT_16 => {
                        let index = Program::read_u16(&mut iter).expect("Couldn't find operand for OP_LOAD_CONSTANT") as usize;
                        stack.push(program.constant_pool[index].clone());
                    },
                    p1::opcodes::OP_LOAD_CONSTANT_32 => {
                        let index = Program::read_u32(&mut iter).expect("Couldn't find operand for OP_LOAD_CONSTANT") as usize;
                        stack.push(program.constant_pool[index].clone());
                    },
                    p1::opcodes::OP_ADD => {
                        let a = stack.pop().expect("Expected value on stack to add!");
                        let b = stack.pop().expect("Expected value on stack to add!");

                        stack.push(
                            match (a,b) {
                                (PureTypes::Pi32(a),PureTypes::Pi32(b)) => PureTypes::Pi32(a + b),
                                (a,b) => panic!("Can't add {:?} and {:?}",a,b),
                            }
                        );
                    },
                    p1::opcodes::OP_PRINT => {
                        let a = stack.pop().expect("Expected value on stack to print!");
                        println!("{:?}",a);
                    },
                    _ => panic!("Unrecognized opcode {}",opcode)
                }
            }
            else {
                break;
            }
        }

        PureVMResults::Success
    }
}