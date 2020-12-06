extern crate purity_lib;
use purity_lib::types::PureType;

pub struct Program {
    pub constant_pool: Vec<Box<dyn PureType>>
}

impl Program {
    pub fn load_from_bytes<'a, T>(bytecode: T) -> Self
        where T: IntoIterator<Item = &'a u8> 
    {
        let mut constant_pool = Vec::new();

        let mut iter = bytecode.into_iter();

        let constant_pool_len = Program::read_u32(&mut iter);

        println!("Constant pool of length {}",constant_pool_len.unwrap());

        Self {
            constant_pool
        }
    }

    fn read_u32<'a, T>(bytecode: &mut T) -> Option<u32>
        where T: Iterator<Item = &'a u8> 
    {
        let mut bytes = [0; 4];
        let mut i = 0;
        while i < 4 {
            if let Some(b) = bytecode.next() {
                bytes[i] = *b;
                i += 1;
            }
            else {
                return None;
            }
        }
        Some(u32::from_be_bytes(bytes))
    }
}