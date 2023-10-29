use std::mem::size_of;

pub const PTR_ALIGNMENT: usize = size_of::<usize>();

pub const MAX_ENCODEABLE_XYZ: f64 = 1.0 + 32767.0 / 32768.0;
pub const MIN_ENCODEABLE_AB2: f64 = -128.0;
pub const MAX_ENCODEABLE_AB2: f64 = (65535.0 / 256.0) - 128.0;
pub const MIN_ENCODEABLE_AB4: f64 = -128.0;
pub const MAX_ENCODEABLE_AB4: f64 = 127.0;

pub const MAX_STAGE_CHANNELS: u32 = 128;

pub const MATRIX_DET_TOLERANCE: f64 = 1e-4;

pub(crate) const MAX_TABLE_TAG: usize = 100;
