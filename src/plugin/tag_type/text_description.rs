use std::{any::Any, mem::size_of};

use crate::{
    io::IoHandler,
    plugin::{read_signature, read_u16, read_u32, read_u8, write_signature, write_u32, write_u16, write_u8},
    sig,
    types::{Dup, Signature, MLU},
    Result, NO_COUNTRY, NO_LANGUAGE, inlines::align_long,
};

use super::{TagTypeHandler, write_utf16_slice};

pub fn type_text_description_read(
    handler: &TagTypeHandler,
    io: &mut IoHandler,
    n_items: &mut usize,
    size_of_tag: usize,
) -> Result<Box<dyn Any>> {
    *n_items = 0;

    // One dword should be there
    if size_of_tag < size_of::<u32>() {
        return Err("Tag is too small to read in type_text_description_read".into());
    }

    // Read len of ASCII
    let ascii_count = read_u32(io)? as usize;
    let size_of_tag = size_of_tag - size_of::<u32>();

    // Check for size
    if size_of_tag < ascii_count {
        return Err("Invalid tag size in type_text_description_read".into());
    }

    // All seems Ok, create the container
    let mut mlu = MLU::new(&handler.context_id, 1);

    // Allocate as much memory as size of tag
    let mut text = vec![0u8; ascii_count];

    // Read it
    if (io.read)(io, &mut text, size_of::<u8>(), ascii_count) != ascii_count {
        return Err("Read error in type_text_description_read".into());
    }
    let size_of_tag = size_of_tag - ascii_count;

    // Set the MLU entry. From here we can be tolerant to wrong types
    mlu.set_ascii(NO_LANGUAGE, NO_COUNTRY, &text)?;

    *n_items = 1;

    // Skip Unicode code
    if size_of_tag < 2 * size_of::<u32>() {
        return Ok(Box::new(mlu));
    }
    if read_u32(io).is_err() {
        // Unicode code
        return Ok(Box::new(mlu));
    }
    let unicode_count = match read_u32(io) {
        Ok(value) => value,
        Err(_) => {
            return Ok(Box::new(mlu));
        }
    };

    let size_of_tag = size_of_tag - 2 * size_of::<u32>();

    // Skip ScriptCode code if present. Some buggy profiles have less data
    // than strictly required. We need to skip it as this type my come
    // embedded in other types.

    if size_of_tag >= size_of::<u16>() + size_of::<u8>() + 67 {
        if read_u16(io).is_err() {
            return Ok(Box::new(mlu));
        }
        if read_u8(io).is_err() {
            return Ok(Box::new(mlu));
        }
        // Skip rest of tag
        let mut dummy = [0u8];
        for i in 0..67 {
            if (io.read)(io, &mut dummy, size_of::<u8>(), 1) != size_of::<u8>() {
                return Ok(Box::new(mlu));
            }
        }
    }

    Ok(Box::new(mlu))
}

pub fn type_text_description_write(
    _handler: &TagTypeHandler,
    io: &mut IoHandler,
    ptr: &dyn Any,
    _n_items: usize,
) -> Result<()> {
    fn write_err() -> Result<()> {
        Err("Write error in type_text_description_write".into())
    }

    match ptr.downcast_ref::<MLU>() {
        None => Err("Invalid object to write with type_text_description_write".into()),
        Some(mlu) => {
            let filler = [0u8; 68];

            // Get the len of string
            let len = match mlu.get_ascii(NO_LANGUAGE, NO_COUNTRY, &mut []) {
                Some(value) => value,
                None => return Err("No ascii text to write in type_text_description_write".into()),
            };

            // Get both representations
            let mut text = vec![0u8; len];
            let mut wide = vec![0u16; len];

            if let None = mlu.get_ascii(NO_LANGUAGE, NO_COUNTRY, &mut text) {
                return Err("No text to write in type_text_description_write".into());
            }
            mlu.get_wide(NO_LANGUAGE, NO_COUNTRY, &mut wide);

            // Tell the real text len
            let len_text = text.len();
            // Compute a total tag size requirement
            let len_tag_requirement = 8  // Alignment
                + 4                             // count
                + len_text                      // desc[count]
                + 4                             // ucLangCode
                + 4                             // ucCount
                + (2 * len_text)                // ucDesc[ucCount]
                + 2                             // scCode
                + 1                             // scCount
                + 67                            // scDesc[67]
                + 0;
            let len_aligned = align_long(len_tag_requirement);

            write_u32(io, len_text as u32)?;
            if !(io.write)(io, len_text, &text) {
                return write_err();
            }

            write_u32(io, 0)?;  // ucLanguageCode

            write_u32(io, len_text as u32)?;
            write_utf16_slice(io, &wide)?;

            // ScriptCode Code & count (unused)
            write_u16(io, 0)?;
            write_u8(io, 0)?;

            if !(io.write)(io, 67usize, &filler) {
                return write_err();
            }

            // possibly add pad at the end of tag
            if len_aligned - len_tag_requirement > 0 {
                if !(io.write)(io, len_aligned - len_tag_requirement, &filler) {
                    return write_err();
                }
            }

            Ok(())
        }
    }
}

type_dup_and_free!(text_description, MLU);

pub fn decide_text_desc_type(icc_version: f64, _data: Box<dyn Any>) -> Signature {
    if icc_version >= 4.0 {
        sig::types::MULTI_LOCALIZED_UNICODE
    } else {
        sig::types::TEXT
    }
}
