use std::io::{Read, Seek, Write};

mod io_handler;
mod null;

pub use io_handler::IoHandler;
pub use null::open_io_handler_from_null;
