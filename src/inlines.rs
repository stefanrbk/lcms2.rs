use std::{mem::size_of, ops::Shl};

use crate::{
    plugin::{IMutex, MutexGuard},
    PTR_ALIGNMENT, S15F16,
};

#[inline]
pub const fn align_long(x: usize) -> usize {
    (x + (size_of::<u32>() - 1)) & !(size_of::<u32>() - 1)
}
#[inline]
pub const fn align_mem(x: usize) -> usize {
    x + (PTR_ALIGNMENT - 1) & !(PTR_ALIGNMENT - 1)
}

#[inline]
pub const fn from_8_to_16(rgb: u8) -> u16 {
    ((rgb as u16) << 8) | rgb as u16
}

#[inline]
pub const fn from_16_to_8(rgb: u16) -> u8 {
    ((((rgb as u32) * 65281 + 8388608) >> 24) & 0xFF) as u8
}

#[inline]
pub const fn fixed_to_int(x: S15F16) -> i32 {
    x >> 16
}

#[inline]
pub const fn fixed_rest_to_int(x: S15F16) -> i32 {
    x & 0xFFFF
}

#[inline]
pub const fn round_fixed_to_int(x: S15F16) -> i32 {
    (x + 0x8000) >> 16
}

#[inline]
pub const fn to_fixed_domain(a: i32) -> S15F16 {
    a + ((a + 0x7fff) / 0xffff)
}

#[inline]
pub const fn from_fixed_domain(a: S15F16) -> i32 {
    a - ((a + 0x7fff) >> 16)
}

#[inline]
pub const fn quick_floor(val: f64) -> i32 {
    const DOUBLE_2_FIX_MAGIC: f64 = 68719476736.0 * 1.5;

    union Split {
        pub val: f64,
        pub halves: [i32; 2],
    }

    let i = Split { val };

    unsafe { i.halves[0] }
}

#[inline]
pub fn quick_floor_word(d: f64) -> u16 {
    (quick_floor(d - 32767.0) + 32767) as u16
}

#[inline]
pub fn quick_saturate_word(d: f64) -> u16 {
    let d = d + 0.5;
    if d <= 0.0 {
        return 0;
    }
    if d >= 65535.0 {
        return 0xffff;
    }

    quick_floor_word(d)
}

#[inline]
pub fn lock_primitive<'a>(m: &'a dyn IMutex<'a>) -> Box<dyn MutexGuard + 'a> {
    m.lock()
}

#[inline]
pub fn unlock_primitive(_m: &dyn IMutex, g: Box<dyn MutexGuard>) {
    drop(g)
}

#[inline]
pub fn init_mutex_primitive() -> Box<dyn IMutex<'static>> {
    Box::new(std::sync::Mutex::new(()))
}

#[inline]
pub fn destroy_mutex_primitive(m: Box<dyn IMutex>) {
    drop(m)
}
