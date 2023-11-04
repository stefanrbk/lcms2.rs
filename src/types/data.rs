pub struct Data {
    pub len: usize,
    pub flag: u32,
    pub data: Box<[u8]>,
}

impl Clone for Data {
    fn clone(&self) -> Self {
        let mut data = vec![0u8; self.len];
        data.clone_from(&self.data.as_ref().to_vec());
        Self {
            len: self.len,
            flag: self.flag,
            data: self.data.clone(),
        }
    }
}
