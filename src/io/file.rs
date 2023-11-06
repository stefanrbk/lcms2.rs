use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write, Result}, sync::Arc,
};

use log::Level;

use crate::{signal_error, state::ErrorCode, Context};

use super::IoHandler;

pub trait FileLike: Read + Write + Seek {}
impl<T> FileLike for T where T: Read + Write + Seek {}

pub struct File {
    used_space: usize,
    reported_size: usize,
    context_id: Context,
    file: Box<dyn FileLike>,
    physical_file: String,
}

impl File {
    pub fn new(context_id: &Context, file: Box<dyn FileLike>, path: &str) -> Self {
        File {
            reported_size: 0,
            used_space: 0,
            context_id: context_id.clone(),
            physical_file: path.to_owned(),
            file,
        }
    }
}
impl IoHandler for File {
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
        let n_read = match self.file.read(&mut buffer[..(size * count)]) {
            Ok(len) => len / size,
            Err(error) => {
                signal_error(
                    &self.context_id,
                    Level::Error,
                    ErrorCode::File,
                    &format!("A read error occured: {}", error),
                );
                return 0;
            }
        };

        if n_read != count {
            signal_error(
                &self.context_id,
                Level::Error,
                ErrorCode::File,
                &format!(
                    "Read error. Got {} bytes, block should be {} bytes",
                    n_read * size,
                    count * size
                ),
            );
            return 0;
        }

        n_read
    }

    fn seek(&mut self, offset: usize) -> bool {
        match self.file.seek(SeekFrom::Start(offset as u64)) {
            Ok(len) => true,
            Err(error) => {
                signal_error(
                    &self.context_id,
                    Level::Error,
                    ErrorCode::Seek,
                    "Seek error; probably corrupted profile",
                );
                false
            }
        }
    }

    fn tell(&mut self) -> usize {
        match self.file.seek(SeekFrom::Current(0)) {
            Ok(pos) => pos as usize,
            Err(error) => {
                signal_error(
                    &self.context_id,
                    Level::Error,
                    ErrorCode::Seek,
                    "Tell error; probably corrupted profile",
                );
                0
            }
        }
    }

    fn write(&mut self, size: usize, buffer: &[u8]) -> bool {
        if size == 0 {
            return true;
        } // We allow to write 0 bytes, but nothing happens

        self.used_space += size;

        match self.file.write_all(&buffer[..size]) {
            Ok(_) => true,
            Err(error) => {
                signal_error(&self.context_id, Level::Error, ErrorCode::Write, &format!("Write error occured: {}", error));
                false
            }
        }
    }

    fn close(&mut self) -> bool {
        true
    }
}

pub fn open_io_handler_from_file(
    context_id: &Context,
    path: &str,
    access: OpenOptions
) -> Result<Arc<dyn IoHandler>> {
    let file = match access.open(path) {
        Ok(f) => f,
        Err(error) => {
            signal_error(&context_id, Level::Error, ErrorCode::File, &format!("File open error: {}", error));
            return Err(error);
        }
    };

    let file = File::new(
        &context_id,
        Box::new(file),
        path
    );

    Ok(Arc::new(file))
}
