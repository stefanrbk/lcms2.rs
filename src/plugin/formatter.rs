use crate::types::Transform;

use super::Base;

pub type Formatter16In =
    for<'a> fn(cargo: Transform, values: &'a mut [u16], buffer: &'a [u8], stride: u32) -> &'a [u8];
pub type FormatterFloatIn =
    for<'a> fn(cargo: Transform, values: &'a mut [f32], buffer: &'a [u8], stride: u32) -> &'a [u8];

pub type Formatter16Out = for<'a> fn(
    cargo: Transform,
    values: &'a [u16],
    buffer: &'a mut [u8],
    stride: u32,
) -> &'a mut [u8];
pub type FormatterFloatOut = for<'a> fn(
    cargo: Transform,
    values: &'a [f32],
    buffer: &'a mut [u8],
    stride: u32,
) -> &'a mut [u8];
pub type FormatterFactoryIn = fn(Type: u32, Flags: u32) -> FormatterIn;
pub type FormatterFactoryOut = fn(Type: u32, Flags: u32) -> FormatterOut;

pub enum FormatterIn {
    F32(Option<FormatterFloatIn>),
    U16(Option<Formatter16In>),
}

pub enum FormatterOut {
    F32(Option<FormatterFloatOut>),
    U16(Option<Formatter16Out>),
}

pub mod pack_flags {
    pub const U16_BITS: u32 = 0;
    pub const FLOAT: u32 = 1;
}

pub struct Formatter {
    pub base: Base,
    pub factory_in: FormatterFactoryIn,
    pub factory_out: FormatterFactoryOut,
}
