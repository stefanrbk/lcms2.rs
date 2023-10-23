use crate::{plugin, MAX_TYPES_IN_PLUGIN};

pub type ParametricCurveEvaluator = fn(Type: i32, Params: [f64; 10], R: f64) -> f64;

pub struct ParametricCurve {
    pub base: plugin::Base,
    pub n_functions: u32,
    pub function_types: [u32; MAX_TYPES_IN_PLUGIN as usize],
    pub parameter_count: [u32; MAX_TYPES_IN_PLUGIN as usize],
    pub evaluator: ParametricCurveEvaluator,
}
