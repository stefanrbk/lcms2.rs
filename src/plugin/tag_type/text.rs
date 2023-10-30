use std::{any::Any, mem::size_of};

use crate::{
    io::IoHandler,
    plugin::{read_signature, write_signature},
    types::{Signature, MLU, Dup},
    Result, NO_COUNTRY, NO_LANGUAGE, sig,
};

use super::TagTypeHandler;

pub fn type_text_read(
    handler: &TagTypeHandler,
    io: &mut IoHandler,
    n_items: &mut usize,
    size_of_tag: usize,
) -> Result<Box<dyn Any>> {
    *n_items = 0;

    let mut value = Box::new(MLU::new(handler.context_id.clone(), 1));

    // We need to store the "\0" at the end, so +1
    if size_of_tag == u32::MAX as usize {
        return Err("Tag is too large to read in type_text_read".into());
    }

    let mut text = vec![0u8; size_of_tag + 1];
    if (io.read)(io, &mut text, size_of::<u8>(), size_of_tag) != size_of_tag {
        return Err("Read error in type_text_read".into());
    }

    // Make sure text is properly ended
    text[size_of_tag] = 0;

    value.as_mut().set_ascii(NO_LANGUAGE, NO_COUNTRY, &text)?;

    *n_items = 1;
    Ok(value)
}

pub fn type_text_write(
    _handler: &TagTypeHandler,
    io: &mut IoHandler,
    ptr: &dyn Any,
    _n_items: usize,
) -> Result<()> {
    match ptr.downcast_ref::<MLU>() {
        None => Err("Invalid object to write with type_text_write".into()),
        Some(mlu) => {
            // Get the size of the string. Note there is an extra "\0" at the end
            let size = match mlu.get_ascii(NO_LANGUAGE, NO_COUNTRY, &mut []) {
                Some(value) => {
                    if value == 0 {
                        return Err("Cannot write zero length text in type_text_write".into());
                    } else {
                        value
                    }
                }
                None => return Err("No ascii text to write in type_text_write".into()),
            };

            let mut text = vec![0u8; size];

            if let None = mlu.get_ascii(NO_LANGUAGE, NO_COUNTRY, &mut text) {
                return Err("No ascii text to write in type_text_write".into());
            }

            if !(io.write)(io, size, &text) {
                return Err("Write error in type_text_write".into());
            }

            Ok(())
        }
    }
}

pub fn type_text_dup(
    _handler: &TagTypeHandler,
    ptr: &dyn Any,
    _n_items: usize,
) -> Result<Box<dyn Any>> {
    match ptr.downcast_ref::<MLU>() {
        None => Err("Invalid object to duplicate with type_text_dup".into()),
        Some(value) => Ok(Box::new(value.clone())),
    }
}

pub fn type_text_free(_handler: &TagTypeHandler, ptr: Box<dyn Any>) {
    drop(ptr);
}

pub fn decide_text_type(icc_version: f64, _data: Box<dyn Any>) -> Signature {
    if icc_version >= 4.0 {
        sig::types::MULTI_LOCALIZED_UNICODE
    } else {
        sig::types::TEXT
    }
}
