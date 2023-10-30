use std::{any::Any, sync::Arc};

/// Maximum number of channels in ICC profiles
pub const MAX_CHANNELS: u32 = 16;

/// Use this flag to prevent changes being written to destination
pub const SAMPLER_INSPECT: u32 = 0x01000000;

pub const MAX_INPUT_DIMENSIONS: u32 = 15;

pub const NO_LANGUAGE: [u8; 2] = [0, 0];
pub const NO_COUNTRY: [u8; 2] = [0, 0];

pub const MAX_TYPES_IN_PLUGIN: u32 = 20;

const PI: f64 = 3.14159265358979323846;
const LOG10E: f64 = 0.434294481903251827651;

pub const DEFAULT_CONTEXT: Arc<Context> = Arc::new(Context {
    alarm_codes: todo!(),
    adaptation_state: todo!(),
    interpolator: todo!(),
    curves: todo!(),
    formatters: todo!(),
    tag_types: todo!(),
    mpe_types: todo!(),
    tags: todo!(),
    intents: todo!(),
    optimizations: todo!(),
    transforms: todo!(),
    mutex: todo!(),
    user_data: todo!(),
});

#[allow(non_camel_case_types)]
pub type s15f16 = i32;
#[allow(non_camel_case_types)]
pub type u16f16 = u32;
#[allow(non_camel_case_types)]
pub type u8f8 = u16;

pub type Result<T> = core::result::Result<T, String>;

pub type Sampler16 = fn(In: &[u16], Out: &mut [u16], Cargo: &mut dyn Any) -> bool;
pub type SamplerFloat = fn(In: &[f32], Out: &mut [f32], Cargo: &mut dyn Any) -> bool;
pub(crate) type PositionTableEntryFn = fn(
    handler: &TagTypeHandler,
    io: &mut IoHandler,
    cargo: &dyn Any,
    n: usize,
    size_of_tag: usize,
) -> Result<()>;

use io::IoHandler;
use plugin::TagTypeHandler;
pub use state::ContextStruct as Context;

mod consts;
mod inlines;
pub(crate) use consts::*;
pub(crate) use inlines::*;

//pub mod cms;
pub mod io;
pub mod list;
pub mod plugin;
pub mod sig;
pub mod state;
pub mod types;

pub mod device_attribute {
    pub const REFLECTIVE: u32 = 0;
    pub const TRANSPARENCY: u32 = 1;
    pub const GLOSSY: u32 = 0;
    pub const MATTE: u32 = 2;
}

pub mod illuminant_type {
    pub const UNKNOWN: u32 = 0x0000000;
    pub const D50: u32 = 0x0000001;
    pub const D65: u32 = 0x0000002;
    pub const D93: u32 = 0x0000003;
    pub const F2: u32 = 0x0000004;
    pub const D55: u32 = 0x0000005;
    pub const A: u32 = 0x0000006;
    pub const E: u32 = 0x0000007;
    pub const F8: u32 = 0x0000008;
}

pub mod screening_frequence {
    pub const PRINTER_DEFAULT: u32 = 1;
    pub const UNITS_LINES_CM: u32 = 0;
    pub const UNITS_LINES_INCH: u32 = 2;
}

pub mod screening_spot_shape {
    pub const UNKNOWN: u32 = 0;
    pub const PRINTER_DEFAULT: u32 = 1;
    pub const ROUND: u32 = 2;
    pub const DIAMOND: u32 = 3;
    pub const ELLIPSE: u32 = 4;
    pub const LINE: u32 = 5;
    pub const SQUARE: u32 = 6;
    pub const CROSS: u32 = 7;
}

pub mod intent {
    pub const PERCEPTUAL: u32 = 0;
    pub const RELATIVE_COLORIMETRIC: u32 = 1;
    pub const SATURATION: u32 = 2;
    pub const ABSOLUTE_COLORIMETRIC: u32 = 3;

    // // Non-ICC intents
    pub const PRESERVE_K_ONLY_PERCEPTUAL: u32 = 10;
    pub const PRESERVE_K_ONLY_RELATIVE_COLORIMETRIC: u32 = 11;
    pub const PRESERVE_K_ONLY_SATURATION: u32 = 12;
    pub const PRESERVE_K_PLANE_PERCEPTUAL: u32 = 13;
    pub const PRESERVE_K_PLANE_RELATIVE_COLORIMETRIC: u32 = 14;
    pub const PRESERVE_K_PLANE_SATURATION: u32 = 15;
}

pub mod flags {
    pub const FLAGS_NOCACHE: u32 = 0x0040;
    pub const FLAGS_NOOPTIMIZE: u32 = 0x0100;
    pub const FLAGS_NULLTRANSFORM: u32 = 0x0200;

    // // Proofing flags
    pub const FLAGS_GAMUTCHECK: u32 = 0x1000;
    pub const FLAGS_SOFTPROOFING: u32 = 0x4000;

    // // Misc
    pub const FLAGS_BLACKPOINTCOMPENSATION: u32 = 0x2000;
    pub const FLAGS_NOWHITEONWHITEFIXUP: u32 = 0x0004;
    pub const FLAGS_HIGHRESPRECALC: u32 = 0x0400;
    pub const FLAGS_LOWRESPRECALC: u32 = 0x0800;

    // // For devicelink creation
    pub const FLAGS_8BITS_DEVICELINK: u32 = 0x0008;
    pub const FLAGS_GUESSDEVICECLASS: u32 = 0x0020;
    pub const FLAGS_KEEP_SEQUENCE: u32 = 0x0080;

    // // Specific to a particular optimizations
    pub const FLAGS_FORCE_CLUT: u32 = 0x0002;
    pub const FLAGS_CLUT_POST_LINEARIZATION: u32 = 0x0001;
    pub const FLAGS_CLUT_PRE_LINEARIZATION: u32 = 0x0010;

    // // Specific to unbounded mode
    pub const FLAGS_NONEGATIVES: u32 = 0x8000;

    // // Copy alpha channels when transforming
    pub const FLAGS_COPY_ALPHA: u32 = 0x04000000;

    // // Fine-tune control over number of gridpoints
    #[macro_export]
    macro_rules! GRIDPOINTS {
        ($n:ident) => {
            (($n) & 0xFF) << 16
        };
    }
    pub use GRIDPOINTS;

    // // CRD special
    pub const FLAGS_NODEFAULTRESOURCEDEF: u32 = 0x01000000;
}

pub enum PostScriptResourceType {
    ColorRenderingDictionary,
    ColorSpaceArray,
}
