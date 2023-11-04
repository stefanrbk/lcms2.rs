use std::{any::Any, mem::size_of};

use crate::{io::IoHandler, Result, plugin::{read_u16f16, write_u16f16}};

use super::TagTypeHandler;

pub fn type_u16_fixed16_read(_handler: &TagTypeHandler, io: &mut IoHandler, n_items: &mut usize, size_of_tag: usize) ->Result<Box<dyn Any>> {
    *n_items = 0;

    let n = size_of_tag / size_of::<u32>();
    let mut array_f64 = vec![0f64; n];

    for i in 0..n {
        array_f64[i] = read_u16f16(io)?;
    }

    *n_items = 1;
    let value = Box::new(array_f64);
    Ok(value)
}

pub fn type_u16_fixed16_write(_handler: &TagTypeHandler, io: &mut IoHandler, ptr: &dyn Any, n_items: usize) -> Result<()> {
    match ptr.downcast_ref::<Vec<f64>>() {
        None => Err("Invalid object to write with type_u16_fixed16_write".into()),
        Some(value) => {
            for i in 0..n_items {
                write_u16f16(io, value[i])?;
            }

            Ok(())
        },
    }
}

type_dup_and_free!(u16_fixed16, Vec<f64>);
