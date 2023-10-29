use crate::plugin::ParametricCurveEvaluator;

use super::{CurveSegment, InterpParams};

pub struct ToneCurve<'a, 'b, 'c, 'd, 'e> {
    pub(crate) interp_params: InterpParams<u16>,
    pub(crate) n_segments: usize,
    pub(crate) segments: &'a [CurveSegment<'b>],
    pub(crate) seg_interp: &'c [InterpParams<f32>],
    pub(crate) evals: &'d [ParametricCurveEvaluator],
    pub(crate) n_entries: usize,
    pub(crate) table_16: &'e [u16],
}
