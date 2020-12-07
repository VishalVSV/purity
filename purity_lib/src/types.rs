pub trait PureType {
    fn to_bytecode(&self) -> Vec<u8>;
    fn from_bytecode<'a, T>(bytecode: &mut T,var_type: PureTypes) -> Option<Self>
        where T: Iterator<Item = (usize,&'a u8)>, Self: Sized;
}

#[derive(Debug,Clone)]
pub enum PureTypes {
    Pi32(i32),
    PString(String)
}

pub struct PureTypeBytes;

#[allow(non_upper_case_globals)]
impl PureTypeBytes {
    pub const Pi32: u8 = 0;
    pub const PString: u8 = 1;
}

impl PureType for PureTypes {
    fn to_bytecode(&self) -> Vec<u8> {
        use PureTypes::*;
        match self {
            Pi32(val) => {
                let mut vec = Vec::with_capacity(4);
                vec.extend_from_slice(&val.to_be_bytes());
                vec.push(0);
                vec
            },
            PString(val) => {
                let mut vec = Vec::with_capacity(val.len());
                vec.extend_from_slice(&val.as_bytes());
                vec
            }
        }
    }

    fn from_bytecode<'a, T>(bytecode: &mut T,var_type: PureTypes) -> Option<Self>
        where T: Iterator<Item = (usize,&'a u8)>
    {
        match var_type {
            PureTypes::Pi32(_) => {
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
                Some(PureTypes::Pi32(i32::from_be_bytes(bytes)))
            },
            PureTypes::PString(_) => {
                let mut vec = Vec::new();
                while let Some((_,b)) = bytecode.next() {
                    if *b != 0 {
                        vec.push(*b);
                    }
                    else {
                        break;
                    }
                }
                Some(PureTypes::PString(String::from_utf8_lossy(&vec).to_string()))
            }
        }
    }
}