use std::any::Any;

use crate::{io::IoHandler, types::Signature};

use super::Base;

pub struct TagTypeHandler {
    pub signature: Signature,
    pub read: fn(
        handler: &TagTypeHandler,
        io: &IoHandler,
        n_items: &mut u32,
        size_of_tag: u32,
    ) -> Result<Box<dyn Any>, &'static str>,
    pub write:
        fn(handler: &TagTypeHandler, io: &IoHandler, ptr: Box<dyn Any>, n_items: u32) -> bool,
    pub dup: fn(handler: &TagTypeHandler, ptr: &Box<dyn Any>, n: u32) -> Result<Box<dyn Any>, &'static str>,
    pub free: fn(handler: &TagTypeHandler, ptr: Box<dyn Any>),
    pub context_id: crate::Context,
    pub icc_version: u32,
}

pub struct TagType {
    pub base: Base,
    pub handler: TagTypeHandler,
}
