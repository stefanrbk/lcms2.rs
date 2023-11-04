use std::{any::Any, sync::Arc};

use log::Level;

use crate::{
    plugin::{InterpFnFactory, OptimizationFn, ParametricCurve, Tag, TagTypeHandler},
    types::TransformFunc,
    ErrorHandlerLogFunction, MAX_CHANNELS, Context,
};

use super::{Formatters, Intent, MutexFunctions, ErrorCode};

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
    pub(crate) user_data: Box<dyn Any + Sync + Send>,
    pub(crate) error_logger: ErrorHandlerLogFunction,
}

pub fn signal_error(context_id: &Context, level: Level, error_code: Option<ErrorCode>, text: &'static str) {
    (context_id.error_logger)(context_id, level, error_code, text)
}
