use bitfield::bitfield;

pub const LCMS_VERSION: u32 = 2150;

bitfield! {
    pub struct Format(u32);
    pub u8, bytes, set_bytes: 2, 0;
    pub u8, channels, set_channels: 6, 3;
    pub u8, extra, set_extra: 9, 7;
    pub bool, doswap, set_doswap: 10;
    pub bool, endian16, set_endian16: 11;
    pub bool, planar, set_planar: 12;
    pub bool, flavor, set_flavor: 13;
    pub bool, swapfirst, set_swapfirst: 14;
    pub u8, colorspace, set_colorspace: 20, 16;
    pub bool, optimized, set_optimized: 21;
    pub bool, float, set_float: 22;
    pub bool, premul, set_premul: 23;
}

pub const fn premul_sh(m: u32) -> u32 {
    m << 23
}
pub const fn float_sh(m: u32) -> u32 {
    m << 22
}
pub const fn optimized_sh(m: u32) -> u32 {
    m << 21
}
pub const fn colorspace_sh(m: u32) -> u32 {
    m << 16
}
pub const fn swapfirst_sh(m: u32) -> u32 {
    m << 14
}
pub const fn flavor_sh(m: u32) -> u32 {
    m << 13
}
pub const fn planar_sh(m: u32) -> u32 {
    m << 12
}
pub const fn endian16_sh(m: u32) -> u32 {
    m << 11
}
pub const fn doswap_sh(m: u32) -> u32 {
    m << 10
}
pub const fn extra_sh(m: u32) -> u32 {
    m << 7
}
pub const fn channels_sh(m: u32) -> u32 {
    m << 3
}
pub const fn bytes_sh(m: u32) -> u32 {
    m << 0
}
