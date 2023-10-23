use std::any::Any;

use crate::{types::Signature, MAX_TYPES_IN_PLUGIN};

use super::Base;

pub struct TagDescriptor {
    pub elem_count: u32,
    pub n_supported_types: u32,
    pub supported_types: [Signature; MAX_TYPES_IN_PLUGIN as usize],
    pub decide_type: Option<fn(icc_version: f64, data: &Box<dyn Any>) -> Signature>,
}

pub struct Tag {
    pub base: Base,
    pub signature: Signature,
    pub descriptor: TagDescriptor,
}
