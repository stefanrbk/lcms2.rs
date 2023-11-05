pub type Eval16Fn = fn(In: &[u16], Out: &mut [u16], data: &Box<dyn Any>);
pub type EvalFloatFn = fn(In: &[f32], Out: &mut [f32], data: &Box<dyn Any>);

pub struct Pipeline {
    pub(crate) elements: list::List<Stage>,
    pub(crate) context_id: Arc<crate::Context>,
    pub(crate) input_channels: u32,
    pub(crate) output_channels: u32,

    pub(crate) data: Box<dyn Any>,

    pub(crate) eval_16_fn: Eval16Fn,
    pub(crate) eval_float_fn: EvalFloatFn,
    pub(crate) free_data_fn: FreeUserDataFn,
    pub(crate) dup_data_fn: DupUserDataFn,

    pub(crate) save_as_8_bits: bool,
}
mod stage;

use std::any::Any;
use std::sync::Arc;

pub use stage::CLutData as CLutStageData;
pub use stage::MatrixData as MatrixStageData;
pub use stage::Tab as CLutStageDataTab;
pub use stage::ToneCurvesData as ToneCurvesStageData;
pub use stage::{Stage, StageDupElemFn, StageEvalFn, StageFreeElemFn};

use crate::list;
use crate::plugin::{DupUserDataFn, FreeUserDataFn};
