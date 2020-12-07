use crate::types::*;

pub mod p1;

pub struct Program {
    pub constant_pool: Vec<PureTypes>,
    pub bytecode: Vec<u8>
}

impl Program {
    pub fn load_from_bytes<'a, T>(bytecode: T) -> Self
        where T: IntoIterator<Item = &'a u8> 
    {
        let mut constant_pool = Vec::new();
        let mut bytes = Vec::new();

        let mut iter = bytecode.into_iter().enumerate();

        let constant_pool_byte_len = Program::read_u32_from_enumeration(&mut iter).expect("Couldn't find length of constant pool!") as usize;
        let constant_pool_len = Program::read_u32_from_enumeration(&mut iter).expect("Expected number of constants") as usize;

        while let Some((index,byte)) = iter.next() {
            if index - 8 < constant_pool_byte_len {
                constant_pool.push(
                    match *byte {
                        PureTypeBytes::Pi32 => PureTypes::from_bytecode(&mut iter,PureTypes::Pi32(0)).expect("Expected i32! Invalid bytecode"),
                        _ => panic!("Unknown type code: {}",byte)
                    }
                );
            }
            else {
                bytes.push(*byte);
            }
        }

        assert!(constant_pool.len() == constant_pool_len, format!("Didn't read all the constants or there weren't enough constants! Found {} constants needed {}",constant_pool.len(),constant_pool_len));

        Self {
            constant_pool,
            bytecode: bytes
        }
    }

    pub fn read_u32_from_enumeration<'a, T>(bytecode: &mut T) -> Option<u32>
        where T: Iterator<Item = (usize,&'a u8)> 
    {
        let mut bytes = [0; 4];
        let mut i = 0;
        while i < 4 {
            if let Some((_,b)) = bytecode.next() {
                bytes[i] = *b;
                i += 1;
            }
            else {
                return None;
            }
        }
        Some(u32::from_be_bytes(bytes))
    }

    pub fn read_u32<'a, T>(bytecode: &mut T) -> Option<u32>
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

    pub fn read_u16_from_enumeration<'a, T>(bytecode: &mut T) -> Option<u16>
        where T: Iterator<Item = (usize,&'a u8)> 
    {
        let mut bytes = [0; 2];
        let mut i = 0;
        while i < 2 {
            if let Some((_,b)) = bytecode.next() {
                bytes[i] = *b;
                i += 1;
            }
            else {
                return None;
            }
        }
        Some(u16::from_be_bytes(bytes))
    }

    pub fn read_u16<'a, T>(bytecode: &mut T) -> Option<u16>
        where T: Iterator<Item = &'a u8> 
    {
        let mut bytes = [0; 2];
        let mut i = 0;
        while i < 2 {
            if let Some(b) = bytecode.next() {
                bytes[i] = *b;
                i += 1;
            }
            else {
                return None;
            }
        }
        Some(u16::from_be_bytes(bytes))
    }
}