use std::any::Any;

use crate::{MAX_CHANNELS, plugin::{InterpFnFactory, ParametricCurve, FormatterFactoryIn, FormatterFactoryOut, TagTypeHandler, TagDescriptor, IntentFn, OptimizationFn, CreateMutexFn, DestroyMutexFn, LockMutexFn, UnlockMutexFn}, types::{Signature, TransformFunc, Transform2Fn}};

pub struct ContextStruct {
    pub(crate) alarm_codes: [u16; MAX_CHANNELS as usize],
    pub(crate) adaptation_state: f64,
    pub(crate) interpolator: InterpFnFactory,
    pub(crate) curves: Vec<ParametricCurve>,
    pub(crate) formatters: Formatters,
    pub(crate) tag_types: Vec<TagTypeHandler>,
    pub(crate) mpe_types: Vec<TagTypeHandler>,
    pub(crate) tags: Vec<Tag>,
    pub(crate) intents: Vec<Intent>,
    pub(crate) optimizations: Vec<OptimizationFn>,
    pub(crate) transforms: Vec<TransformFunc>,
    pub(crate) mutex: MutexFunctions,
    pub(crate) user_data: Box<dyn Any>,
}

pub struct Tag {
    pub signature: Signature,
    pub descriptor: TagDescriptor,
}

pub struct Formatters {
    pub r#in: Vec<FormatterFactoryIn>,
    pub out: Vec<FormatterFactoryOut>,
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

pub mod plugin;
