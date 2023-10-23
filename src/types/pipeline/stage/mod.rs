pub type StageEvalFn = fn(In: &[f32], Out: &mut [f32], mpe: &Stage);
pub type StageDupElemFn = fn(mpe: &Stage) -> Result<Stage, &'static str>;
pub type StageFreeElemFn = fn(mpe: Stage);

pub struct Stage {
    pub context_id: crate::Context,
}

mod tone_curve;
mod matrix;
mod clut;

pub use tone_curve::ToneCurvesData;
pub use matrix::MatrixData;
pub use clut::{CLutData, Tab};
