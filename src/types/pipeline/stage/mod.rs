pub type StageEvalFn = fn(In: &[f32], Out: &mut [f32], mpe: &Stage);
pub type StageDupElemFn = for<'a, 'b, 'c> fn(mpe: &'c Stage<'a, 'b>) -> Result<Stage<'a, 'b>, String>;
pub type StageFreeElemFn = fn(mpe: Stage);

pub struct Stage<'ctx, 'dat> {
    pub(crate) context_id: &'ctx crate::Context,

    pub(crate) r#type: Signature,
    pub(crate) implements: Signature,

    pub(crate) input_channels: u32,
    pub(crate) output_channels: u32,

    pub(crate) eval_ptr: StageEvalFn,
    pub(crate) dup_elem_ptr: StageDupElemFn,
    pub(crate) free_ptr: StageFreeElemFn,

    pub(crate) data: &'dat dyn Any,

    pub(crate) next: list::Link<Stage<'ctx, 'dat>>,
}

mod tone_curve;
mod matrix;
mod clut;

use std::any::Any;

pub use tone_curve::ToneCurvesData;
pub use matrix::MatrixData;
pub use clut::{CLutData, Tab};

use crate::{types::Signature, list};
