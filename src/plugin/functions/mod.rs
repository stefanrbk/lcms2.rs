use chrono::{DateTime as dt, Datelike, TimeZone, Timelike, Utc};
use std::mem::size_of;

use crate::{
    inlines::align_long,
    io::IoHandler,
    s15f16,
    types::{DateTime, Signature, XYZ},
    u16f16, u8f8, Result,
};

#[inline]
pub const fn adjust_endianess16(word: u16) -> u16 {
    word.to_be()
}

#[inline]
pub const fn adjust_endianess32(d_word: u32) -> u32 {
    d_word.to_be()
}

#[inline]
pub const fn adjust_endianess64(q_word: u64) -> u64 {
    q_word.to_be()
}

pub fn read_u8(io: &mut IoHandler) -> Result<u8> {
    let mut tmp = [0u8; size_of::<u8>()];
    if (io.read)(io, &mut tmp, size_of::<u8>(), 1) != size_of::<u8>() {
        return Err("Read error in read_u8".into());
    }

    Ok(tmp[0])
}

pub fn read_u16(io: &mut IoHandler) -> Result<u16> {
    let mut tmp = [0u8; size_of::<u16>()];
    if (io.read)(io, &mut tmp, size_of::<u16>(), 1) != size_of::<u16>() {
        return Err("Read error in read_u16".into());
    }

    Ok(adjust_endianess16(u16::from_ne_bytes(tmp)))
}

pub fn read_u16_slice<'a>(io: &mut IoHandler, array: &'a mut [u16]) -> Result<&'a [u16]> {
    for i in 0..array.len() {
        let value = read_u16(io);
        if value.is_err() {
            return Err("Read error in read_u16_array".into());
        }
        unsafe {
            array[i] = value.unwrap_unchecked();
        }
    }

    Ok(array)
}

pub fn read_u32(io: &mut IoHandler) -> Result<u32> {
    let mut tmp = [0u8; size_of::<u32>()];
    if (io.read)(io, &mut tmp, size_of::<u32>(), 1) != size_of::<u32>() {
        return Err("Read error in read_u32".into());
    }

    Ok(adjust_endianess32(u32::from_ne_bytes(tmp)))
}

pub fn read_f32(io: &mut IoHandler) -> Result<f32> {
    let mut tmp = [0u8; size_of::<f32>()];
    if (io.read)(io, &mut tmp, size_of::<f32>(), 1) != size_of::<f32>() {
        return Err("Read error in read_f32".into());
    }

    let result = f32::from_bits(adjust_endianess32(u32::from_ne_bytes(tmp)));
    if result > 1e20f32 || result < -1e20f32 {
        return Err("Float values are out of bounds in read_f32".into());
    }

    if result.is_normal() || result == 0f32 {
        return Ok(result);
    }

    Err("Float value was subnormal in read_f32".into())
}

pub fn read_signature(io: &mut IoHandler) -> Result<Signature> {
    match read_u32(io) {
        Ok(value) => Ok(Signature(value)),
        Err(_) => Err("Read error in read_signature".into()),
    }
}

pub fn read_u64(io: &mut IoHandler) -> Result<u64> {
    let mut tmp = [0u8; size_of::<u64>()];
    if (io.read)(io, &mut tmp, size_of::<u64>(), 1) != size_of::<u64>() {
        return Err("Read error in read_u64".into());
    }

    Ok(adjust_endianess64(u64::from_ne_bytes(tmp)))
}

pub fn read_s15f16(io: &mut IoHandler) -> Result<f64> {
    match read_u32(io) {
        Ok(value) => Ok(s15_fixed16_to_f64(i32::from_ne_bytes(value.to_ne_bytes()))),
        Err(_) => Err("Read error in read_s15f16".into()),
    }
}

pub fn read_u16f16(io: &mut IoHandler) -> Result<f64> {
    match read_u32(io) {
        Ok(value) => Ok(u16_fixed16_to_f64(value)),
        Err(_) => Err("Read error in read_u16f16".into()),
    }
}

pub fn read_xyz(io: &mut IoHandler) -> Result<XYZ> {
    let x = read_s15f16(io);
    if x.is_err() {
        return Err("Read error in read_xyz".into());
    }

    let y = read_s15f16(io);
    if y.is_err() {
        return Err("Read error in read_xyz".into());
    }

    let z = read_s15f16(io);
    if z.is_err() {
        return Err("Read error in read_xyz".into());
    }

    unsafe {
        Ok(XYZ {
            x: x.unwrap_unchecked(),
            y: y.unwrap_unchecked(),
            z: z.unwrap_unchecked(),
        })
    }
}

pub fn write_u8(io: &mut IoHandler, n: u8) -> Result<()> {
    let tmp = [n];
    match (io.write)(io, size_of::<u8>(), &tmp) {
        true => Ok(()),
        false => Err("Write error in write_u8".into()),
    }
}

pub fn write_u16(io: &mut IoHandler, n: u16) -> Result<()> {
    let tmp = adjust_endianess16(n).to_ne_bytes();
    match (io.write)(io, size_of::<u16>(), &tmp) {
        true => Ok(()),
        false => Err("Write error in write_u16".into()),
    }
}

pub fn write_u16_slice(io: &mut IoHandler, array: &[u16]) -> Result<()> {
    for n in array {
        if write_u16(io, *n).is_err() {
            return Err("Write error in write_u16".into());
        }
    }

    Ok(())
}

pub fn write_u32(io: &mut IoHandler, n: u32) -> Result<()> {
    let tmp = adjust_endianess32(n).to_ne_bytes();
    match (io.write)(io, size_of::<f32>(), &tmp) {
        true => Ok(()),
        false => Err("Write error in write_u32".into()),
    }
}

pub fn write_f32(io: &mut IoHandler, n: f32) -> Result<()> {
    let tmp = adjust_endianess32(n.to_bits()).to_ne_bytes();
    match (io.write)(io, size_of::<u32>(), &tmp) {
        true => Ok(()),
        false => Err("Write error in write_f32".into()),
    }
}

pub fn write_signature(io: &mut IoHandler, n: Signature) -> Result<()> {
    match write_u32(io, n.0) {
        Ok(_) => Ok(()),
        Err(_) => Err("Write error in write_signature".into()),
    }
}

pub fn write_u64(io: &mut IoHandler, n: u64) -> Result<()> {
    let tmp = adjust_endianess64(n).to_ne_bytes();
    match (io.write)(io, size_of::<u64>(), &tmp) {
        true => Ok(()),
        false => Err("Write error in write_u64".into()),
    }
}

pub fn write_s15f16(io: &mut IoHandler, n: f64) -> Result<()> {
    let n = u32::from_ne_bytes(f64_to_s15_fixed16(n).to_ne_bytes());
    match write_u32(io, n) {
        Ok(_) => Ok(()),
        Err(_) => Err("Write error in write_s15f16".into()),
    }
}

pub fn write_u16f16(io: &mut IoHandler, n: f64) -> Result<()> {
    let n = f64_to_u16_fixed16(n);
    match write_u32(io, n) {
        Ok(_) => Ok(()),
        Err(_) => Err("Write error in write_u16f16".into()),
    }
}

pub fn write_xyz(io: &mut IoHandler, xyz: XYZ) -> Result<()> {
    if write_s15f16(io, xyz.x).is_err()
        || write_s15f16(io, xyz.y).is_err()
        || write_s15f16(io, xyz.z).is_err()
    {
        return Err("Write error in read_xyz".into());
    }

    Ok(())
}

pub fn read_type_base(io: &mut IoHandler) -> Result<Signature> {
    let mut tmp = [0u8; size_of::<u32>()];
    if (io.read)(io, &mut tmp, size_of::<u32>(), 1) != size_of::<u32>() {
        return Err("Write error in read_type_base".into());
    }
    let result = Signature(u32::from_ne_bytes(tmp));

    if (io.read)(io, &mut tmp, size_of::<u32>(), 1) != size_of::<u32>() {
        return Err("Write error in read_type_base".into());
    }
    Ok(result)
}

pub fn write_type_base(io: &mut IoHandler, sig: Signature) -> Result<()> {
    if write_signature(io, sig).is_err() || write_u32(io, 0u32).is_err() {
        return Err("Write error in write_type_base".into());
    }

    Ok(())
}

pub fn read_alignment(io: &mut IoHandler) -> Result<()> {
    let mut buffer = [0u8; 4];

    let at = (io.tell)(io);
    let next_aligned = align_long(at);
    let bytes_to_next_aligned_pos = next_aligned - at;
    if bytes_to_next_aligned_pos == 0 {
        return Ok(());
    }

    if bytes_to_next_aligned_pos > 4 {
        return Err("Alignment issues in read_alignment".into());
    }

    if (io.read)(io, &mut buffer, bytes_to_next_aligned_pos, 1) != bytes_to_next_aligned_pos {
        Err("Read error in read_alignment".into())
    } else {
        Ok(())
    }
}

pub fn write_alignment(io: &mut IoHandler) -> Result<()> {
    let buffer = [0u8; 4];

    let at = (io.tell)(io);
    let next_aligned = align_long(at);
    let bytes_to_next_aligned_pos = next_aligned - at;
    if bytes_to_next_aligned_pos == 0 {
        return Ok(());
    }

    if bytes_to_next_aligned_pos > 4 {
        return Err("Alignment issues in write_alignment".into());
    }

    match (io.write)(io, bytes_to_next_aligned_pos, &buffer) {
        false => Err("Write error in write_alignment".into()),
        true => Ok(()),
    }
}

pub fn u8f8_to_f64(fixed8: u8f8) -> f64 {
    let lsb = fixed8 & 0xFF;
    let msb = fixed8 >> 8;

    msb as f64 + (lsb as f64 / 255.0)
}

pub fn f64_to_u8f8(val: f64) -> u8f8 {
    let tmp = f64_to_s15_fixed16(val);
    ((tmp >> 8) & 0xFFFF) as u8f8
}

pub fn s15_fixed16_to_f64(fix32: s15f16) -> f64 {
    let sign = if fix32 < 0 { -1.0 } else { 1.0 };
    let fix32 = fix32.abs();

    let whole = ((fix32 >> 16) & 0xFFFF) as u16;
    let frac_part = (fix32 & 0xFFFF) as u16;

    let mid = frac_part as f64 / 65536.0;
    let floater = whole as f64 + mid;

    floater * sign
}

pub fn u16_fixed16_to_f64(fix32: u16f16) -> f64 {
    fix32 as f64 / 65536.0
}

pub fn f64_to_s15_fixed16(v: f64) -> s15f16 {
    f64::floor((v * 65536.0) + 0.5) as s15f16
}

pub fn f64_to_u16_fixed16(v: f64) -> u16f16 {
    f64::floor((v * 65536.0) + 0.5) as u16f16
}

pub fn encode_date_time(source: dt<Utc>) -> DateTime {
    DateTime {
        seconds: adjust_endianess16(source.second() as u16),
        minutes: adjust_endianess16(source.minute() as u16),
        hours: adjust_endianess16(source.hour() as u16),
        day: adjust_endianess16(source.day() as u16),
        month: adjust_endianess16(source.month() as u16),
        year: adjust_endianess16(source.year() as u16),
    }
}

pub fn decode_date_time(source: DateTime) -> dt<Utc> {
    let utc = Utc;
    utc.with_ymd_and_hms(
        source.year as i32,
        source.month as u32,
        source.day as u32,
        source.hours as u32,
        source.minutes as u32,
        source.seconds as u32,
    )
    .unwrap()
}
