use std::{
    fs::OpenOptions,
    io::{Read, Seek, SeekFrom, Write, Result}, sync::{Arc, RwLock},
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
    file: Arc<RwLock<Box<dyn FileLike>>>,
    physical_file: String,
}

impl File {
    pub fn new(context_id: &Context, file: Arc<RwLock<Box<dyn FileLike>>>, path: &str) -> Self {
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
        let n_read = match self.file.write().unwrap().read(&mut buffer[..(size * count)]) {
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
        self.file.write().unwrap().seek(SeekFrom::Start(offset as u64)).is_ok()
    }

    fn tell(&mut self) -> usize {
        match self.file.write().unwrap().seek(SeekFrom::Current(0)) {
            Ok(pos) => pos as usize,
            Err(_) => {
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

        match self.file.write().unwrap().write_all(&buffer[..size]) {
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

pub fn open_io_handler_from_file_for_reading(
    context_id: &Context,
    path: &str,
    access: OpenOptions
) -> Result<Arc<dyn IoHandler>> {
    let mut file = match access.clone().read(true).open(path) {
        Ok(f) => f,
        Err(error) => {
            signal_error(&context_id, Level::Error, ErrorCode::File, &format!("File open error: {}", error));
            return Err(error);
        }
    };

    let file_len = file_length(&mut file)?;

    let mut file = File::new(
        &context_id,
        Arc::new(RwLock::new(Box::new(file))),
        path
    );
    file.reported_size = file_len;

    Ok(Arc::new(file))
}

pub fn open_io_handler_from_file_for_writing(
    context_id: &Context,
    path: &str,
    access: OpenOptions
) -> Result<Arc<dyn IoHandler>> {
    let file = match access.clone().write(true).open(path) {
        Ok(f) => f,
        Err(error) => {
            signal_error(&context_id, Level::Error, ErrorCode::File, &format!("File open error: {}", error));
            return Err(error);
        }
    };

    let file = File::new(
        &context_id,
        Arc::new(RwLock::new(Box::new(file))),
        path
    );

    Ok(Arc::new(file))
}

pub fn open_io_handler_from_stream(context_id: &Context, stream: &Arc<RwLock<Box<dyn FileLike>>>) -> Result<Arc<dyn IoHandler>> {
    let mut file = stream.write().unwrap();
    let file = file.as_mut();
    let file_size = file_length(file)?;

    let mut file = File::new(
        &context_id,
        stream.clone(),
        ""
    );
    file.reported_size = file_size;

    Ok(Arc::new(file))
}

fn file_length(file: &mut dyn FileLike) -> Result<usize> {
    let p = file.seek(SeekFrom::Current(0))?;

    let n = file.seek(SeekFrom::End(0))?;

    file.seek(SeekFrom::Start(p))?;

    Ok(n as usize)
}
