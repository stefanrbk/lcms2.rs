use std::io::{Read, Seek, Write};

mod io_handler;

pub use io_handler::IoHandler;

pub trait Stream: Read + Write + Seek {}
impl<T> Stream for T where T: Read + Write + Seek {}
