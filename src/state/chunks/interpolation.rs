use std::fmt::Debug;

use crate::plugins::{InterpFnFactory, InterpFunction};

#[derive(Copy, Clone)]
pub struct InterpolationPluginChunk {
    pub interpolators: InterpFnFactory,
}

impl InterpolationPluginChunk {
    pub(crate) fn new(interpolators: InterpFnFactory) -> Self {
        Self { interpolators }
    }
}

impl Default for InterpolationPluginChunk {
    fn default() -> Self {
        Self {
            interpolators: default_interpolatior_factory,
        }
    }
}

impl Debug for InterpolationPluginChunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LogErrorChunk")
            .field("handler", &"[Function Pointer]")
            .finish()
    }
}

pub fn default_interpolatior_factory(_num_input_channels: u32, _num_output_channels: u32, _flags: u32) -> InterpFunction {
    InterpFunction::InterpFn16(|_input, _output, _p| {})
}
