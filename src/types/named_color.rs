use crate::{list, Context, MAX_CHANNELS};

pub struct NamedColorEntry {
    pub name: String,
    pub pcs: [u16; 3],
    pub device_colorant: [u16; MAX_CHANNELS as usize],
}

pub struct NamedColor<'ctx> {
    pub context_id: &'ctx Context,
    pub n_colors: usize,
    pub allocated: usize,
    pub colorant_count: usize,
    pub prefix: String,
    pub suffix: String,
    pub list: list::Link<NamedColorEntry>,
}
