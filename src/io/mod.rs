use std::io::{Read, Seek, Write};

mod file;
mod io_handler;
mod mem;
mod null;

pub use io_handler::IoHandler;
pub use mem::{open_io_handler_from_mem_for_reading, open_io_handler_from_mem_for_writing};
pub use null::open_io_handler_from_null;
