use once_cell::sync::Lazy;
use paste::paste;
use std::{any::Any, sync::Arc};

use crate::{io::IoHandler, sig, types::Signature, Result};

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

macro_rules! type_dup_and_free {
    ($tag_type:ident, $type:ty) => {
        paste::paste! {
            pub fn [<type_ $tag_type _dup>](
                _handler: &TagTypeHandler,
                ptr: &dyn Any,
                _n_items: usize,
            ) -> Result<Box<dyn Any>> {
                match ptr.downcast_ref::<$type>() {
                    None => Err("Invalid object to duplicate with [<type_ $tag_type _dup>]".into()),
                    Some(value) => Ok(Box::new(value.clone())),
                }
            }

            pub fn [<type_ $tag_type _free>](_handler: &TagTypeHandler, ptr: Box<dyn Any>) {
                drop(ptr);
            }
        }
    };
}

pub(crate) mod chromaticity;
pub(crate) mod colorant_order_type;
pub(crate) mod data;
mod functions;
pub(crate) mod s15_fixed16;
pub(crate) mod signature;
pub(crate) mod text;
pub(crate) mod text_description;
pub(crate) mod u16_fixed16;
pub(crate) mod xyz;

use chromaticity::*;
use colorant_order_type::*;
use data::*;
pub(crate) use functions::*;
use s15_fixed16::*;
use signature::*;
use text::*;
use text_description::*;
use u16_fixed16::*;
use xyz::*;

macro_rules! TypeHandler {
    ($t:path, $x:ident) => {
        paste::paste! {
            TagTypeHandler {
                signature: $t,
                read: [<type_ $x _read>],
                write: [<type_ $x _write>],
                dup: [<type_ $x _dup>],
                free: [<type_ $x _free>],
                context_id: crate::DEFAULT_CONTEXT,
                icc_version: 0,
            }
        }
    };
}
macro_rules! TypeMpeHandler {
    ($t:ty, $x:ident) => {
        paste::paste! {
            TagTypeHandler {
                signature: $t,
                read: [<type_ $x _read>],
                write: [<type_ $x _write>],
                dup: generic_mpe_dup,
                free: generic_mpe_free,
                context_id: crate::DEFAULT_CONTEXT,
                icc_version: 0,
            }
        }
    };
}

pub static SUPPORTED_TAG_TYPES: Lazy<Vec<TagTypeHandler>> = Lazy::new(|| {
    vec![
        TypeHandler!(sig::types::XYZ, xyz),
        TypeHandler!(sig::types::CHROMATICITY, chromaticity),
        TypeHandler!(sig::types::COLORANT_ORDER, colorant_order_type),
        TypeHandler!(sig::types::S15_FIXED16_ARRAY, s15_fixed16),
        TypeHandler!(sig::types::U16_FIXED16_ARRAY, u16_fixed16),
        TypeHandler!(sig::types::SIGNATURE, signature),
        TypeHandler!(sig::types::TEXT, text),
        TypeHandler!(sig::types::DATA, data),
        TypeHandler!(sig::types::TEXT_DESCRIPTION, text_description),
    ]
});
