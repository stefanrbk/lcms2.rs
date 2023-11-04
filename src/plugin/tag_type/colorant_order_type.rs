use std::{any::Any, mem::size_of};

use crate::{
    io::IoHandler,
    plugin::{read_u32, write_u32},
    Result, MAX_CHANNELS,
};

use super::TagTypeHandler;

pub fn type_colorant_order_type_read(
    _handler: &TagTypeHandler,
    io: &mut IoHandler,
    n_items: &mut usize,
    _size_of_tag: usize,
) -> Result<Box<dyn Any>> {
    *n_items = 0;

    let count = read_u32(io)? as usize;

    if count > MAX_CHANNELS {
        return Err("Channel count out of range in type_colorant_order_type_read".into());
    }

    // We use FF as end marker
    let mut colorant_order = [0xFFu8; MAX_CHANNELS];

    if (io.read)(io, &mut colorant_order, size_of::<u8>(), count) != count {
        return Err("Read error in type_colorant_order_type_read".into());
    }

    *n_items = 1;
    Ok(Box::new(colorant_order))
}

pub fn type_colorant_order_type_write(
    _handler: &TagTypeHandler,
    io: &mut IoHandler,
    ptr: &dyn Any,
    _n_items: usize,
) -> Result<()> {
    match ptr.downcast_ref::<[u8; 16]>() {
        None => Err("Invalid object to write with type_colorant_order_type_write".into()),
        Some(colorant_order) => {
            let mut count = 0usize;

            for i in 0..(MAX_CHANNELS as usize) {
                if colorant_order[i] != 0xFFu8 {
                    count += 1;
                }
            }
            write_u32(io, count as u32)?;

            let sz = count * size_of::<u8>();
            if !(io.write)(io, sz, colorant_order) {
                return Err("Write error in type_colorant_order_type_write".into());
            }

            Ok(())
        }
    }
}

type_dup_and_free!(colorant_order_type, [u8; 16]);
