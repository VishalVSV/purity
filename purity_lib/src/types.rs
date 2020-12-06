pub trait PureType {
    fn to_bytecode(&self) -> Vec<u8>;
    fn type_name(&self) -> &'static str;
}

pub enum PureTypes {
    Pi32(Pi32),

}

pub struct Pi32 {
    pub val: i32
}

impl PureType for Pi32 {
    fn to_bytecode(&self) -> Vec<u8> {
        let mut vec = Vec::with_capacity(4);
        vec.extend_from_slice(&self.val.to_be_bytes());
        vec
    }

    fn type_name(&self) -> &'static str {
        "i32"
    }
}