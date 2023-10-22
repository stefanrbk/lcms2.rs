use crate::types::Signature;

pub struct Entry {
    pub sig: Signature,
    pub offset: u32,
    pub size: u32,
}
