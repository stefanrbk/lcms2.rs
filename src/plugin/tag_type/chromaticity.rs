use std::any::Any;

use crate::{
    io::IoHandler,
    plugin::{read_s15f16, read_u16, write_s15f16, write_u16},
    types::{XYYTriple, XYY},
    Result,
};

use super::TagTypeHandler;

pub fn type_chromaticity_read(
    _handler: &TagTypeHandler,
    io: &mut dyn IoHandler,
    n_items: &mut usize,
    size_of_tag: usize,
) -> Result<Box<dyn Any>> {
    *n_items = 0;

    let mut n_chans = read_u16(io)?;

    // Let's recover from a bug introduced in early versions of lcms1
    if n_chans == 0 && size_of_tag == 32 {
        read_u16(io)?;
        n_chans = read_u16(io)?;
    }

    if n_chans != 3 {
        return Err("Invalid number of channels in type_chromaticity_read".into());
    }

    let chrm = XYYTriple {
        red: XYY {
            x: read_s15f16(io)?,
            y: read_s15f16(io)?,
            y_lum: 1.0,
        },
        green: XYY {
            x: read_s15f16(io)?,
            y: read_s15f16(io)?,
            y_lum: 1.0,
        },
        blue: XYY {
            x: read_s15f16(io)?,
            y: read_s15f16(io)?,
            y_lum: 1.0,
        },
    };

    *n_items = 1;
    Ok(Box::new(chrm))
}

fn save_one_chromaticity(x: f64, y: f64, io: &mut dyn IoHandler) -> Result<()> {
    write_s15f16(io, x)?;
    write_s15f16(io, y)?;

    Ok(())
}

pub fn type_chromaticity_write(
    _handler: &TagTypeHandler,
    io: &mut dyn IoHandler,
    ptr: &dyn Any,
    _n_items: usize,
) -> Result<()> {
    match ptr.downcast_ref::<XYYTriple>() {
        None => Err("Invalid object to write with type_chromaticity_write".into()),
        Some(chrm) => {
            write_u16(io, 3)?;
            write_u16(io, 0)?;

            save_one_chromaticity(chrm.red.x, chrm.red.y, io)?;
            save_one_chromaticity(chrm.green.x, chrm.green.y, io)?;
            save_one_chromaticity(chrm.blue.x, chrm.blue.y, io)?;

            Ok(())
        }
    }
}

type_dup_and_free!(chromaticity, XYYTriple);
