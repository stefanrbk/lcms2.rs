use std::{
    io::{ErrorKind, Read, Seek, Write},
    sync::{Arc, RwLock},
};

use log::Level;

use crate::{signal_error, state::ErrorCode, Context, Result, plugin::IMutex};

use super::IoHandler;

pub struct FileMem {
    used_space: usize,
    reported_size: usize,
    context_id: Context,
    block: Arc<RwLock<Box<[u8]>>>,
    pointer: usize,
    physical_file: String,
}

impl FileMem {
    pub fn new(context_id: &Context, block: &Arc<RwLock<Box<[u8]>>>) -> Self {
        FileMem {
            pointer: 0,
            reported_size: 0,
            used_space: 0,
            context_id: context_id.clone(),
            physical_file: String::new(),
            block: block.clone(),
        }
    }
}
impl IoHandler for FileMem {
    fn context_id(&self) -> &Context {
        &self.context_id
    }

    fn used_space(&self) -> usize {
        self.used_space
    }

    fn reported_size(&self) -> usize {
        self.reported_size
    }

    fn physical_file(&self) -> &String {
        &self.physical_file
    }

    fn read(&mut self, buffer: &mut [u8], size: usize, count: usize) -> usize {
        let len = size * count;
        let block_size = self.block.read().unwrap().len();

        if self.pointer + len > block_size {
            let len = block_size - self.pointer;
            signal_error(
                &self.context_id,
                Level::Error,
                ErrorCode::Read,
                &format!(
                    "Read from memory error. Got {} bytes, block should be of {} bytes",
                    len,
                    count * size
                ),
            );
            return 0;
        }

        let ptr = &self.block.read().unwrap()[self.pointer..];
        buffer[..len].copy_from_slice(&ptr[..len]);
        self.pointer += len;

        count
    }

    fn seek(&mut self, offset: usize) -> bool {
        if offset > self.block.read().unwrap().len() {
            signal_error(&self.context_id, Level::Error, ErrorCode::Seek, "Too few data; probably corrupted profile");
            return false;
        }
        self.pointer = offset;
        true
    }

    fn tell(&mut self) -> usize {
        if self.block.read().unwrap().len() == 0 {
            return 0;
        }

        self.pointer
    }

    fn write(&mut self, mut size: usize, buffer: &[u8]) -> bool {
        let block_size = self.block.read().unwrap().len();

        // Housekeeping
        if block_size == 0 {
            return false;
        }

        // Check for available space. Clip.
        if self.pointer + size > block_size {
            size = block_size - self.pointer;
        }

        // Write zero bytes is ok, but does nothing
        if size == 0 {
            return true;
        }

        self.block.write().unwrap()[self.pointer..][..size].copy_from_slice(&buffer[..size]);
        self.pointer += size;

        if self.pointer > self.used_space {
            self.used_space = self.pointer;
        }

        true
    }

    fn close(&mut self) -> bool {
        true
    }
}
pub fn open_io_handler_from_mem_for_reading(context_id: &Context, buffer: &[u8]) -> Arc<dyn IoHandler> {
    let mut block = vec![0u8; buffer.len()];
    block.copy_from_slice(&buffer);

    let mut mem = FileMem::new(&context_id, &Arc::new(RwLock::new(block.into_boxed_slice())));
    mem.reported_size = buffer.len();

    Arc::new(mem)
}
pub fn open_io_handler_from_mem_for_writing(context_id: &Context, buffer: &Arc<RwLock<Box<[u8]>>>) -> Arc<dyn IoHandler> {
    let mem = FileMem::new(&context_id, &buffer);

    Arc::new(mem)
}
