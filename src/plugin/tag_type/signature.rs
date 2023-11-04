use std::any::Any;

use crate::{io::IoHandler, Result, plugin::{read_signature, write_signature}, types::Signature};

use super::TagTypeHandler;

pub fn type_signature_read(_handler: &TagTypeHandler, io: &mut IoHandler, n_items: &mut usize, _size_of_tag: usize) ->Result<Box<dyn Any>> {
    *n_items = 0;

    let value = Box::new(read_signature(io)?);

    *n_items = 1;
    Ok(value)
}

pub fn type_signature_write(_handler: &TagTypeHandler, io: &mut IoHandler, ptr: &dyn Any, _n_items: usize) -> Result<()> {
    match ptr.downcast_ref::<Signature>() {
        None => Err("Invalid object to write with type_signature_write".into()),
        Some(value) => {
            write_signature(io, *value)?;

            Ok(())
        },
    }
}

type_dup_and_free!(signature, Signature);
