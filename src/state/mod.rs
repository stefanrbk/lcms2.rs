use std::any::Any;

use crate::{
    plugin::{
        CreateMutexFn, DestroyMutexFn, FormatterFactoryIn, FormatterFactoryOut, IntentFn,
        InterpFnFactory, LockMutexFn, OptimizationFn, ParametricCurveEvaluator, TagDescriptor,
        TagTypeHandler, UnlockMutexFn,
    },
    types::{Signature, Transform2Fn, TransformFunc},
    ErrorHandlerLogFunction, MAX_CHANNELS,
};

pub(crate) mod context;
mod error;
pub mod plugin;

pub use context::ContextStruct;
pub use error::{default_error_handler_log_function, ErrorCode};

pub struct Tag {
    pub signature: Signature,
    pub descriptor: TagDescriptor,
}

pub struct Formatters {
    pub r#in: Vec<FormatterFactoryIn>,
    pub out: Vec<FormatterFactoryOut>,
}

pub struct ParametricCurve {
    pub functions: Vec<(i32, u32)>,
    pub eval: ParametricCurveEvaluator,
}

pub struct Intent {
    pub value: u32,
    pub description: &'static str,
    pub func: IntentFn,
}

pub struct MutexFunctions {
    pub create: Option<CreateMutexFn>,
    pub destroy: Option<DestroyMutexFn>,
    pub lock: Option<LockMutexFn>,
    pub unlock: Option<UnlockMutexFn>,
}

pub struct Parallelization {
    pub max_workers: i32,
    pub worker_flags: i32,
    pub scheduler: Option<Transform2Fn>,
}
