use std::any::Any;

use crate::{io::IoHandler, Result, plugin::{read_xyz, write_xyz}, types::{XYZ, Signature}, sig};

use super::TagTypeHandler;

pub fn type_xyz_read(_handler: &TagTypeHandler, io: &mut IoHandler, n_items: &mut usize, _size_of_tag: usize) ->Result<Box<dyn Any>> {
    *n_items = 0;

    let value = read_xyz(io)?;

    *n_items = 1;
    Ok(Box::new(value))
}

pub fn type_xyz_write(_handler: &TagTypeHandler, io: &mut IoHandler, ptr: &dyn Any, _n_items: usize) -> Result<()> {
    match ptr.downcast_ref::<XYZ>() {
        None => Err("Invalid object to write with type_xyz_write".into()),
        Some(xyz) => match write_xyz(io, *xyz) {
            Ok(_) => Ok(()),
            Err(msg) => Err(format!("Error in type_xyz_write. The error was \"{}\"", msg))
        },
    }
}

type_dup_and_free!(xyz, XYZ);

pub fn decide_xyz_type(_icc_version: f64, _data: Box<dyn Any>) -> Signature {
    sig::types::XYZ
}
