use crate::types::{Transform2Factory, TransformFactory};

use super::Base;

pub struct Transform {
    pub base: Base,
    pub factory: TransformFactories,
}

pub enum TransformFactories {
    LegacyTransform(TransformFactory),
    Transform(Transform2Factory),
}
