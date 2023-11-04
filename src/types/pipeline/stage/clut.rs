use std::any::TypeId;

use crate::types::InterpParams;

pub struct CLutData<T>
where
    T: Copy + 'static,
{
    pub table: Tab,
    pub params: Box<[InterpParams<T>]>,
    pub n_entries: u32,
}

pub enum Tab {
    T(Box<[u16]>),
    TFloat(Box<[f32]>),
}

impl<T: Copy + 'static> CLutData<T> {
    pub fn has_float_values() -> bool {
        TypeId::of::<T>() == TypeId::of::<f32>()
    }
}
