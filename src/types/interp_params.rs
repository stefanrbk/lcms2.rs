use crate::{cms::Context, MAX_INPUT_DIMENSIONS, plugin::InterpFunction};

pub struct InterpParams<T> {
    context: Context,
    flags: u32,
    n_outputs: u32,
    n_samples: [u32; MAX_INPUT_DIMENSIONS as usize],
    domain: [u32; MAX_INPUT_DIMENSIONS as usize],
    opta: [u32; MAX_INPUT_DIMENSIONS as usize],
    table: Box<[T]>,
    interpolation: Option<InterpFunction>
}
