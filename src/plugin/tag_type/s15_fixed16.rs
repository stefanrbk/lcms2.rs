use std::{any::Any, mem::size_of};

use crate::{io::IoHandler, Result, plugin::{read_s15f16, write_s15f16}};

use super::TagTypeHandler;

pub fn type_s15_fixed16_read(_handler: &TagTypeHandler, io: &mut IoHandler, n_items: &mut usize, size_of_tag: usize) ->Result<Box<dyn Any>> {
    *n_items = 0;

    let n = size_of_tag / size_of::<u32>();
    let mut array_f64 = vec![0f64; n];

    for i in 0..n {
        array_f64[i] = read_s15f16(io)?;
    }

    *n_items = 1;
    let value = Box::new(array_f64);
    Ok(value)
}

pub fn type_s15_fixed16_write(_handler: &TagTypeHandler, io: &mut IoHandler, ptr: &dyn Any, n_items: usize) -> Result<()> {
    match ptr.downcast_ref::<Vec<f64>>() {
        None => Err("Invalid object to write with type_s15_fixed16_write".into()),
        Some(value) => {
            for i in 0..n_items {
                write_s15f16(io, value[i])?;
            }

            Ok(())
        },
    }
}

pub fn type_s15_fixed16_dup(_handler: &TagTypeHandler, ptr: &dyn Any, _n_items: usize) -> Result<Box<dyn Any>> {
    match ptr.downcast_ref::<Vec<f64>>() {
        None => Err("Invalid object to duplicate with type_s15_fixed16_dup".into()),
        Some(value) => Ok(Box::new(value.clone())),
    }
}

pub fn type_s15_fixed16_free(_handler: &TagTypeHandler, ptr: Box<dyn Any>) {
    drop(ptr);
}
