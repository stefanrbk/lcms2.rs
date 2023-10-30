use crate::plugin::ParametricCurveEvaluator;

use super::{CurveSegment, InterpParams};

pub struct ToneCurve<'a> {
    pub(crate) interp_params: InterpParams<u16>,
    pub(crate) n_segments: usize,
    pub(crate) segments: &'a [CurveSegment<'a>],
    pub(crate) seg_interp: &'a [InterpParams<f32>],
    pub(crate) evals: &'a [ParametricCurveEvaluator],
    pub(crate) n_entries: usize,
    pub(crate) table_16: &'a [u16],
}
