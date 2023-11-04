use std::any::Any;

use crate::{types::{Signature, Dup}, Context};

pub type FreeUserDataFn = fn(context_id: Context, Box<dyn Any>);
pub type DupUserDataFn = fn(contect_id: Context, Box<&dyn Any>) -> Box<dyn Any>;

pub const GUESS_MAX_WORKERS: i32 = -1;

pub struct Base {
    pub magic: Signature,
    pub expected_version: u32,
    pub r#type: Signature,
}

mod formatter;
mod interp;
mod mpe;
mod mutex;
mod optimization;
mod parallel;
mod parametric_curve;
mod rendering_intent;
mod tag;
mod tag_type;
mod transform;
mod functions;

pub use formatter::*;
pub use interp::{InterpFnFactory, Interpolation};
pub use mpe::MultiProcessElement;
pub use mutex::{
    CreateMutexFn, DestroyMutexFn, IMutex, LockMutexFn, Mutex, MutexGuard, UnlockMutexFn,
};
pub use optimization::{Optimization, OptimizationFn};
pub use parallel::Parallelization;
pub use parametric_curve::{ParametricCurve, ParametricCurveEvaluator};
pub use rendering_intent::{IntentFn, RenderingIntent};
pub use tag::{Tag, TagDescriptor};
pub use tag_type::{TagType, TagTypeHandler};
pub use transform::{Transform, TransformFactories};
pub(crate) use functions::*;
