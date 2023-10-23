use std::any::TypeId;

use crate::types::InterpParams;

pub struct CLutData<'a, T>
where
    T: 'static,
{
    pub table: Tab<'a>,
    pub params: &'a [InterpParams<T>],
    pub n_entries: u32,
}

pub enum Tab<'a> {
    T(&'a [u16]),
    TFloat(&'a [f32]),
}

impl<'a, T: 'static> CLutData<'a, T> {
    pub fn has_float_values() -> bool {
        TypeId::of::<T>() == TypeId::of::<f32>()
    }
}
