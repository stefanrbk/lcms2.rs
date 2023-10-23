use crate::types::{Format, Pipeline};

use super::Base;

pub type OptimizationFn = fn(
    lut: &mut Pipeline,
    intent: u32,
    input_format: &mut Format,
    output_format: &mut Format,
    flags: &mut u32,
) -> bool;

pub struct Optimization {
    pub base: Base,
    pub optimize_ptr: OptimizationFn,
}
