use crate::plugin;
use crate::types::InterpFunction;

pub type InterpFnFactory = fn(nInputChannels: u32, nOutputChannels: u32, flags: u32) -> InterpFunction;

pub struct Interpolation {
    pub base: plugin::Base,
    pub interpolators_factory: InterpFnFactory,
}
