use std::{fmt::Debug, sync::Arc};

use crate::{
    plugins::Formatter,
    state::Context,
    types::{NamedColorList, Pipeline, Sequence, Signature, CIEXYZ, MAX_CHANNELS},
};

pub struct Cache {
    cache_in: [u16; MAX_CHANNELS],
    cache_out: [u16; MAX_CHANNELS],
}

#[derive(Copy, Clone)]
pub struct Stride {
    bytes_per_line_in: u32,
    bytes_per_line_out: u32,
    bytes_per_plane_in: u32,
    bytes_per_plane_out: u32,
}

pub type TransformFn = fn(
    context: &mut Context,
    cargo: &Transform,
    input_buffer: &[u8],
    output_buffer: &mut [u8],
    size: usize,
    stride: usize,
) -> Box<[u8]>;
pub type Transform2Fn = fn(
    context: &mut Context,
    cargo: &Transform,
    input_buffer: &[u8],
    output_buffer: &mut [u8],
    pixels_per_line: usize,
    line_count: usize,
    stride: Stride,
) -> Box<[u8]>;
pub type TransformFactory = fn(
    transform: TransformFn,
    user_data: &mut Box<[u8]>,
    lut: &mut Box<Pipeline>,
) -> Option<(Signature, Signature, u32)>;
pub type Transform2Factory = fn(
    transform: Transform2Fn,
    user_data: &mut Box<[u8]>,
    lut: &mut Box<Pipeline>,
) -> Option<(Signature, Signature, u32)>;

#[derive(Clone)]
pub enum TransformFactories {
    Legacy(TransformFactory),
    Modern(Transform2Factory),
}

impl Debug for TransformFactories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Legacy(_arg0) => f.debug_tuple("Legacy").field(&"[Function Ptr]").finish(),
            Self::Modern(_arg0) => f.debug_tuple("Modern").field(&"[Function Ptr]").finish(),
        }
    }
}

pub struct Transform {
    input_format: Signature,
    output_format: Signature,
    transform: Option<Transform2Fn>,
    from_input: Formatter,
    to_output: Formatter,
    cache: Cache,
    lut: Arc<Pipeline>,
    gamut_check: Arc<Pipeline>,
    input_colorant: Arc<NamedColorList>,
    output_colorant: Arc<NamedColorList>,
    entry_color_space: Signature,
    exit_color_space: Signature,
    entry_white_point: CIEXYZ,
    exit_white_point: CIEXYZ,
    sequence: Arc<Sequence>,
    original_flags: u32,
    adaptation_state: f64,
    rendering_intent: Signature,
    user_data: Box<[u8]>,
    old_transform: Option<TransformFn>,
}

pub type TransformCollection = Vec<TransformFactories>;
