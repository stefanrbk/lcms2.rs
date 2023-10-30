use crate::Context;

use super::Stream;

pub struct IoHandler {
    pub stream: Box<dyn Stream>,
    pub context_id: &'static Context,
    pub used_space: usize,
    pub reported_size: usize,
    pub physical_file: String,
    pub read: fn(iohandler: &mut Self, buffer: &mut [u8], size: usize, count: usize) -> usize,
    pub seek: fn(iohandler: &mut Self, offset: usize) -> bool,
    pub close: fn(iohandler: &mut Self) -> bool,
    pub tell: fn(iohandler: &mut Self) -> usize,
    pub write: fn(iohandler: &mut Self, size: usize, buffer: &[u8]) -> bool,
}
