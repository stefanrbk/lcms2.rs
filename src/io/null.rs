use std::{
    io::{ErrorKind, Read, Seek, Write},
    sync::Arc,
};

use crate::{Context, Result};

use super::IoHandler;

pub struct FileNull {
    used_space: usize,
    context_id: Context,
    pointer: usize,
    physical_file: String,
}

impl FileNull {
    pub fn new(context_id: &Context) -> Self {
        FileNull {
            pointer: 0,
            used_space: 0,
            context_id: context_id.clone(),
            physical_file: String::new(),
        }
    }
}
impl IoHandler for FileNull {
    fn context_id(&self) -> &Context {
        &self.context_id
    }

    fn used_space(&self) -> usize {
        self.used_space
    }

    fn reported_size(&self) -> usize {
        0
    }

    fn physical_file(&self) -> &String {
        &self.physical_file
    }

    fn read(&mut self, _buffer: &mut [u8], size: usize, count: usize) -> usize {
        let len = size * count;
        self.pointer += len;
        count
    }

    fn seek(&mut self, offset: usize) -> bool {
        self.pointer = offset;
        true
    }

    fn close(&mut self) -> bool {
        true
    }

    fn tell(&mut self) -> usize {
        self.pointer
    }

    fn write(&mut self, size: usize, _buffer: &[u8]) -> bool {
        self.pointer += size;
        if self.pointer > self.used_space {
            self.used_space = self.pointer;
        }

        true
    }
}
pub fn open_io_handler_from_null(context_id: &Context) -> Arc<dyn IoHandler> {
    Arc::new(FileNull::new(&context_id))
}
