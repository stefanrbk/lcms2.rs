pub type Eval16Fn = fn(In: &[u16], Out: &mut [u16], data: &Box<dyn Any>);
pub type EvalFloatFn = fn(In: &[f32], Out: &mut [f32], data: &Box<dyn Any>);

pub struct Pipeline {
    pub context_id: crate::Context,
}
mod stage;

use std::any::Any;

pub use stage::{Stage, StageDupElemFn, StageEvalFn, StageFreeElemFn};
pub use stage::CLutData as CLutStageData;
pub use stage::Tab as CLutStageDataTab;
pub use stage::MatrixData as MatrixStageData;
pub use stage::ToneCurvesData as ToneCurvesStageData;
