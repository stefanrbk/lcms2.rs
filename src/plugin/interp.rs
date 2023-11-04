use crate::{plugin, Result};
use crate::types::InterpFunction;

pub type InterpFnFactory = fn(nInputChannels: usize, nOutputChannels: usize, flags: u32) -> Result<InterpFunction>;

pub struct Interpolation {
    pub base: plugin::Base,
    pub interpolators_factory: InterpFnFactory,
}
