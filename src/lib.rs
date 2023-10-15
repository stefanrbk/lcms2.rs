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

pub const PT_ANY: u32 = 0;
pub const PT_GRAY: u32 = 3;
pub const PT_RGB: u32 = 4;
pub const PT_CMY: u32 = 5;
pub const PT_CMYK: u32 = 6;
pub const PT_YCB_CR: u32 = 7;
pub const PT_YUV: u32 = 8;
pub const PT_XYZ: u32 = 9;
pub const PT_LAB: u32 = 10;
pub const PT_YUVK: u32 = 11;
pub const PT_HSV: u32 = 12;
pub const PT_HLS: u32 = 13;
pub const PT_YXY: u32 = 14;
pub const PT_MCH1: u32 = 15;
pub const PT_MCH2: u32 = 16;
pub const PT_MCH3: u32 = 17;
pub const PT_MCH4: u32 = 18;
pub const PT_MCH5: u32 = 19;
pub const PT_MCH6: u32 = 20;
pub const PT_MCH7: u32 = 21;
pub const PT_MCH8: u32 = 22;
pub const PT_MCH9: u32 = 23;
pub const PT_MCH10: u32 = 24;
pub const PT_MCH11: u32 = 25;
pub const PT_MCH12: u32 = 26;
pub const PT_MCH13: u32 = 27;
pub const PT_MCH14: u32 = 28;
pub const PT_MCH15: u32 = 29;
pub const PT_LAB_V2: u32 = 30;

pub const TYPE_GRAY_8:u32 = colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(1);
pub const TYPE_GRAY_8_REV:u32 = colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(1) | flavor_sh(1);
pub const TYPE_GRAY_16:u32 = colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2);
pub const TYPE_GRAY_16_REV:u32 = colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2) | flavor_sh(1);
pub const TYPE_GRAY_16_SE:u32 = colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_GRAYA_8:u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1);
pub const TYPE_GRAYA_8_PREMUL:u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1) | premul_sh(1);
pub const TYPE_GRAYA_16:u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2);
pub const TYPE_GRAYA_16_PREMUL:u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | premul_sh(1);
pub const TYPE_GRAYA_16_SE:u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_GRAYA_8_PLANAR:u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1) | planar_sh(1);
pub const TYPE_GRAYA_16_PLANAR:u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | planar_sh(1);

pub const TYPE_RGB_8:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1);
pub const TYPE_RGB_8_PLANAR:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_BGR_8:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_BGR_8_PLANAR:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | planar_sh(1);
pub const TYPE_RGB_16:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2);
pub const TYPE_RGB_16_PLANAR:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_RGB_16_SE:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_BGR_16:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_BGR_16_PLANAR:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | planar_sh(1);
pub const TYPE_BGR_16_SE:u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);

pub const TYPE_RGBA_8:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1);
pub const TYPE_RGBA_8_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | premul_sh(1);
pub const TYPE_RGBA_8_PLANAR:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_RGBA_16:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2);
pub const TYPE_RGBA_16_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | premul_sh(1);
pub const TYPE_RGBA_16_PLANAR:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_RGBA_16_SE:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

pub const TYPE_ARGB_8:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | swapfirst_sh(1);
pub const TYPE_ARGB_8_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | swapfirst_sh(1) | premul_sh(1);
pub const TYPE_ARGB_8_PLANAR:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | swapfirst_sh(1) | planar_sh(1);
pub const TYPE_ARGB_16:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | swapfirst_sh(1);
pub const TYPE_ARGB_16_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | swapfirst_sh(1) | premul_sh(1);

pub const TYPE_ABGR_8:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_ABGR_8_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | premul_sh(1);
pub const TYPE_ABGR_8_PLANAR:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | planar_sh(1);
pub const TYPE_ABGR_16:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_ABGR_16_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | premul_sh(1);
pub const TYPE_ABGR_16_PLANAR:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | planar_sh(1);
pub const TYPE_ABGR_16_SE:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);

pub const TYPE_BGRA_8:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | swapfirst_sh(1);
pub const TYPE_BGRA_8_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | swapfirst_sh(1) | premul_sh(1);
pub const TYPE_BGRA_8_PLANAR:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | swapfirst_sh(1) | planar_sh(1);
pub const TYPE_BGRA_16:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | swapfirst_sh(1);
pub const TYPE_BGRA_16_PREMUL:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | swapfirst_sh(1) | premul_sh(1);
pub const TYPE_BGRA_16_SE:u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | endian16_sh(1) | doswap_sh(1) | swapfirst_sh(1);

pub const TYPE_CMY_8:u32 = colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(1);
pub const TYPE_CMY_8_PLANAR:u32 = colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_CMY_16:u32 = colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(2);
pub const TYPE_CMY_16_PLANAR:u32 = colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_CMY_16_SE:u32 = colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

pub const TYPE_CMYK_8:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1);
pub const TYPE_CMYKA_8:u32 = colorspace_sh(PT_CMYK) | extra_sh(1) | channels_sh(4) | bytes_sh(1);
pub const TYPE_CMYK_8_REV:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | flavor_sh(1);
pub const TYPE_YUVK_8:u32 = TYPE_CMYK_8_REV;
pub const TYPE_CMYK_8_PLANAR:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | planar_sh(1);
pub const TYPE_CMYK_16:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2);
pub const TYPE_CMYK_16_REV:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | flavor_sh(1);
pub const TYPE_YUVK_16:u32 = TYPE_CMYK_16_REV;
pub const TYPE_CMYK_16_PLANAR:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | planar_sh(1);
pub const TYPE_CMYK_16_SE:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | endian16_sh(1);

pub const TYPE_KYMC_8:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC_16:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC_16_SE:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);

pub const TYPE_KCMY_8:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | swapfirst_sh(1);
pub const TYPE_KCMY_8_REV:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | flavor_sh(1) | swapfirst_sh(1);
pub const TYPE_KCMY_16:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | swapfirst_sh(1);
pub const TYPE_KCMY_16_REV:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | flavor_sh(1) | swapfirst_sh(1);
pub const TYPE_KCMY_16_SE:u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | endian16_sh(1) | swapfirst_sh(1);

pub const TYPE_CMYK5_8:u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(1);
pub const TYPE_CMYK5_16:u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2);
pub const TYPE_CMYK5_16_SE:u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC5_8:u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC5_16:u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC5_16_SE:u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK6_8:u32 = colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(1);
pub const TYPE_CMYK6_8_PLANAR:u32 = colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(1) | planar_sh(1);
pub const TYPE_CMYK6_16:u32 = colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(2);
pub const TYPE_CMYK6_16_PLANAR:u32 = colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(2) | planar_sh(1);
pub const TYPE_CMYK6_16_SE:u32 = colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_CMYK7_8:u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(1);
pub const TYPE_CMYK7_16:u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2);
pub const TYPE_CMYK7_16_SE:u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC7_8:u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC7_16:u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC7_16_SE:u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK8_8:u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(1);
pub const TYPE_CMYK8_16:u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2);
pub const TYPE_CMYK8_16_SE:u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC8_8:u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC8_16:u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC8_16_SE:u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK9_8:u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(1);
pub const TYPE_CMYK9_16:u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2);
pub const TYPE_CMYK9_16_SE:u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC9_8:u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC9_16:u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC9_16_SE:u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK10_8:u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(1);
pub const TYPE_CMYK10_16:u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2);
pub const TYPE_CMYK10_16_SE:u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC10_8:u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC10_16:u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC10_16_SE:u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK11_8:u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(1);
pub const TYPE_CMYK11_16:u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2);
pub const TYPE_CMYK11_16_SE:u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC11_8:u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC11_16:u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC11_16_SE:u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK12_8:u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(1);
pub const TYPE_CMYK12_16:u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2);
pub const TYPE_CMYK12_16_SE:u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC12_8:u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC12_16:u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC12_16_SE:u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);

// Colorimetric
pub const TYPE_XYZ_16:u32 = colorspace_sh(PT_XYZ) | channels_sh(3) | bytes_sh(2);

pub const TYPE_LAB_8:u32 = colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(1);
pub const TYPE_LAB_V2_8:u32 = colorspace_sh(PT_LAB_V2) | channels_sh(3) | bytes_sh(1);

pub const TYPE_ALAB_8:u32 = colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(1) | extra_sh(1) | swapfirst_sh(1);
pub const TYPE_ALAB_V2_8:u32 = colorspace_sh(PT_LAB_V2) | channels_sh(3) | bytes_sh(1) | extra_sh(1) | swapfirst_sh(1);
pub const TYPE_LAB_16:u32 = colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(2);
pub const TYPE_LAB_V2_16:u32 = colorspace_sh(PT_LAB_V2) | channels_sh(3) | bytes_sh(2);
pub const TYPE_YXY_16:u32 = colorspace_sh(PT_YXY) | channels_sh(3) | bytes_sh(2);

// YCbCr
pub const TYPE_YCB_CR_8:u32 = colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(1);

pub const TYPE_YCB_CR_8_PLANAR:u32 = colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_YCB_CR_16:u32 = colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(2);
pub const TYPE_YCB_CR_16_PLANAR:u32 = colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_YCB_CR_16_SE:u32 = colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// YUV
pub const TYPE_YUV_8:u32 = colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(1);

pub const TYPE_YUV_8_PLANAR:u32 = colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_YUV_16:u32 = colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(2);
pub const TYPE_YUV_16_PLANAR:u32 = colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_YUV_16_SE:u32 = colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// HLS
pub const TYPE_HLS_8:u32 = colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(1);

pub const TYPE_HLS_8_PLANAR:u32 = colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_HLS_16:u32 = colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(2);
pub const TYPE_HLS_16_PLANAR:u32 = colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_HLS_16_SE:u32 = colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// HSV
pub const TYPE_HSV_8:u32 = colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(1);

pub const TYPE_HSV_8_PLANAR:u32 = colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_HSV_16:u32 = colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(2);
pub const TYPE_HSV_16_PLANAR:u32 = colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_HSV_16_SE:u32 = colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// Named color index. Only 16 bits is allowed (don't check colorspace)
pub const TYPE_NAMED_COLOR_INDEX:u32 = channels_sh(1) | bytes_sh(2);

// Float formatters.
pub const TYPE_XYZ_FLT:u32 = float_sh(1) | colorspace_sh(PT_XYZ) | channels_sh(3) | bytes_sh(4);

pub const TYPE_LAB_FLT:u32 = float_sh(1) | colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(4);
pub const TYPE_LAB_A_FLT:u32 = float_sh(1) | colorspace_sh(PT_LAB) | extra_sh(1) | channels_sh(3) | bytes_sh(4);
pub const TYPE_GRAY_FLT:u32 = float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(4);
pub const TYPE_GRAYA_FLT:u32 = float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(4) | extra_sh(1);
pub const TYPE_GRAYA_FLT_PREMUL:u32 = float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(4) | extra_sh(1) | premul_sh(1);
pub const TYPE_RGB_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(4);

pub const TYPE_RGBA_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4);
pub const TYPE_RGBA_FLT_PREMUL:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | premul_sh(1);
pub const TYPE_ARGB_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | swapfirst_sh(1);
pub const TYPE_ARGB_FLT_PREMUL:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | swapfirst_sh(1) | premul_sh(1);
pub const TYPE_BGR_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(4) | doswap_sh(1);
pub const TYPE_BGRA_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | doswap_sh(1) | swapfirst_sh(1);
pub const TYPE_BGRA_FLT_PREMUL:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | doswap_sh(1) | swapfirst_sh(1) | premul_sh(1);
pub const TYPE_ABGR_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | doswap_sh(1);
pub const TYPE_ABGR_FLT_PREMUL:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | doswap_sh(1) | premul_sh(1);

pub const TYPE_CMYK_FLT:u32 = float_sh(1) | colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(4);

// Floating point formatters.
// NOTE THAT 'BYTES' FIELD IS SET TO ZERO ON DLB because 8 bytes overflows the bitfield
pub const TYPE_XYZ_DBL:u32 = float_sh(1) | colorspace_sh(PT_XYZ) | channels_sh(3) | bytes_sh(0);

pub const TYPE_LAB_DBL:u32 = float_sh(1) | colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(0);
pub const TYPE_GRAY_DBL:u32 = float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(0);
pub const TYPE_RGB_DBL:u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(0);
pub const TYPE_BGR_DBL:u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(0) | doswap_sh(1);
pub const TYPE_CMYK_DBL:u32 = float_sh(1) | colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(0);

// IEEE 754-2008 "half"
pub const TYPE_GRAY_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2);

pub const TYPE_RGB_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2);
pub const TYPE_RGBA_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2);
pub const TYPE_CMYK_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2);

pub const TYPE_ARGB_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | swapfirst_sh(1);
pub const TYPE_BGR_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_BGRA_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | swapfirst_sh(1);
pub const TYPE_ABGR_HALF_FLT:u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);
