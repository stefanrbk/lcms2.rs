use crate::{
    plugin::{Formatter16In, Formatter16Out, FormatterFloatIn, FormatterFloatOut, FreeUserDataFn},
    MAX_CHANNELS,
};
use std::any::Any;

use super::{Format, NamedColor, Pipeline, Signature, XYZ, Seq};

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
        Box<Pipeline<'b, 'c, 'd>>,
        u32,
        u32,
        u32,
    ),
    String,
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
        Box<Pipeline<'b, 'c, 'd>>,
        u32,
        u32,
        u32,
    ),
    String,
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

pub struct Transform<'ctx, 'dat, 'a> {
    pub(crate) input_format: Format,
    pub(crate) output_format: Format,

    pub(crate) xform: Transform2Fn,

    pub(crate) from_input: Formatter16In,
    pub(crate) to_output: Formatter16Out,

    pub(crate) from_input_float: FormatterFloatIn,
    pub(crate) to_output_float: FormatterFloatOut,

    pub(crate) cache: Cache,

    pub(crate) lut: Box<Pipeline<'ctx, 'dat, 'a>>,

    pub(crate) gamut_check: Box<Pipeline<'ctx, 'dat, 'a>>,

    pub(crate) input_colorant: NamedColor<'ctx>,
    pub(crate) output_colorant: NamedColor<'ctx>,

    pub(crate) entry_color_space: Signature,
    pub(crate) exit_color_space: Signature,

    pub(crate) entry_white_point: XYZ,
    pub(crate) exit_white_point: XYZ,

    pub(crate) sequence: Seq<'ctx, 'a>,

    pub(crate) original_flags: u32,
    pub(crate) adaptation_stage: f64,

    pub(crate) rendering_intent: u32,

    pub(crate) context_id: &'ctx crate::Context,

    pub(crate) user_data: Option<Box<dyn Any>>,
    pub(crate) free_user_data: FreeUserDataFn,

    pub(crate) old_xform: Option<TransformFn>,

    pub(crate) worker: Option<Transform2Fn>,
    pub(crate) max_workers: i32,
    pub(crate) worker_flags: u32,
}

pub(crate) struct Cache {
    pub r#in: [u16; MAX_CHANNELS as usize],
    pub out: [u16; MAX_CHANNELS as usize],
}
