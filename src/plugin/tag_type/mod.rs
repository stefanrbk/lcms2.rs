use std::{any::Any, sync::Arc};
use once_cell::sync::Lazy;
use paste::paste;

use crate::{io::IoHandler, types::Signature, Result};

use super::Base;

pub struct TagTypeHandler {
    pub signature: Signature,
    pub read: fn(
        handler: &TagTypeHandler,
        io: &mut IoHandler,
        n_items: &mut usize,
        size_of_tag: usize,
    ) -> Result<Box<dyn Any>>,
    pub write: fn(
        handler: &TagTypeHandler,
        io: &mut IoHandler,
        ptr: &dyn Any,
        n_items: usize,
    ) -> Result<()>,
    pub dup: fn(handler: &TagTypeHandler, ptr: &dyn Any, n: usize) -> Result<Box<dyn Any>>,
    pub free: fn(handler: &TagTypeHandler, ptr: Box<dyn Any>),
    pub context_id: Arc<crate::Context>,
    pub icc_version: u32,
}

pub struct TagType {
    pub base: Base,
    pub handler: TagTypeHandler,
}

mod functions;
pub(crate) mod xyz;
pub(crate) mod chromaticity;
pub(crate) mod colorant_order_type;
pub(crate) mod s15_fixed16;
pub(crate) mod u16_fixed16;
pub(crate) mod signature;
pub(crate) mod text;

pub(crate) use functions::*;

macro_rules! ReadFn {
    ($x:ident) => {
        paste! {
            $x::[<type_ $x _read>]
        }
    }
}
macro_rules! WriteFn {
    ($x:ident) => {
        paste! {
            $x::[<type_ $x _write>]
        }
    }
}
macro_rules! DupFn {
    ($x:ident) => {
        paste! {
            $x::[<type_ $x _dup>]
        }
    }
}
macro_rules! FreeFn {
    ($x:ident) => {
        paste! {
            $x::[<type_ $x _free>]
        }
    }
}
macro_rules! TypeHandler {
    ($t:path, $x:ident) => {
        TagTypeHandler { signature: $t, read: ReadFn!($x), write: WriteFn!($x), dup: DupFn!($x), free: FreeFn!($x), context_id: crate::DEFAULT_CONTEXT, icc_version: 0 }
    };
}
macro_rules! TypeMpeHandler {
    ($t:path, $x:ident) => {
        TagTypeHandler { signature: $t, read: ReadFn!($x), write: WriteFn!($x), dup: generic_mpe_dup, free: generic_mpe_free, context_id: crate::DEFAULT_CONTEXT, icc_version: 0 }
    };
}
pub static SUPPORTED_TAG_TYPES: Lazy<Vec<TagTypeHandler>> = Lazy::new(|| vec![
    TypeHandler!(crate::sig::types::XYZ, xyz),
    TypeHandler!(crate::sig::types::CHROMATICITY, chromaticity),
    TypeHandler!(crate::sig::types::COLORANT_ORDER, colorant_order_type),
    TypeHandler!(crate::sig::types::S15_FIXED16_ARRAY, s15_fixed16),
    TypeHandler!(crate::sig::types::U16_FIXED16_ARRAY, u16_fixed16),
    TypeHandler!(crate::sig::types::SIGNATURE, signature),
    TypeHandler!(crate::sig::types::TEXT, text),
]) ;
