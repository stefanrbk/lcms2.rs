use std::{any::Any, mem::size_of};

use crate::{
    io::IoHandler,
    plugin::{read_u16, write_u16, read_u32, write_u32},
    PositionTableEntryFn, Result,
};

use super::TagTypeHandler;

pub fn write_utf16_slice(io: &mut IoHandler, slice: &[u16]) -> Result<()> {
    for n in slice {
        if write_u16(io, *n).is_err() {
            return Err("Error in write_utf16_slice".into());
        }
    }
    Ok(())
}

pub fn read_utf16_slice<'a>(io: &mut IoHandler, slice: &'a mut [u16]) -> Result<&'a [u16]> {
    for n in &mut *slice {
        match read_u16(io) {
            Err(_) => return Err("Error in read_utf16_slice".into()),
            Ok(value) => *n = value,
        }
    }
    Ok(slice)
}

pub fn read_position_table(
    handler: &TagTypeHandler,
    io: &mut IoHandler,
    count: usize,
    base_offset: usize,
    cargo: &dyn Any,
    element_fn: PositionTableEntryFn,
) -> Result<()> {
    let current_pos = (io.tell)(io);

    // Verify there is enough space left to read at least two uint items for count items
    if ((io.reported_size - current_pos) / (2 * size_of::<u32>())) < count {
        return Err("File too short for read in read_position_table".into());
    }

    // Let's take the offsets to each element
    let mut element_offsets = vec![0usize; count];
    let mut element_sizes = vec![0usize; count];

    for i in 0..count {
        element_offsets[i] = match read_u32(io) {
            Ok(value) => value as usize,
            Err(_) => return Err("Read error in read_position_table".into()),
        } + base_offset;
        element_sizes[i] = match read_u32(io) {
            Ok(value) => value as usize,
            Err(_) => return Err("Read error in read_position_table".into()),
        };
    }

    // Seek to each element and read it
    for i in 0..count {
        if !(io.seek)(io, element_offsets[i]) {
            return Err("Seek error in read_position_table".into());
        }

        // This is the reader callback
        if let Err(msg) = element_fn(handler, io, cargo, i, element_sizes[i]) {
            return Err(format!("Reader callback had an error in read_position_table. The error was \"{}\"", msg));
        }
    }

    Ok(())
}

pub fn write_position_table(
    handler: &TagTypeHandler,
    io: &mut IoHandler,
    _size_of_tag: usize,
    count: usize,
    base_offset: usize,
    cargo: &dyn Any,
    element_fn: PositionTableEntryFn,
) -> Result<()> {
    // Create table
    let mut element_offsets = vec![0usize; count];
    let mut element_sizes = vec![0usize; count];

    // Keep starting position of curve offsets
    let directory_pos = (io.tell)(io);

    // Write a fake directory to be filled later on
    for _i in 0..count {
        if write_u32(io, 0u32).is_err()
            || write_u32(io, 0u32).is_err() {
            return Err("Write error in write_position_table".into());
        }
    }

    // Write each element. Keep track of the size as well.
    for i in 0..count {
        let before = (io.tell)(io);
        element_offsets[i] = before - base_offset;

        // Callback to write...
        if let Err(msg) = element_fn(handler, io, cargo, i, element_sizes[i]) {
            return Err(format!("Writer callback had an error in write_position_table. The error was \"{}\"", msg));
        }

        // Now the size
        element_sizes[i] = (io.tell)(io) - before;
    }

    // Write the directory
    let current_pos = (io.tell)(io);
    if !(io.seek)(io, directory_pos) {
        return Err("Seek error in write_position_table".into());
    }

    for i in 0..count {
        if write_u32(io, element_offsets[i] as u32).is_err()
            || write_u32(io, element_sizes[i] as u32).is_err() {
            return Err("Write error in write_position_table".into());
        }
    }

    if !(io.seek)(io, current_pos) {
        return Err("Seek error in write_position_table".into());
    }

    Ok(())
}
