use std::mem::size_of;

use crate::{S15F16, PTR_ALIGNMENT, plugin::{IMutex, MutexGuard}};

#[inline]
pub const fn AlignLong(x: usize) -> usize {
    (x + (size_of::<u32>() - 1)) & !(size_of::<u32>() - 1)
}
#[inline]
pub const fn AlignMem(x: usize) -> usize {
    x + (PTR_ALIGNMENT - 1) & !(PTR_ALIGNMENT - 1)
}

#[inline]
pub const fn From8To16(rgb: u8) -> u16 {
    ((rgb as u16) << 8) | rgb as u16
}

#[inline]
pub const fn From16To8(rgb: u16) -> u8 {
    ((((rgb as u32) * 65281 + 8388608) >> 24) & 0xFF) as u8
}

#[inline]
pub const fn FixedToInt(x: S15F16) -> i32 {
    x >> 16
}

#[inline]
pub const fn FixedRestToInt(x: S15F16) -> i32 {
    x & 0xFFFF
}

#[inline]
pub const fn RoundFixedToInt(x: S15F16) -> i32 {
    (x + 0x8000) >> 16
}

#[inline]
pub const fn ToFixedDomain(a: i32) -> S15F16 {
    a + ((a + 0x7fff) / 0xffff)
}

#[inline]
pub const fn FromFixedDomain(a: S15F16) -> i32 {
    a - ((a + 0x7fff) >> 16)
}

#[inline]
pub const fn QuickFloor(val: f64) -> i32 {
    const double2fixmagic: f64 = 68719476736.0 * 1.5;

    union Split {
        pub val: f64,
        pub halves: [i32; 2],
    }

    let i = Split { val };

    unsafe { i.halves[0] }
}

#[inline]
pub fn QuickFloorWord(d: f64) -> u16 {
    (QuickFloor(d - 32767.0) + 32767) as u16
}

#[inline]
pub fn QuickSaturateWord(d: f64) -> u16 {
    let d = d + 0.5;
    if d <= 0.0 {
        return 0;
    }
    if d >= 65535.0 {
        return 0xffff;
    }

    QuickFloorWord(d)
}

#[inline]
pub fn LockPrimitive<'a>(m: &'a dyn IMutex<'a>) -> Box<dyn MutexGuard + 'a> {
    m.lock()
}

#[inline]
pub fn UnlockPrimitive(_m: &dyn IMutex, g: Box<dyn MutexGuard>) {
    drop(g)
}

#[inline]
pub fn InitMutexPrimitive() -> Box<dyn IMutex<'static>> {
    Box::new(std::sync::Mutex::new(()))
}

#[inline]
pub fn DestroyMutexPrimitive(m: Box<dyn IMutex>) {
    drop(m)
}
