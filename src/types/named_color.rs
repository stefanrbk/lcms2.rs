use crate::{list, Context, MAX_CHANNELS};

pub struct NamedColorEntry<'a> {
    pub name: &'a str,
    pub pcs: [u16; 3],
    pub device_colorant: [u16; MAX_CHANNELS as usize],
}

pub struct NamedColor<'ctx, 'a, 'b, 'c> {
    pub context_id: &'ctx Context,
    pub n_colors: usize,
    pub allocated: usize,
    pub colorant_count: usize,
    pub prefix: &'a str,
    pub suffix: &'b str,
    pub list: list::Link<NamedColorEntry<'c>>,
}
