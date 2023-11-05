use std::{any::Any, mem::size_of};

use crate::{
    io::IoHandler,
    plugin::{read_signature, read_u32, write_signature, write_u32},
    sig,
    types::{Data, Dup, Signature, MLU},
    Result, NO_COUNTRY, NO_LANGUAGE,
};

use super::TagTypeHandler;

pub fn type_data_read(
    _handler: &TagTypeHandler,
    io: &mut dyn IoHandler,
    n_items: &mut usize,
    size_of_tag: usize,
) -> Result<Box<dyn Any>> {
    *n_items = 0;

    if size_of_tag < size_of::<u32>() {
        return Err("Tag is too small to read in type_data_read".into());
    }

    let len_of_data = size_of_tag - size_of::<u32>();
    if len_of_data > i32::MAX.try_into().unwrap() {
        return Err("Tag is too large to read in type_data_read".into());
    }

    let flags = read_u32(io)?;

    let mut data = vec![0u8; len_of_data];
    if io.read(&mut data, size_of::<u8>(), len_of_data) != len_of_data {
        return Err("Read error in type_data_read".into());
    }

    *n_items = 1;
    Ok(Box::new(Data {
        len: len_of_data,
        flag: flags,
        data: data.as_slice().into(),
    }))
}

pub fn type_data_write(
    _handler: &TagTypeHandler,
    io: &mut dyn IoHandler,
    ptr: &dyn Any,
    _n_items: usize,
) -> Result<()> {
    match ptr.downcast_ref::<Data>() {
        None => Err("Invalid object to write with type_data_write".into()),
        Some(data) => {
            write_u32(io, data.len as u32)?;

            match io.write(data.len, &data.data) {
                false => Err("Write error in type_data_write".into()),
                true => Ok(()),
            }
        }
    }
}

type_dup_and_free!(data, Data);
