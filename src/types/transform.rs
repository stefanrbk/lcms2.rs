use crate::{plugin::FreeUserDataFn, MAX_CHANNELS};
use std::any::Any;

use super::Pipeline;

pub type TransformFn =
    fn(cargo: &Transform, input_buffer: &[u8], output_buffer: &mut [u8], size: u32, stride: u32);
pub type Transform2Fn = fn(
    cargo: &Transform,
    input_buffer: &[u8],
    output_buffer: &mut [u8],
    pixels_per_line: u32,
    line_count: u32,
    stride: Stride,
);
pub type TransformFactory = for<'a, 'b, 'c, 'd> fn(
    lut: &'a Pipeline<'b, 'c, 'd>,
    input_format: u32,
    output_format: u32,
    flags: u32,
) -> Result<
    (
        TransformFn,
        Option<Box<dyn Any>>,
        Option<FreeUserDataFn>,
        &'a Pipeline<'b, 'c, 'd>,
        u32,
        u32,
        u32,
    ),
    &'static str,
>;
pub type Transform2Factory = for<'a, 'b, 'c, 'd> fn(
    lut: &'a Pipeline<'b, 'c, 'd>,
    input_format: u32,
    output_format: u32,
    flags: u32,
) -> Result<
    (
        Transform2Fn,
        Option<Box<dyn Any>>,
        Option<FreeUserDataFn>,
        &'a Pipeline<'b, 'c, 'd>,
        u32,
        u32,
        u32,
    ),
    &'static str,
>;

pub enum TransformFunc {
    Factory(Transform2Factory),
    OldFactory(TransformFactory),
}

pub struct Stride {
    pub bytes_per_line_in: u32,
    pub bytes_per_line_out: u32,
    pub bytes_per_plane_in: u32,
    pub bytes_per_plane_out: u32,
}

pub struct Transform {
    pub(crate) context_id: crate::Context,
}

pub(crate) struct Cache {
    pub r#in: [u16; MAX_CHANNELS as usize],
    pub out: [u16; MAX_CHANNELS as usize],
}
