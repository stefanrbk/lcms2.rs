use crate::Context;

use super::Stream;

pub struct IoHandler<'a> {
    pub stream: &'a dyn Stream,
    pub context_id: Context,
    pub used_space: u32,
    pub reported_size: u32,
    pub physical_file: &'a str,
    pub read: fn(iohandler: &mut Self, buffer: &[u8], size: u32, count: u32) -> u32,
    pub seek: fn(iohandler: &mut Self, offset: u32) -> bool,
    pub close: fn(iohandler: &mut Self) -> bool,
    pub tell: fn(iohandler: &mut Self) -> u32,
    pub write: fn(iohandler: &mut Self, size: u32, buffer: &mut [u8]),
}
