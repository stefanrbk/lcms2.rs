use super::Signature;

pub struct Base {
    pub sig: Signature,
    pub reserved: [i8; 4],
}

pub mod directory;
