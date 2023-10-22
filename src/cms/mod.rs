use crate::device_attribute;
use crate::sig;
use crate::types;
use crate::types::XYZ;

pub mod plugin;

/// Version/release
pub const LCMS_VERSION: u32 = 2150;

pub const MAX_PATH: u32 = 256;

/// D50 XYZ normalized to Y=1.0
pub const D50: XYZ = XYZ {
    x: 0.9642,
    y: 1.0,
    z: 0.8249,
};

/// V4 perceptual black
pub const PERCEPTUAL_BLACK: XYZ = XYZ {
    x: 0.00336,
    y: 0.0034731,
    z: 0.00287,
};

/// Definitions in ICC spec
pub use sig::LCMS_SIGNATURE;
pub use sig::MAGIC_NUMBER;

// Base ICC type definitions
pub use sig::types::CHROMATICITY as SIG_CHROMATICITY_TYPE;
pub use sig::types::CICP as SIG_CICP_TYPE;
pub use sig::types::COLORANT_ORDER as SIG_COLORANT_ORDER_TYPE;
pub use sig::types::COLORANT_TABLE as SIG_COLORANT_TABLE_TYPE;
pub use sig::types::CRD_INFO as SIG_CRD_INFO_TYPE;
pub use sig::types::CURVE as SIG_CURVE_TYPE;
pub use sig::types::DATA as SIG_DATA_TYPE;
pub use sig::types::DATE_TIME as SIG_DATE_TIME_TYPE;
pub use sig::types::DEVICE_SETTINGS as SIG_DEVICE_SETTINGS_TYPE;
pub use sig::types::DICT as SIG_DICT_TYPE;
pub use sig::types::LUT16 as SIG_LUT16_TYPE;
pub use sig::types::LUT8 as SIG_LUT8_TYPE;
pub use sig::types::LUT_A_TO_B as SIG_LUT_A_TO_B_TYPE;
pub use sig::types::LUT_B_TO_A as SIG_LUT_B_TO_A_TYPE;
pub use sig::types::MEASUREMENT as SIG_MEASUREMENT_TYPE;
pub use sig::types::MULTI_LOCALIZED_UNICODE as SIG_MULTI_LOCALIZED_UNICODE_TYPE;
pub use sig::types::MULTI_PROCESS_ELEMENT as SIG_MULTI_PROCESS_ELEMENT_TYPE;
#[deprecated = "use SIG_NAMED_COLOR2_TYPE"]
#[allow(deprecated)]
pub use sig::types::NAMED_COLOR as SIG_NAMED_COLOR_TYPE;
pub use sig::types::NAMED_COLOR2 as SIG_NAMED_COLOR2_TYPE;
pub use sig::types::PARAMETRIC_CURVE as SIG_PARAMETRIC_CURVE_TYPE;
pub use sig::types::PROFILE_SEQUENCE_DESC as SIG_PROFILE_SEQUENCE_DESC_TYPE;
pub use sig::types::PROFILE_SEQUENCE_ID as SIG_PROFILE_SEQUENCE_ID_TYPE;
pub use sig::types::RESPONSE_CURVE_SET16 as SIG_RESPONSE_CURVE_SET16_TYPE;
pub use sig::types::S15_FIXED16_ARRAY as SIG_S15_FIXED16_ARRAY_TYPE;
pub use sig::types::SCREENING as SIG_SCREENING_TYPE;
pub use sig::types::SIGNATURE as SIG_SIGNATURE_TYPE;
pub use sig::types::TEXT as SIG_TEXT_TYPE;
pub use sig::types::TEXT_DESCRIPTION as SIG_TEXT_DESCRIPTION_TYPE;
pub use sig::types::U16_FIXED16_ARRAY as SIG_U16_FIXED16_ARRAY_TYPE;
pub use sig::types::UCR_BG as SIG_UCR_BG_TYPE;
pub use sig::types::UINT16_ARRAY as SIG_UINT16_ARRAY_TYPE;
pub use sig::types::UINT32_ARRAY as SIG_UINT32_ARRAY_TYPE;
pub use sig::types::UINT64_ARRAY as SIG_UINT64_ARRAY_TYPE;
pub use sig::types::UINT8_ARRAY as SIG_UINT8_ARRAY_TYPE;
pub use sig::types::VCGT as SIG_VCGT_TYPE;
pub use sig::types::VIEWING_CONDITIONS as SIG_VIEWING_CONDITIONS_TYPE;
pub use sig::types::XYZ as SIG_XYZ_TYPE;

// Base ICC tag definitions
pub use sig::tags::ARGYLL_ARTS as SIG_ARGYLL_ARTS_TAG;
pub use sig::tags::A_TO_B0 as SIG_A_TO_B0_TAG;
pub use sig::tags::A_TO_B1 as SIG_A_TO_B1_TAG;
pub use sig::tags::A_TO_B2 as SIG_A_TO_B2_TAG;
pub use sig::tags::BLUE_COLORANT as SIG_BLUE_COLORANT_TAG;
pub use sig::tags::BLUE_MATRIX_COLUMN as SIG_BLUE_MATRIX_COLUMN_TAG;
pub use sig::tags::BLUE_TRC as SIG_BLUE_TRC_TAG;
pub use sig::tags::B_TO_A0 as SIG_B_TO_A0_TAG;
pub use sig::tags::B_TO_A1 as SIG_B_TO_A1_TAG;
pub use sig::tags::B_TO_A2 as SIG_B_TO_A2_TAG;
pub use sig::tags::B_TO_D0 as SIG_B_TO_D0_TAG;
pub use sig::tags::B_TO_D1 as SIG_B_TO_D1_TAG;
pub use sig::tags::B_TO_D2 as SIG_B_TO_D2_TAG;
pub use sig::tags::B_TO_D3 as SIG_B_TO_D3_TAG;
pub use sig::tags::CALIBRATION_DATE_TIME as SIG_CALIBRATION_DATE_TIME_TAG;
pub use sig::tags::CHAR_TARGET as SIG_CHAR_TARGET_TAG;
pub use sig::tags::CHROMATICITY as SIG_CHROMATICITY_TAG;
pub use sig::tags::CHROMATIC_ADAPTATION as SIG_CHROMATIC_ADAPTATION_TAG;
pub use sig::tags::CICP as SIG_CICP_TAG;
pub use sig::tags::COLORANT_ORDER as SIG_COLORANT_ORDER_TAG;
pub use sig::tags::COLORANT_TABLE as SIG_COLORANT_TABLE_TAG;
pub use sig::tags::COLORANT_TABLE_OUT as SIG_COLORANT_TABLE_OUT_TAG;
pub use sig::tags::COLORIMETRIC_INTENT_IMAGE_STATE as SIG_COLORIMETRIC_INTENT_IMAGE_STATE_TAG;
pub use sig::tags::COPYRIGHT as SIG_COPYRIGHT_TAG;
pub use sig::tags::CRD_INFO as SIG_CRD_INFO_TAG;
pub use sig::tags::DATA as SIG_DATA_TAG;
pub use sig::tags::DATE_TIME as SIG_DATE_TIME_TAG;
pub use sig::tags::DEVICE_MFG_DESC as SIG_DEVICE_MFG_DESC_TAG;
pub use sig::tags::DEVICE_MODEL_DESC as SIG_DEVICE_MODEL_DESC_TAG;
pub use sig::tags::DEVICE_SETTINGS as SIG_DEVICE_SETTINGS_TAG;
pub use sig::tags::D_TO_B0 as SIG_D_TO_B0_TAG;
pub use sig::tags::D_TO_B1 as SIG_D_TO_B1_TAG;
pub use sig::tags::D_TO_B2 as SIG_D_TO_B2_TAG;
pub use sig::tags::D_TO_B3 as SIG_D_TO_B3_TAG;
pub use sig::tags::GAMUT as SIG_GAMUT_TAG;
pub use sig::tags::GRAY_TRC as SIG_GRAY_TRC_TAG;
pub use sig::tags::GREEN_COLORANT as SIG_GREEN_COLORANT_TAG;
pub use sig::tags::GREEN_MATRIX_COLUMN as SIG_GREEN_MATRIX_COLUMN_TAG;
pub use sig::tags::GREEN_TRC as SIG_GREEN_TRC_TAG;
pub use sig::tags::LUMINANCE as SIG_LUMINANCE_TAG;
pub use sig::tags::MEASUREMENT as SIG_MEASUREMENT_TAG;
pub use sig::tags::MEDIA_BLACK_POINT as SIG_MEDIA_BLACK_POINT_TAG;
pub use sig::tags::MEDIA_WHITE_POINT as SIG_MEDIA_WHITE_POINT_TAG;
pub use sig::tags::META as SIG_META_TAG;
#[deprecated = "use SIG_NAMED_COLOR2_TAG"]
#[allow(deprecated)]
pub use sig::tags::NAMED_COLOR as SIG_NAMED_COLOR_TAG;
pub use sig::tags::NAMED_COLOR2 as SIG_NAMED_COLOR2_TAG;
pub use sig::tags::OUTPUT_RESPONSE as SIG_OUTPUT_RESPONSE_TAG;
pub use sig::tags::PERCEPTUAL_RENDERING_INTENT_GAMUT as SIG_PERCEPTUAL_RENDERING_INTENT_GAMUT_TAG;
pub use sig::tags::PREVIEW0 as SIG_PREVIEW0_TAG;
pub use sig::tags::PREVIEW1 as SIG_PREVIEW1_TAG;
pub use sig::tags::PREVIEW2 as SIG_PREVIEW2_TAG;
pub use sig::tags::PROFILE_DESCRIPTION as SIG_PROFILE_DESCRIPTION_TAG;
pub use sig::tags::PROFILE_DESCRIPTION_ML as SIG_PROFILE_DESCRIPTION_ML_TAG;
pub use sig::tags::PROFILE_SEQUENCE_DESC as SIG_PROFILE_SEQUENCE_DESC_TAG;
pub use sig::tags::PROFILE_SEQUENCE_ID as SIG_PROFILE_SEQUENCE_ID_TAG;
pub use sig::tags::PS2_CRD0 as SIG_PS2_CRD0_TAG;
pub use sig::tags::PS2_CRD1 as SIG_PS2_CRD1_TAG;
pub use sig::tags::PS2_CRD2 as SIG_PS2_CRD2_TAG;
pub use sig::tags::PS2_CRD3 as SIG_PS2_CRD3_TAG;
pub use sig::tags::PS2_CSA as SIG_PS2_CSA_TAG;
pub use sig::tags::PS2_RENDERING_INTENT as SIG_PS2_RENDERING_INTENT_TAG;
pub use sig::tags::RED_COLORANT as SIG_RED_COLORANT_TAG;
pub use sig::tags::RED_MATRIX_COLUMN as SIG_RED_MATRIX_COLUMN_TAG;
pub use sig::tags::RED_TRC as SIG_RED_TRC_TAG;
pub use sig::tags::SATURATION_RENDERING_INTENT_GAMUT as SIG_SATURATION_RENDERING_INTENT_GAMUT_TAG;
pub use sig::tags::SCREENING as SIG_SCREENING_TAG;
pub use sig::tags::SCREENING_DESC as SIG_SCREENING_DESC_TAG;
pub use sig::tags::TECHNOLOGY as SIG_TECHNOLOGY_TAG;
pub use sig::tags::UCR_BG as SIG_UCR_BG_TAG;
pub use sig::tags::VCGT as SIG_VCGT_TAG;
pub use sig::tags::VIEWING_CONDITIONS as SIG_VIEWING_CONDITIONS_TAG;
pub use sig::tags::VIEWING_COND_DESC as SIG_VIEWING_COND_DESC_TAG;

// ICC Technology tag
pub use sig::technology::AM_DISPLAY as SIG_AM_DISPLAY;
pub use sig::technology::CRT_DISPLAY as SIG_CRT_DISPLAY;
pub use sig::technology::DIGITAL_CAMERA as SIG_DIGITAL_CAMERA;
pub use sig::technology::DIGITAL_CINEMA_PROJECTOR as SIG_DIGITAL_CINEMA_PROJECTOR;
pub use sig::technology::DIGITAL_MOTION_PICTURE_CAMERA as SIG_DIGITAL_MOTION_PICTURE_CAMERA;
pub use sig::technology::DYE_SUBLIMATION_PRINTER as SIG_DYE_SUBLIMATION_PRINTER;
pub use sig::technology::ELECTROPHOTOGRAPHIC_PRINTER as SIG_ELECTROPHOTOGRAPHIC_PRINTER;
pub use sig::technology::ELECTROSTATIC_PRINTER as SIG_ELECTROSTATIC_PRINTER;
pub use sig::technology::FILM_SCANNER as SIG_FILM_SCANNER;
pub use sig::technology::FILM_WRITER as SIG_FILM_WRITER;
pub use sig::technology::FLEXOGRAPHY as SIG_FLEXOGRAPHY;
pub use sig::technology::GRAVURE as SIG_GRAVURE;
pub use sig::technology::INK_JET_PRINTER as SIG_INK_JET_PRINTER;
pub use sig::technology::MOTION_PICTURE_FILM_RECORDER as SIG_MOTION_PICTURE_FILM_RECORDER;
pub use sig::technology::MOTION_PICTURE_FILM_SCANNER as SIG_MOTION_PICTURE_FILM_SCANNER;
pub use sig::technology::OFFSET_LITHOGRAPHY as SIG_OFFSET_LITHOGRAPHY;
pub use sig::technology::PHOTOGRAPHIC_PAPER_PRINTER as SIG_PHOTOGRAPHIC_PAPER_PRINTER;
pub use sig::technology::PHOTO_CD as SIG_PHOTO_CD;
pub use sig::technology::PHOTO_IMAGE_SETTER as SIG_PHOTO_IMAGE_SETTER;
pub use sig::technology::PM_DISPLAY as SIG_PM_DISPLAY;
pub use sig::technology::PROJECTION_TELEVISION as SIG_PROJECTION_TELEVISION;
pub use sig::technology::REFLECTIVE_SCANNER as SIG_REFLECTIVE_SCANNER;
pub use sig::technology::SILKSCREEN as SIG_SILKSCREEN;
pub use sig::technology::THERMAL_WAX_PRINTER as SIG_THERMAL_WAX_PRINTER;
pub use sig::technology::VIDEO_CAMERA as SIG_VIDEO_CAMERA;
pub use sig::technology::VIDEO_MONITOR as SIG_VIDEO_MONITOR;

// ICC Color spaces
pub use sig::colorspace::CMY as SIG_CMY_DATA;
pub use sig::colorspace::CMYK as SIG_CMYK_DATA;
pub use sig::colorspace::COLOR1 as SIG_1_COLOR_DATA;
pub use sig::colorspace::COLOR10 as SIG_10_COLOR_DATA;
pub use sig::colorspace::COLOR11 as SIG_11_COLOR_DATA;
pub use sig::colorspace::COLOR12 as SIG_12_COLOR_DATA;
pub use sig::colorspace::COLOR13 as SIG_13_COLOR_DATA;
pub use sig::colorspace::COLOR14 as SIG_14_COLOR_DATA;
pub use sig::colorspace::COLOR15 as SIG_15_COLOR_DATA;
pub use sig::colorspace::COLOR2 as SIG_2_COLOR_DATA;
pub use sig::colorspace::COLOR3 as SIG_3_COLOR_DATA;
pub use sig::colorspace::COLOR4 as SIG_4_COLOR_DATA;
pub use sig::colorspace::COLOR5 as SIG_5_COLOR_DATA;
pub use sig::colorspace::COLOR6 as SIG_6_COLOR_DATA;
pub use sig::colorspace::COLOR7 as SIG_7_COLOR_DATA;
pub use sig::colorspace::COLOR8 as SIG_8_COLOR_DATA;
pub use sig::colorspace::COLOR9 as SIG_9_COLOR_DATA;
pub use sig::colorspace::GRAY as SIG_GRAY_DATA;
pub use sig::colorspace::HLS as SIG_HLS_DATA;
pub use sig::colorspace::HSV as SIG_HSV_DATA;
pub use sig::colorspace::LAB as SIG_LAB_DATA;
pub use sig::colorspace::LUV as SIG_LUV_DATA;
pub use sig::colorspace::LUVK as SIG_LUVK_DATA;
pub use sig::colorspace::MCH1 as SIG_MCH1_DATA;
pub use sig::colorspace::MCH2 as SIG_MCH2_DATA;
pub use sig::colorspace::MCH3 as SIG_MCH3_DATA;
pub use sig::colorspace::MCH4 as SIG_MCH4_DATA;
pub use sig::colorspace::MCH5 as SIG_MCH5_DATA;
pub use sig::colorspace::MCH6 as SIG_MCH6_DATA;
pub use sig::colorspace::MCH7 as SIG_MCH7_DATA;
pub use sig::colorspace::MCH8 as SIG_MCH8_DATA;
pub use sig::colorspace::MCH9 as SIG_MCH9_DATA;
pub use sig::colorspace::MCHA as SIG_MCHA_DATA;
pub use sig::colorspace::MCHB as SIG_MCHB_DATA;
pub use sig::colorspace::MCHC as SIG_MCHC_DATA;
pub use sig::colorspace::MCHD as SIG_MCHD_DATA;
pub use sig::colorspace::MCHE as SIG_MCHE_DATA;
pub use sig::colorspace::MCHF as SIG_MCHF_DATA;
pub use sig::colorspace::NAMED as SIG_NAMED_DATA;
pub use sig::colorspace::RGB as SIG_RGB_DATA;
pub use sig::colorspace::XYZ as SIG_XYZ_DATA;
pub use sig::colorspace::YCBCR as SIG_YCBCR_DATA;
pub use sig::colorspace::YXY as SIG_YXY_DATA;

// ICC Profile Class
pub use sig::class::ABSTRACT as SIG_ABSTRACT_CLASS;
pub use sig::class::COLOR_SPACE as SIG_COLOR_SPACE_CLASS;
pub use sig::class::DISPLAY as SIG_DISPLAY_CLASS;
pub use sig::class::INPUT as SIG_INPUT_CLASS;
pub use sig::class::LINK as SIG_LINK_CLASS;
pub use sig::class::NAMED_COLOR as SIG_NAMED_COLOR_CLASS;
pub use sig::class::OUTPUT as SIG_OUTPUT_CLASS;

// ICC Platforms
pub use sig::platform::MACINTOSH as SIG_MACINTOSH;
pub use sig::platform::MICROSOFT as SIG_MICROSOFT;
pub use sig::platform::SGI as SIG_SGI;
pub use sig::platform::SOLARIS as SIG_SOLARIS;
pub use sig::platform::TALIGENT as SIG_TALIGENT;
pub use sig::platform::UNICES as SIG_UNICES;

// Reference gamut
pub use sig::PERCEPTUAL_REFERENCE_MEDIUM_GAMUT as SIG_PERCEPTUAL_REFERENCE_MEDIUM_GAMUT;

// For SIG_COLORIMETRIC_INTENT_IMAGE_STATE_TAG
pub use sig::colorimetric_intent::FOCAL_PLANE_COLORIMETRY_ESTIMATES as SIG_FOCAL_PLANE_COLORIMETRY_ESTIMATES;
pub use sig::colorimetric_intent::REFLECTION_HARDCOPY_ORIGINAL_COLORIMETRY as SIG_REFLECTION_HARDCOPY_ORIGINAL_COLORIMETRY;
pub use sig::colorimetric_intent::REFLECTION_PRINT_OUTPUT_COLORIMETRY as SIG_REFLECTION_PRINT_OUTPUT_COLORIMETRY;
pub use sig::colorimetric_intent::SCENE_APPEARANCE_ESTIMATES as SIG_SCENE_APPEARANCE_ESTIMATES;
pub use sig::colorimetric_intent::SCENE_COLORIMETRY_ESTIMATES as SIG_SCENE_COLORIMETRY_ESTIMATES;

// Multi process elements types
pub use sig::mpe_stage::BACS as SIG_BACS_ELEM_TYPE;
pub use sig::mpe_stage::CLIP_NEGATIVES as SIG_CLIP_NEGATIVES_ELEM_TYPE;
pub use sig::mpe_stage::CLUT as SIG_CLUT_ELEM_TYPE;
pub use sig::mpe_stage::CURVE_SET as SIG_CURVE_SET_ELEM_TYPE;
pub use sig::mpe_stage::EACS as SIG_EACS_ELEM_TYPE;
pub use sig::mpe_stage::FLOAT_PCS_2_LAB as SIG_FLOAT_PCS_2_LAB;
pub use sig::mpe_stage::FLOAT_PCS_2_XYZ as SIG_FLOAT_PCS_2_XYZ;
pub use sig::mpe_stage::IDENTITY as SIG_IDENTITY_ELEM_TYPE;
pub use sig::mpe_stage::LAB_2_FLOAT_PCS as SIG_LAB_2_FLOAT_PCS;
pub use sig::mpe_stage::LAB_2_XYZ as SIG_LAB_2_XYZ_ELEM_TYPE;
pub use sig::mpe_stage::LAB_V2_TO_V4 as SIG_LAB_V2_TO_V4;
pub use sig::mpe_stage::LAB_V4_TO_V2 as SIG_LAB_V4_TO_V2;
pub use sig::mpe_stage::MATRIX as SIG_MATRIX_ELEM_TYPE;
pub use sig::mpe_stage::NAMED_COLOR as SIG_NAMED_COLOR_ELEM_TYPE;
pub use sig::mpe_stage::XYZ_2_FLOAT_PCS as SIG_XYZ_2_FLOAT_PCS;
pub use sig::mpe_stage::XYZ_2_LAB as SIG_XYZ_2_LAB_ELEM_TYPE;

// Types of CurveElements
pub use sig::curve_segment::FORMULA as SIG_FORMULA_CURVE_SEG;
pub use sig::curve_segment::SAMPLED as SIG_SAMPLED_CURVE_SEG;
pub use sig::curve_segment::SEGMENTED as SIG_SEGMENTED_CURVE;

// Used in ResponseCurveType
pub use sig::response_curve::DN as SIG_DN;
pub use sig::response_curve::DNN as SIG_DNN;
pub use sig::response_curve::DNNP as SIG_DNNP;
pub use sig::response_curve::DNP as SIG_DNP;
pub use sig::response_curve::STATUS_A as SIG_STATUS_A;
pub use sig::response_curve::STATUS_E as SIG_STATUS_E;
pub use sig::response_curve::STATUS_I as SIG_STATUS_I;
pub use sig::response_curve::STATUS_M as SIG_STATUS_M;
pub use sig::response_curve::STATUS_T as SIG_STATUS_T;

// Device attributes, currently defined values correspond to the low 4 bytes
// of the 8 byte attribute quantity
pub use device_attribute::{GLOSSY, MATTE, REFLECTIVE, TRANSPARENCY};

// Common structures in ICC tags
pub use types::Data as ICCData;

// ICC date time
pub use types::DateTime as DateTimeNumber;

// ICC XYZ
pub use types::EncodedXYZ as EncodedXYZNumber;

// Profile ID as computed by MD5 algorithm
pub use types::ProfileID;

// ICC profile internal base types
pub use types::profile::Header as ICCHeader;

// ICC base tag
pub use types::tag::Base as TagBase;

// A tag entry in directory
pub use types::tag::directory::Entry as TagEntry;

pub use crate::MAX_CHANNELS;

// Format of pixel is defined by one cmsUInt32Number, using bit fields as follows
//
//                               2                1          0
//                        4 3 2 10987 6 5 4 3 2 1 098 7654 321
//                        M A O TTTTT U Y F P X S EEE CCCC BBB
//
//            M: Premultiplied alpha (only works when extra samples is 1)
//            A: Floating point -- With this flag we can differentiate 16 bits as float and as int
//            O: Optimized -- previous optimization already returns the final 8-bit value
//            T: Pixeltype
//            F: Flavor  0=MinIsBlack(Chocolate) 1=MinIsWhite(Vanilla)
//            P: Planar? 0=Chunky, 1=Planar
//            X: swap 16 bps endianness?
//            S: Do swap? ie, BGR, KYMC
//            E: Extra samples
//            C: Channels (Samples per pixel)
//            B: bytes per sample
//            Y: Swap first - changes ABGR to BGRA and KCMY to CMYK

pub use types::format::{
    bytes_sh, channels_sh, colorspace_sh, doswap_sh, endian16_sh, extra_sh, flavor_sh, float_sh,
    optimized_sh, planar_sh, premul_sh, swapfirst_sh,
};

// These macros unpack format specifiers into integers
macro_rules! t_premul {
    ($m:ident) => {
        (($m) >> 23) & 1
    };
}
macro_rules! t_float {
    ($a:ident) => {
        (($a) >> 22) & 1
    };
}
macro_rules! t_optimized {
    ($o:ident) => {
        (($o) >> 21) & 1
    };
}
macro_rules! t_colorspace {
    ($s:ident) => {
        (($s) >> 16) & 31
    };
}
macro_rules! t_swapfirst {
    ($s:ident) => {
        (($s) >> 14) & 1
    };
}
macro_rules! t_flavor {
    ($s:ident) => {
        (($s) >> 13) & 1
    };
}
macro_rules! t_planar {
    ($p:ident) => {
        (($p) >> 12) & 1
    };
}
macro_rules! t_endian16 {
    ($e:ident) => {
        (($e) >> 11) & 1
    };
}
macro_rules! t_doswap {
    ($e:ident) => {
        (($e) >> 10) & 1
    };
}
macro_rules! t_extra {
    ($e:ident) => {
        (($e) >> 7) & 7
    };
}
macro_rules! t_channels {
    ($c:ident) => {
        (($c) >> 3) & 15
    };
}
macro_rules! t_bytes {
    ($b:ident) => {
        ($b) & 7
    };
}

// // Pixel types
pub use types::format::pixel_type::ANY as PT_ANY;
pub use types::format::pixel_type::CMY as PT_CMY;
pub use types::format::pixel_type::CMYK as PT_CMYK;
pub use types::format::pixel_type::GRAY as PT_GRAY;
pub use types::format::pixel_type::HLS as PT_HLS;
pub use types::format::pixel_type::HSV as PT_HSV;
pub use types::format::pixel_type::LAB as PT_LAB;
pub use types::format::pixel_type::LAB_V2 as PT_LAB_V2;
pub use types::format::pixel_type::MCH1 as PT_MCH1;
pub use types::format::pixel_type::MCH10 as PT_MCH10;
pub use types::format::pixel_type::MCH11 as PT_MCH11;
pub use types::format::pixel_type::MCH12 as PT_MCH12;
pub use types::format::pixel_type::MCH13 as PT_MCH13;
pub use types::format::pixel_type::MCH14 as PT_MCH14;
pub use types::format::pixel_type::MCH15 as PT_MCH15;
pub use types::format::pixel_type::MCH2 as PT_MCH2;
pub use types::format::pixel_type::MCH3 as PT_MCH3;
pub use types::format::pixel_type::MCH4 as PT_MCH4;
pub use types::format::pixel_type::MCH5 as PT_MCH5;
pub use types::format::pixel_type::MCH6 as PT_MCH6;
pub use types::format::pixel_type::MCH7 as PT_MCH7;
pub use types::format::pixel_type::MCH8 as PT_MCH8;
pub use types::format::pixel_type::MCH9 as PT_MCH9;
pub use types::format::pixel_type::RGB as PT_RGB;
pub use types::format::pixel_type::XYZ as PT_XYZ;
pub use types::format::pixel_type::YCB_CR as PT_YCB_CR;
pub use types::format::pixel_type::YUV as PT_YUV;
pub use types::format::pixel_type::YUVK as PT_YUVK;
pub use types::format::pixel_type::YXY as PT_YXY;

// Some (not all!) representations

pub const TYPE_GRAY_8: u32 = colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(1);
pub const TYPE_GRAY_8_REV: u32 =
    colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(1) | flavor_sh(1);
pub const TYPE_GRAY_16: u32 = colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2);
pub const TYPE_GRAY_16_REV: u32 =
    colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2) | flavor_sh(1);
pub const TYPE_GRAY_16_SE: u32 =
    colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_GRAYA_8: u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1);
pub const TYPE_GRAYA_8_PREMUL: u32 =
    colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1) | premul_sh(1);
pub const TYPE_GRAYA_16: u32 = colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2);
pub const TYPE_GRAYA_16_PREMUL: u32 =
    colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | premul_sh(1);
pub const TYPE_GRAYA_16_SE: u32 =
    colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_GRAYA_8_PLANAR: u32 =
    colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(1) | planar_sh(1);
pub const TYPE_GRAYA_16_PLANAR: u32 =
    colorspace_sh(PT_GRAY) | extra_sh(1) | channels_sh(1) | bytes_sh(2) | planar_sh(1);

pub const TYPE_RGB_8: u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1);
pub const TYPE_RGB_8_PLANAR: u32 =
    colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_BGR_8: u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_BGR_8_PLANAR: u32 =
    colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(1) | doswap_sh(1) | planar_sh(1);
pub const TYPE_RGB_16: u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2);
pub const TYPE_RGB_16_PLANAR: u32 =
    colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_RGB_16_SE: u32 =
    colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_BGR_16: u32 = colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_BGR_16_PLANAR: u32 =
    colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | planar_sh(1);
pub const TYPE_BGR_16_SE: u32 =
    colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);

pub const TYPE_RGBA_8: u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1);
pub const TYPE_RGBA_8_PREMUL: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | premul_sh(1);
pub const TYPE_RGBA_8_PLANAR: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_RGBA_16: u32 = colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2);
pub const TYPE_RGBA_16_PREMUL: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | premul_sh(1);
pub const TYPE_RGBA_16_PLANAR: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_RGBA_16_SE: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

pub const TYPE_ARGB_8: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | swapfirst_sh(1);
pub const TYPE_ARGB_8_PREMUL: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(1)
    | swapfirst_sh(1)
    | premul_sh(1);
pub const TYPE_ARGB_8_PLANAR: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(1)
    | swapfirst_sh(1)
    | planar_sh(1);
pub const TYPE_ARGB_16: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | swapfirst_sh(1);
pub const TYPE_ARGB_16_PREMUL: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | swapfirst_sh(1)
    | premul_sh(1);

pub const TYPE_ABGR_8: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_ABGR_8_PREMUL: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(1)
    | doswap_sh(1)
    | premul_sh(1);
pub const TYPE_ABGR_8_PLANAR: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(1)
    | doswap_sh(1)
    | planar_sh(1);
pub const TYPE_ABGR_16: u32 =
    colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_ABGR_16_PREMUL: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | doswap_sh(1)
    | premul_sh(1);
pub const TYPE_ABGR_16_PLANAR: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | doswap_sh(1)
    | planar_sh(1);
pub const TYPE_ABGR_16_SE: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | doswap_sh(1)
    | endian16_sh(1);

pub const TYPE_BGRA_8: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(1)
    | doswap_sh(1)
    | swapfirst_sh(1);
pub const TYPE_BGRA_8_PREMUL: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(1)
    | doswap_sh(1)
    | swapfirst_sh(1)
    | premul_sh(1);
pub const TYPE_BGRA_8_PLANAR: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(1)
    | doswap_sh(1)
    | swapfirst_sh(1)
    | planar_sh(1);
pub const TYPE_BGRA_16: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | doswap_sh(1)
    | swapfirst_sh(1);
pub const TYPE_BGRA_16_PREMUL: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | doswap_sh(1)
    | swapfirst_sh(1)
    | premul_sh(1);
pub const TYPE_BGRA_16_SE: u32 = colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | endian16_sh(1)
    | doswap_sh(1)
    | swapfirst_sh(1);

pub const TYPE_CMY_8: u32 = colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(1);
pub const TYPE_CMY_8_PLANAR: u32 =
    colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_CMY_16: u32 = colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(2);
pub const TYPE_CMY_16_PLANAR: u32 =
    colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_CMY_16_SE: u32 =
    colorspace_sh(PT_CMY) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

pub const TYPE_CMYK_8: u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1);
pub const TYPE_CMYKA_8: u32 = colorspace_sh(PT_CMYK) | extra_sh(1) | channels_sh(4) | bytes_sh(1);
pub const TYPE_CMYK_8_REV: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | flavor_sh(1);
pub const TYPE_YUVK_8: u32 = TYPE_CMYK_8_REV;
pub const TYPE_CMYK_8_PLANAR: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | planar_sh(1);
pub const TYPE_CMYK_16: u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2);
pub const TYPE_CMYK_16_REV: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | flavor_sh(1);
pub const TYPE_YUVK_16: u32 = TYPE_CMYK_16_REV;
pub const TYPE_CMYK_16_PLANAR: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | planar_sh(1);
pub const TYPE_CMYK_16_SE: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | endian16_sh(1);

pub const TYPE_KYMC_8: u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC_16: u32 = colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC_16_SE: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);

pub const TYPE_KCMY_8: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | swapfirst_sh(1);
pub const TYPE_KCMY_8_REV: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(1) | flavor_sh(1) | swapfirst_sh(1);
pub const TYPE_KCMY_16: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | swapfirst_sh(1);
pub const TYPE_KCMY_16_REV: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | flavor_sh(1) | swapfirst_sh(1);
pub const TYPE_KCMY_16_SE: u32 =
    colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2) | endian16_sh(1) | swapfirst_sh(1);

pub const TYPE_CMYK5_8: u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(1);
pub const TYPE_CMYK5_16: u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2);
pub const TYPE_CMYK5_16_SE: u32 =
    colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC5_8: u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC5_16: u32 = colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC5_16_SE: u32 =
    colorspace_sh(PT_MCH5) | channels_sh(5) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK6_8: u32 = colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(1);
pub const TYPE_CMYK6_8_PLANAR: u32 =
    colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(1) | planar_sh(1);
pub const TYPE_CMYK6_16: u32 = colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(2);
pub const TYPE_CMYK6_16_PLANAR: u32 =
    colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(2) | planar_sh(1);
pub const TYPE_CMYK6_16_SE: u32 =
    colorspace_sh(PT_MCH6) | channels_sh(6) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_CMYK7_8: u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(1);
pub const TYPE_CMYK7_16: u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2);
pub const TYPE_CMYK7_16_SE: u32 =
    colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC7_8: u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC7_16: u32 = colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC7_16_SE: u32 =
    colorspace_sh(PT_MCH7) | channels_sh(7) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK8_8: u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(1);
pub const TYPE_CMYK8_16: u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2);
pub const TYPE_CMYK8_16_SE: u32 =
    colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC8_8: u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC8_16: u32 = colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC8_16_SE: u32 =
    colorspace_sh(PT_MCH8) | channels_sh(8) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK9_8: u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(1);
pub const TYPE_CMYK9_16: u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2);
pub const TYPE_CMYK9_16_SE: u32 =
    colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC9_8: u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC9_16: u32 = colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC9_16_SE: u32 =
    colorspace_sh(PT_MCH9) | channels_sh(9) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK10_8: u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(1);
pub const TYPE_CMYK10_16: u32 = colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2);
pub const TYPE_CMYK10_16_SE: u32 =
    colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC10_8: u32 =
    colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC10_16: u32 =
    colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC10_16_SE: u32 =
    colorspace_sh(PT_MCH10) | channels_sh(10) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK11_8: u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(1);
pub const TYPE_CMYK11_16: u32 = colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2);
pub const TYPE_CMYK11_16_SE: u32 =
    colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC11_8: u32 =
    colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC11_16: u32 =
    colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC11_16_SE: u32 =
    colorspace_sh(PT_MCH11) | channels_sh(11) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);
pub const TYPE_CMYK12_8: u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(1);
pub const TYPE_CMYK12_16: u32 = colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2);
pub const TYPE_CMYK12_16_SE: u32 =
    colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2) | endian16_sh(1);
pub const TYPE_KYMC12_8: u32 =
    colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(1) | doswap_sh(1);
pub const TYPE_KYMC12_16: u32 =
    colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_KYMC12_16_SE: u32 =
    colorspace_sh(PT_MCH12) | channels_sh(12) | bytes_sh(2) | doswap_sh(1) | endian16_sh(1);

// // Colorimetric
pub const TYPE_XYZ_16: u32 = colorspace_sh(PT_XYZ) | channels_sh(3) | bytes_sh(2);
pub const TYPE_LAB_8: u32 = colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(1);
pub const TYPE_LABV2_8: u32 = colorspace_sh(PT_LAB_V2) | channels_sh(3) | bytes_sh(1);

pub const TYPE_ALAB_8: u32 =
    colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(1) | extra_sh(1) | swapfirst_sh(1);
pub const TYPE_ALABV2_8: u32 =
    colorspace_sh(PT_LAB_V2) | channels_sh(3) | bytes_sh(1) | extra_sh(1) | swapfirst_sh(1);
pub const TYPE_LAB_16: u32 = colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(2);
pub const TYPE_LABV2_16: u32 = colorspace_sh(PT_LAB_V2) | channels_sh(3) | bytes_sh(2);
pub const TYPE_YXY_16: u32 = colorspace_sh(PT_YXY) | channels_sh(3) | bytes_sh(2);

// // YCBCR
pub const TYPE_YCBCR_8: u32 = colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(1);
pub const TYPE_YCBCR_8_PLANAR: u32 =
    colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_YCBCR_16: u32 = colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(2);
pub const TYPE_YCBCR_16_PLANAR: u32 =
    colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_YCBCR_16_SE: u32 =
    colorspace_sh(PT_YCB_CR) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// // YUV
pub const TYPE_YUV_8: u32 = colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(1);
pub const TYPE_YUV_8_PLANAR: u32 =
    colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_YUV_16: u32 = colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(2);
pub const TYPE_YUV_16_PLANAR: u32 =
    colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_YUV_16_SE: u32 =
    colorspace_sh(PT_YUV) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// // HLS
pub const TYPE_HLS_8: u32 = colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(1);
pub const TYPE_HLS_8_PLANAR: u32 =
    colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_HLS_16: u32 = colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(2);
pub const TYPE_HLS_16_PLANAR: u32 =
    colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_HLS_16_SE: u32 =
    colorspace_sh(PT_HLS) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// // HSV
pub const TYPE_HSV_8: u32 = colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(1);
pub const TYPE_HSV_8_PLANAR: u32 =
    colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(1) | planar_sh(1);
pub const TYPE_HSV_16: u32 = colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(2);
pub const TYPE_HSV_16_PLANAR: u32 =
    colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(2) | planar_sh(1);
pub const TYPE_HSV_16_SE: u32 =
    colorspace_sh(PT_HSV) | channels_sh(3) | bytes_sh(2) | endian16_sh(1);

// // Named color index. Only 16 bits is allowed (don't check colorspace)
pub const TYPE_NAMED_COLOR_INDEX: u32 = channels_sh(1) | bytes_sh(2);

// // Float formatters.
pub const TYPE_XYZ_FLT: u32 = float_sh(1) | colorspace_sh(PT_XYZ) | channels_sh(3) | bytes_sh(4);
pub const TYPE_LAB_FLT: u32 = float_sh(1) | colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(4);
pub const TYPE_LABA_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_LAB) | extra_sh(1) | channels_sh(3) | bytes_sh(4);
pub const TYPE_GRAY_FLT: u32 = float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(4);
pub const TYPE_GRAYA_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(4) | extra_sh(1);
pub const TYPE_GRAYA_FLT_PREMUL: u32 = float_sh(1)
    | colorspace_sh(PT_GRAY)
    | channels_sh(1)
    | bytes_sh(4)
    | extra_sh(1)
    | premul_sh(1);
pub const TYPE_RGB_FLT: u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(4);

pub const TYPE_RGBA_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4);
pub const TYPE_RGBA_FLT_PREMUL: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | premul_sh(1);
pub const TYPE_ARGB_FLT: u32 = float_sh(1)
    | colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(4)
    | swapfirst_sh(1);
pub const TYPE_ARGB_FLT_PREMUL: u32 = float_sh(1)
    | colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(4)
    | swapfirst_sh(1)
    | premul_sh(1);
pub const TYPE_BGR_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(4) | doswap_sh(1);
pub const TYPE_BGRA_FLT: u32 = float_sh(1)
    | colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(4)
    | doswap_sh(1)
    | swapfirst_sh(1);
pub const TYPE_BGRA_FLT_PREMUL: u32 = float_sh(1)
    | colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(4)
    | doswap_sh(1)
    | swapfirst_sh(1)
    | premul_sh(1);
pub const TYPE_ABGR_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(4) | doswap_sh(1);
pub const TYPE_ABGR_FLT_PREMUL: u32 = float_sh(1)
    | colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(4)
    | doswap_sh(1)
    | premul_sh(1);

pub const TYPE_CMYK_FLT: u32 = float_sh(1) | colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(4);

// // Floating point formatters.
// // NOTE THAT 'BYTES' FIELD IS SET TO ZERO ON DLB because 8 bytes overflows the bitfield
pub const TYPE_XYZ_DBL: u32 = float_sh(1) | colorspace_sh(PT_XYZ) | channels_sh(3) | bytes_sh(0);
pub const TYPE_LAB_DBL: u32 = float_sh(1) | colorspace_sh(PT_LAB) | channels_sh(3) | bytes_sh(0);
pub const TYPE_GRAY_DBL: u32 = float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(0);
pub const TYPE_RGB_DBL: u32 = float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(0);
pub const TYPE_BGR_DBL: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(0) | doswap_sh(1);
pub const TYPE_CMYK_DBL: u32 = float_sh(1) | colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(0);

// // IEEE 754-2008 "half"
pub const TYPE_GRAY_HALF_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_GRAY) | channels_sh(1) | bytes_sh(2);
pub const TYPE_RGB_HALF_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2);
pub const TYPE_RGBA_HALF_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | extra_sh(1) | channels_sh(3) | bytes_sh(2);
pub const TYPE_CMYK_HALF_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_CMYK) | channels_sh(4) | bytes_sh(2);

pub const TYPE_ARGB_HALF_FLT: u32 = float_sh(1)
    | colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | swapfirst_sh(1);
pub const TYPE_BGR_HALF_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);
pub const TYPE_BGRA_HALF_FLT: u32 = float_sh(1)
    | colorspace_sh(PT_RGB)
    | extra_sh(1)
    | channels_sh(3)
    | bytes_sh(2)
    | doswap_sh(1)
    | swapfirst_sh(1);
pub const TYPE_ABGR_HALF_FLT: u32 =
    float_sh(1) | colorspace_sh(PT_RGB) | channels_sh(3) | bytes_sh(2) | doswap_sh(1);

pub use types::JCh;
pub use types::LCh as CIELCh;
pub use types::Lab as CIELab;
pub use types::XYYTriple as CIExyYTRIPLE;
pub use types::XYZTriple as CIEXYZTRIPLE;
pub use types::XYY as CIExyY;
pub use types::XYZ as CIEXYZ;

// // Illuminant types for structs below
pub use crate::illuminant_type::A as ILLUMINANT_TYPE_A;
pub use crate::illuminant_type::D50 as ILLUMINANT_TYPE_D50;
pub use crate::illuminant_type::D55 as ILLUMINANT_TYPE_D55;
pub use crate::illuminant_type::D65 as ILLUMINANT_TYPE_D65;
pub use crate::illuminant_type::D93 as ILLUMINANT_TYPE_D93;
pub use crate::illuminant_type::E as ILLUMINANT_TYPE_E;
pub use crate::illuminant_type::F2 as ILLUMINANT_TYPE_F2;
pub use crate::illuminant_type::F8 as ILLUMINANT_TYPE_F8;
pub use crate::illuminant_type::UNKNOWN as ILLUMINANT_TYPE_UNKNOWN;

pub use types::MeasurementConditions as ICCMeasurementConditions;
pub use types::VideoSignalType;
pub use types::ViewingConditions as ICCViewingConditions;

// Get LittleCMS version (for shared objects) -----------------------------------------------------------------------------

// CMSAPI int               CMSEXPORT cmsGetEncodedCMMversion(void);

// Support of non-standard functions --------------------------------------------------------------------------------------

// CMSAPI int               CMSEXPORT cmsstrcasecmp(const char* s1, const char* s2);
// CMSAPI long int          CMSEXPORT cmsfilelength(FILE* f);

// Context handling --------------------------------------------------------------------------------------------------------

// Each context holds its owns globals and its own plug-ins. There is a global context with the id = 0 for lecacy compatibility
// though using the global context is not recommended. Proper context handling makes lcms more thread-safe.

pub type Context = Option<crate::Context>;

// CMSAPI cmsContext       CMSEXPORT cmsCreateContext(void* Plugin, void* UserData);
// CMSAPI void             CMSEXPORT cmsDeleteContext(cmsContext ContextID);
// CMSAPI cmsContext       CMSEXPORT cmsDupContext(cmsContext ContextID, void* NewUserData);
// CMSAPI void*            CMSEXPORT cmsGetContextUserData(cmsContext ContextID);

// Plug-In registering  --------------------------------------------------------------------------------------------------

// CMSAPI cmsBool           CMSEXPORT cmsPlugin(void* Plugin);
// CMSAPI cmsBool           CMSEXPORT cmsPluginTHR(cmsContext ContextID, void* Plugin);
// CMSAPI void              CMSEXPORT cmsUnregisterPlugins(void);
// CMSAPI void              CMSEXPORT cmsUnregisterPluginsTHR(cmsContext ContextID);

// Error logging ----------------------------------------------------------------------------------------------------------

// There is no error handling at all. When a function fails, it returns proper value.
// For example, all create functions does return NULL on failure. Other may return FALSE.
// It may be interesting, for the developer, to know why the function is failing.
// for that reason, lcms2 does offer a logging function. This function will get
// an ENGLISH string with some clues on what is going wrong. You can show this
// info to the end user if you wish, or just create some sort of log on disk.
// The logging function should NOT terminate the program, as this obviously can leave
// unfreed resources. It is the programmer's responsibility to check each function
// return code to make sure it didn't fail.

pub const ERROR_UNDEFINED: u32 = 0;
pub const ERROR_FILE: u32 = 1;
pub const ERROR_RANGE: u32 = 2;
pub const ERROR_INTERNAL: u32 = 3;
pub const ERROR_NULL: u32 = 4;
pub const ERROR_READ: u32 = 5;
pub const ERROR_SEEK: u32 = 6;
pub const ERROR_WRITE: u32 = 7;
pub const ERROR_UNKNOWN_EXTENSION: u32 = 8;
pub const ERROR_COLORSPACE_CHECK: u32 = 9;
pub const ERROR_ALREADY_DEFINED: u32 = 10;
pub const ERROR_BAD_SIGNATURE: u32 = 11;
pub const ERROR_CORRUPTION_DETECTED: u32 = 12;
pub const ERROR_NOT_SUITABLE: u32 = 13;

// Error logger is called with the ContextID when a message is raised. This gives the
// chance to know which thread is responsible of the warning and any environment associated
// with it. Non-multithreading applications may safely ignore this parameter.
// Note that under certain special circumstances, ContextID may be NULL.
pub type LogErrorHandlerFunction = fn(ContextID: Context, ErrorCode: u32, Text: str);

// Allows user to set any specific logger
// CMSAPI void              CMSEXPORT cmsSetLogErrorHandler(cmsLogErrorHandlerFunction Fn);
// CMSAPI void              CMSEXPORT cmsSetLogErrorHandlerTHR(cmsContext ContextID, cmsLogErrorHandlerFunction Fn);

// Conversions --------------------------------------------------------------------------------------------------------------

// Returns pointers to constant structs
// CMSAPI const cmsCIEXYZ*  CMSEXPORT cmsD50_XYZ(void);
// CMSAPI const cmsCIExyY*  CMSEXPORT cmsD50_xyY(void);

// Colorimetric space conversions
// CMSAPI void              CMSEXPORT cmsXYZ2xyY(cmsCIExyY* Dest, const cmsCIEXYZ* Source);
// CMSAPI void              CMSEXPORT cmsxyY2XYZ(cmsCIEXYZ* Dest, const cmsCIExyY* Source);
// CMSAPI void              CMSEXPORT cmsXYZ2Lab(const cmsCIEXYZ* WhitePoint, cmsCIELab* Lab, const cmsCIEXYZ* xyz);
// CMSAPI void              CMSEXPORT cmsLab2XYZ(const cmsCIEXYZ* WhitePoint, cmsCIEXYZ* xyz, const cmsCIELab* Lab);
// CMSAPI void              CMSEXPORT cmsLab2LCh(cmsCIELCh*LCh, const cmsCIELab* Lab);
// CMSAPI void              CMSEXPORT cmsLCh2Lab(cmsCIELab* Lab, const cmsCIELCh* LCh);

// Encoding /Decoding on PCS
// CMSAPI void              CMSEXPORT cmsLabEncoded2Float(cmsCIELab* Lab, const cmsUInt16Number wLab[3]);
// CMSAPI void              CMSEXPORT cmsLabEncoded2FloatV2(cmsCIELab* Lab, const cmsUInt16Number wLab[3]);
// CMSAPI void              CMSEXPORT cmsFloat2LabEncoded(cmsUInt16Number wLab[3], const cmsCIELab* Lab);
// CMSAPI void              CMSEXPORT cmsFloat2LabEncodedV2(cmsUInt16Number wLab[3], const cmsCIELab* Lab);
// CMSAPI void              CMSEXPORT cmsXYZEncoded2Float(cmsCIEXYZ* fxyz, const cmsUInt16Number XYZ[3]);
// CMSAPI void              CMSEXPORT cmsFloat2XYZEncoded(cmsUInt16Number XYZ[3], const cmsCIEXYZ* fXYZ);

// DeltaE metrics
// CMSAPI cmsFloat64Number  CMSEXPORT cmsDeltaE(const cmsCIELab* Lab1, const cmsCIELab* Lab2);
// CMSAPI cmsFloat64Number  CMSEXPORT cmsCIE94DeltaE(const cmsCIELab* Lab1, const cmsCIELab* Lab2);
// CMSAPI cmsFloat64Number  CMSEXPORT cmsBFDdeltaE(const cmsCIELab* Lab1, const cmsCIELab* Lab2);
// CMSAPI cmsFloat64Number  CMSEXPORT cmsCMCdeltaE(const cmsCIELab* Lab1, const cmsCIELab* Lab2, cmsFloat64Number l, cmsFloat64Number c);
// CMSAPI cmsFloat64Number  CMSEXPORT cmsCIE2000DeltaE(const cmsCIELab* Lab1, const cmsCIELab* Lab2, cmsFloat64Number Kl, cmsFloat64Number Kc, cmsFloat64Number Kh);

// Temperature <-> Chromaticity (Black body)
// CMSAPI cmsBool           CMSEXPORT cmsWhitePointFromTemp(cmsCIExyY* WhitePoint, cmsFloat64Number  TempK);
// CMSAPI cmsBool           CMSEXPORT cmsTempFromWhitePoint(cmsFloat64Number* TempK, const cmsCIExyY* WhitePoint);

// Chromatic adaptation
// CMSAPI cmsBool           CMSEXPORT cmsAdaptToIlluminant(cmsCIEXYZ* Result, const cmsCIEXYZ* SourceWhitePt,
//                                                                            const cmsCIEXYZ* Illuminant,
//                                                                            const cmsCIEXYZ* Value);

// CIECAM02 ---------------------------------------------------------------------------------------------------

// Viewing conditions. Please note those are CAM model viewing conditions, and not the ICC tag viewing
// conditions, which I'm naming cmsICCViewingConditions to make differences evident. Unfortunately, the tag
// cannot deal with surround La, Yb and D value so is basically useless to store CAM02 viewing conditions.

pub use types::viewing_conditions::cam02::surround::AVG as AVG_SURROUND;
pub use types::viewing_conditions::cam02::surround::CUTSHEET as CUTSHEET_SURROUND;
pub use types::viewing_conditions::cam02::surround::DARK as DARK_SURROUND;
pub use types::viewing_conditions::cam02::surround::DIM as DIM_SURROUND;

pub use types::viewing_conditions::cam02::D_CALCULATE;

pub use types::viewing_conditions::cam02::ViewingConditions;

// CMSAPI cmsHANDLE         CMSEXPORT cmsCIECAM02Init(cmsContext ContextID, const cmsViewingConditions* pVC);
// CMSAPI void              CMSEXPORT cmsCIECAM02Done(cmsHANDLE hModel);
// CMSAPI void              CMSEXPORT cmsCIECAM02Forward(cmsHANDLE hModel, const cmsCIEXYZ* pIn, cmsJCh* pOut);
// CMSAPI void              CMSEXPORT cmsCIECAM02Reverse(cmsHANDLE hModel, const cmsJCh* pIn,    cmsCIEXYZ* pOut);

// Tone curves -----------------------------------------------------------------------------------------

// This describes a curve segment. For a table of supported types, see the manual. User can increase the number of
// available types by using a proper plug-in. Parametric segments allow 10 parameters at most

pub use types::CurveSegment;

// The internal representation is none of your business.
// typedef struct _cms_curve_struct cmsToneCurve;

// CMSAPI cmsToneCurve*     CMSEXPORT cmsBuildSegmentedToneCurve(cmsContext ContextID, cmsUInt32Number nSegments, const cmsCurveSegment Segments[]);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsBuildParametricToneCurve(cmsContext ContextID, cmsInt32Number Type, const cmsFloat64Number Params[]);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsBuildGamma(cmsContext ContextID, cmsFloat64Number Gamma);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsBuildTabulatedToneCurve16(cmsContext ContextID, cmsUInt32Number nEntries, const cmsUInt16Number values[]);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsBuildTabulatedToneCurveFloat(cmsContext ContextID, cmsUInt32Number nEntries, const cmsFloat32Number values[]);
// CMSAPI void              CMSEXPORT cmsFreeToneCurve(cmsToneCurve* Curve);
// CMSAPI void              CMSEXPORT cmsFreeToneCurveTriple(cmsToneCurve* Curve[3]);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsDupToneCurve(const cmsToneCurve* Src);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsReverseToneCurve(const cmsToneCurve* InGamma);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsReverseToneCurveEx(cmsUInt32Number nResultSamples, const cmsToneCurve* InGamma);
// CMSAPI cmsToneCurve*     CMSEXPORT cmsJoinToneCurve(cmsContext ContextID, const cmsToneCurve* X,  const cmsToneCurve* Y, cmsUInt32Number nPoints);
// CMSAPI cmsBool           CMSEXPORT cmsSmoothToneCurve(cmsToneCurve* Tab, cmsFloat64Number lambda);
// CMSAPI cmsFloat32Number  CMSEXPORT cmsEvalToneCurveFloat(const cmsToneCurve* Curve, cmsFloat32Number v);
// CMSAPI cmsUInt16Number   CMSEXPORT cmsEvalToneCurve16(const cmsToneCurve* Curve, cmsUInt16Number v);
// CMSAPI cmsBool           CMSEXPORT cmsIsToneCurveMultisegment(const cmsToneCurve* InGamma);
// CMSAPI cmsBool           CMSEXPORT cmsIsToneCurveLinear(const cmsToneCurve* Curve);
// CMSAPI cmsBool           CMSEXPORT cmsIsToneCurveMonotonic(const cmsToneCurve* t);
// CMSAPI cmsBool           CMSEXPORT cmsIsToneCurveDescending(const cmsToneCurve* t);
// CMSAPI cmsInt32Number    CMSEXPORT cmsGetToneCurveParametricType(const cmsToneCurve* t);
// CMSAPI cmsFloat64Number  CMSEXPORT cmsEstimateGamma(const cmsToneCurve* t, cmsFloat64Number Precision);
// CMSAPI cmsFloat64Number* CMSEXPORT cmsGetToneCurveParams(const cmsToneCurve* t);

// Tone curve tabular estimation
// CMSAPI cmsUInt32Number         CMSEXPORT cmsGetToneCurveEstimatedTableEntries(const cmsToneCurve* t);
// CMSAPI const cmsUInt16Number*  CMSEXPORT cmsGetToneCurveEstimatedTable(const cmsToneCurve* t);

// Implements pipelines of multi-processing elements -------------------------------------------------------------

// Nothing to see here, move along
// typedef struct _cmsPipeline_struct cmsPipeline;
// typedef struct _cmsStage_struct cmsStage;

// Those are hi-level pipelines
// CMSAPI cmsPipeline*      CMSEXPORT cmsPipelineAlloc(cmsContext ContextID, cmsUInt32Number InputChannels, cmsUInt32Number OutputChannels);
// CMSAPI void              CMSEXPORT cmsPipelineFree(cmsPipeline* lut);
// CMSAPI cmsPipeline*      CMSEXPORT cmsPipelineDup(const cmsPipeline* Orig);

// CMSAPI cmsContext        CMSEXPORT cmsGetPipelineContextID(const cmsPipeline* lut);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsPipelineInputChannels(const cmsPipeline* lut);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsPipelineOutputChannels(const cmsPipeline* lut);

// CMSAPI cmsUInt32Number   CMSEXPORT cmsPipelineStageCount(const cmsPipeline* lut);
// CMSAPI cmsStage*         CMSEXPORT cmsPipelineGetPtrToFirstStage(const cmsPipeline* lut);
// CMSAPI cmsStage*         CMSEXPORT cmsPipelineGetPtrToLastStage(const cmsPipeline* lut);

// CMSAPI void              CMSEXPORT cmsPipelineEval16(const cmsUInt16Number In[], cmsUInt16Number Out[], const cmsPipeline* lut);
// CMSAPI void              CMSEXPORT cmsPipelineEvalFloat(const cmsFloat32Number In[], cmsFloat32Number Out[], const cmsPipeline* lut);
// CMSAPI cmsBool           CMSEXPORT cmsPipelineEvalReverseFloat(cmsFloat32Number Target[], cmsFloat32Number Result[], cmsFloat32Number Hint[], const cmsPipeline* lut);
// CMSAPI cmsBool           CMSEXPORT cmsPipelineCat(cmsPipeline* l1, const cmsPipeline* l2);
// CMSAPI cmsBool           CMSEXPORT cmsPipelineSetSaveAs8bitsFlag(cmsPipeline* lut, cmsBool On);

// Where to place/locate the stages in the pipeline chain
pub enum StageLoc {
    AtBegin,
    AtEnd,
}

// CMSAPI cmsBool           CMSEXPORT cmsPipelineInsertStage(cmsPipeline* lut, cmsStageLoc loc, cmsStage* mpe);
// CMSAPI void              CMSEXPORT cmsPipelineUnlinkStage(cmsPipeline* lut, cmsStageLoc loc, cmsStage** mpe);

// This function is quite useful to analyze the structure of a Pipeline and retrieve the Stage elements
// that conform the Pipeline. It should be called with the Pipeline, the number of expected elements and
// then a list of expected types followed with a list of double pointers to Stage elements. If
// the function founds a match with current pipeline, it fills the pointers and returns TRUE
// if not, returns FALSE without touching anything.
// CMSAPI cmsBool           CMSEXPORT cmsPipelineCheckAndRetreiveStages(const cmsPipeline* Lut, cmsUInt32Number n, ...);

// Matrix has double precision and CLUT has only float precision. That is because an ICC profile can encode
// matrices with far more precision that CLUTS
// CMSAPI cmsStage*         CMSEXPORT cmsStageAllocIdentity(cmsContext ContextID, cmsUInt32Number nChannels);
// CMSAPI cmsStage*         CMSEXPORT cmsStageAllocToneCurves(cmsContext ContextID, cmsUInt32Number nChannels, cmsToneCurve* const Curves[]);
// CMSAPI cmsStage*         CMSEXPORT cmsStageAllocMatrix(cmsContext ContextID, cmsUInt32Number Rows, cmsUInt32Number Cols, const cmsFloat64Number* Matrix, const cmsFloat64Number* Offset);

// CMSAPI cmsStage*         CMSEXPORT cmsStageAllocCLut16bit(cmsContext ContextID, cmsUInt32Number nGridPoints, cmsUInt32Number inputChan, cmsUInt32Number outputChan, const cmsUInt16Number* Table);
// CMSAPI cmsStage*         CMSEXPORT cmsStageAllocCLutFloat(cmsContext ContextID, cmsUInt32Number nGridPoints, cmsUInt32Number inputChan, cmsUInt32Number outputChan, const cmsFloat32Number* Table);

// CMSAPI cmsStage*         CMSEXPORT cmsStageAllocCLut16bitGranular(cmsContext ContextID, const cmsUInt32Number clutPoints[], cmsUInt32Number inputChan, cmsUInt32Number outputChan, const cmsUInt16Number* Table);
// CMSAPI cmsStage*         CMSEXPORT cmsStageAllocCLutFloatGranular(cmsContext ContextID, const cmsUInt32Number clutPoints[], cmsUInt32Number inputChan, cmsUInt32Number outputChan, const cmsFloat32Number* Table);

// CMSAPI cmsStage*         CMSEXPORT cmsStageDup(cmsStage* mpe);
// CMSAPI void              CMSEXPORT cmsStageFree(cmsStage* mpe);
// CMSAPI cmsStage*         CMSEXPORT cmsStageNext(const cmsStage* mpe);

// CMSAPI cmsUInt32Number   CMSEXPORT cmsStageInputChannels(const cmsStage* mpe);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsStageOutputChannels(const cmsStage* mpe);
// CMSAPI cmsStageSignature CMSEXPORT cmsStageType(const cmsStage* mpe);
// CMSAPI void*             CMSEXPORT cmsStageData(const cmsStage* mpe);
// CMSAPI cmsContext        CMSEXPORT cmsGetStageContextID(const cmsStage* mpe);

// Sampling
pub use crate::{Sampler16, SamplerFloat};

/// Use this flag to prevent changes being written to destination
pub use crate::SAMPLER_INSPECT;

// For CLUT only
// CMSAPI cmsBool           CMSEXPORT cmsStageSampleCLut16bit(cmsStage* mpe, cmsSAMPLER16 Sampler, void* Cargo, cmsUInt32Number dwFlags);
// CMSAPI cmsBool           CMSEXPORT cmsStageSampleCLutFloat(cmsStage* mpe, cmsSAMPLERFLOAT Sampler, void* Cargo, cmsUInt32Number dwFlags);

// Slicers
// CMSAPI cmsBool           CMSEXPORT cmsSliceSpace16(cmsUInt32Number nInputs, const cmsUInt32Number clutPoints[],
//                                                    cmsSAMPLER16 Sampler, void * Cargo);

// CMSAPI cmsBool           CMSEXPORT cmsSliceSpaceFloat(cmsUInt32Number nInputs, const cmsUInt32Number clutPoints[],
//                                                    cmsSAMPLERFLOAT Sampler, void * Cargo);

// Multilocalized Unicode management ---------------------------------------------------------------------------------------

// typedef struct _cms_MLU_struct cmsMLU;

pub use crate::{NO_COUNTRY, NO_LANGUAGE};

// CMSAPI cmsMLU*           CMSEXPORT cmsMLUalloc(cmsContext ContextID, cmsUInt32Number nItems);
// CMSAPI void              CMSEXPORT cmsMLUfree(cmsMLU* mlu);
// CMSAPI cmsMLU*           CMSEXPORT cmsMLUdup(const cmsMLU* mlu);

// CMSAPI cmsBool           CMSEXPORT cmsMLUsetASCII(cmsMLU* mlu,
//                                                   const char LanguageCode[3], const char CountryCode[3],
//                                                   const char* ASCIIString);
// CMSAPI cmsBool           CMSEXPORT cmsMLUsetWide(cmsMLU* mlu,
//                                                   const char LanguageCode[3], const char CountryCode[3],
//                                                   const wchar_t* WideString);

// CMSAPI cmsUInt32Number   CMSEXPORT cmsMLUgetASCII(const cmsMLU* mlu,
//                                                   const char LanguageCode[3], const char CountryCode[3],
//                                                   char* Buffer,    cmsUInt32Number BufferSize);

// CMSAPI cmsUInt32Number   CMSEXPORT cmsMLUgetWide(const cmsMLU* mlu,
//                                                  const char LanguageCode[3], const char CountryCode[3],
//                                                  wchar_t* Buffer, cmsUInt32Number BufferSize);

// CMSAPI cmsBool           CMSEXPORT cmsMLUgetTranslation(const cmsMLU* mlu,
//                                                          const char LanguageCode[3], const char CountryCode[3],
//                                                          char ObtainedLanguage[3], char ObtainedCountry[3]);

// CMSAPI cmsUInt32Number   CMSEXPORT cmsMLUtranslationsCount(const cmsMLU* mlu);

// CMSAPI cmsBool           CMSEXPORT cmsMLUtranslationsCodes(const cmsMLU* mlu,
//                                                              cmsUInt32Number idx,
//                                                              char LanguageCode[3],
//                                                              char CountryCode[3]);

// Undercolorremoval & black generation -------------------------------------------------------------------------------------

pub use types::UcrBg;

// Screening ----------------------------------------------------------------------------------------------------------------

pub use crate::screening_frequence::PRINTER_DEFAULT as PRINTER_DEFAULT_SCREENS;
pub use crate::screening_frequence::UNITS_LINES_CM as FREQUENCE_UNITS_LINES_CM;
pub use crate::screening_frequence::UNITS_LINES_INCH as FREQUENCE_UNITS_LINES_INCH;

pub use crate::screening_spot_shape::CROSS as SPOT_CROSS;
pub use crate::screening_spot_shape::DIAMOND as SPOT_DIAMOND;
pub use crate::screening_spot_shape::ELLIPSE as SPOT_ELLIPSE;
pub use crate::screening_spot_shape::LINE as SPOT_LINE;
pub use crate::screening_spot_shape::PRINTER_DEFAULT as SPOT_PRINTER_DEFAULT;
pub use crate::screening_spot_shape::ROUND as SPOT_ROUND;
pub use crate::screening_spot_shape::SQUARE as SPOT_SQUARE;
pub use crate::screening_spot_shape::UNKNOWN as SPOT_UNKNOWN;

pub use types::{Screening, ScreeningChannel};

// Named color -----------------------------------------------------------------------------------------------------------------

// typedef struct _cms_NAMEDCOLORLIST_struct cmsNAMEDCOLORLIST;

// CMSAPI cmsNAMEDCOLORLIST* CMSEXPORT cmsAllocNamedColorList(cmsContext ContextID,
//                                                            cmsUInt32Number n,
//                                                            cmsUInt32Number ColorantCount,
//                                                            const char* Prefix, const char* Suffix);

// CMSAPI void               CMSEXPORT cmsFreeNamedColorList(cmsNAMEDCOLORLIST* v);
// CMSAPI cmsNAMEDCOLORLIST* CMSEXPORT cmsDupNamedColorList(const cmsNAMEDCOLORLIST* v);
// CMSAPI cmsBool            CMSEXPORT cmsAppendNamedColor(cmsNAMEDCOLORLIST* v, const char* Name,
//                                                             cmsUInt16Number PCS[3],
//                                                             cmsUInt16Number Colorant[cmsMAXCHANNELS]);

// CMSAPI cmsUInt32Number    CMSEXPORT cmsNamedColorCount(const cmsNAMEDCOLORLIST* v);
// CMSAPI cmsInt32Number     CMSEXPORT cmsNamedColorIndex(const cmsNAMEDCOLORLIST* v, const char* Name);

// CMSAPI cmsBool            CMSEXPORT cmsNamedColorInfo(const cmsNAMEDCOLORLIST* NamedColorList, cmsUInt32Number nColor,
//                                                       char* Name,
//                                                       char* Prefix,
//                                                       char* Suffix,
//                                                       cmsUInt16Number* PCS,
//                                                       cmsUInt16Number* Colorant);

// Retrieve named color list from transform
// CMSAPI cmsNAMEDCOLORLIST* CMSEXPORT cmsGetNamedColorList(cmsHTRANSFORM xform);

// Profile sequence -----------------------------------------------------------------------------------------------------

// Profile sequence descriptor. Some fields come from profile sequence descriptor tag, others
// come from Profile Sequence Identifier Tag
pub use types::{PSeqDesc, Seq};

// CMSAPI cmsSEQ*           CMSEXPORT cmsAllocProfileSequenceDescription(cmsContext ContextID, cmsUInt32Number n);
// CMSAPI cmsSEQ*           CMSEXPORT cmsDupProfileSequenceDescription(const cmsSEQ* pseq);
// CMSAPI void              CMSEXPORT cmsFreeProfileSequenceDescription(cmsSEQ* pseq);

// Dictionaries --------------------------------------------------------------------------------------------------------

pub use types::DictionaryEntry as DICTentry;

// CMSAPI cmsHANDLE           CMSEXPORT cmsDictAlloc(cmsContext ContextID);
// CMSAPI void                CMSEXPORT cmsDictFree(cmsHANDLE hDict);
// CMSAPI cmsHANDLE           CMSEXPORT cmsDictDup(cmsHANDLE hDict);

// CMSAPI cmsBool             CMSEXPORT cmsDictAddEntry(cmsHANDLE hDict, const wchar_t* Name, const wchar_t* Value, const cmsMLU *DisplayName, const cmsMLU *DisplayValue);
// CMSAPI const cmsDICTentry* CMSEXPORT cmsDictGetEntryList(cmsHANDLE hDict);
// CMSAPI const cmsDICTentry* CMSEXPORT cmsDictNextEntry(const cmsDICTentry* e);

// Access to Profile data ----------------------------------------------------------------------------------------------
// CMSAPI cmsHPROFILE       CMSEXPORT cmsCreateProfilePlaceholder(cmsContext ContextID);

// CMSAPI cmsContext        CMSEXPORT cmsGetProfileContextID(cmsHPROFILE hProfile);
// CMSAPI cmsInt32Number    CMSEXPORT cmsGetTagCount(cmsHPROFILE hProfile);
// CMSAPI cmsTagSignature   CMSEXPORT cmsGetTagSignature(cmsHPROFILE hProfile, cmsUInt32Number n);
// CMSAPI cmsBool           CMSEXPORT cmsIsTag(cmsHPROFILE hProfile, cmsTagSignature sig);

// Read and write pre-formatted data
// CMSAPI void*             CMSEXPORT cmsReadTag(cmsHPROFILE hProfile, cmsTagSignature sig);
// CMSAPI cmsBool           CMSEXPORT cmsWriteTag(cmsHPROFILE hProfile, cmsTagSignature sig, const void* data);
// CMSAPI cmsBool           CMSEXPORT cmsLinkTag(cmsHPROFILE hProfile, cmsTagSignature sig, cmsTagSignature dest);
// CMSAPI cmsTagSignature   CMSEXPORT cmsTagLinkedTo(cmsHPROFILE hProfile, cmsTagSignature sig);

// Read and write raw data
// CMSAPI cmsUInt32Number   CMSEXPORT cmsReadRawTag(cmsHPROFILE hProfile, cmsTagSignature sig, void* Buffer, cmsUInt32Number BufferSize);
// CMSAPI cmsBool           CMSEXPORT cmsWriteRawTag(cmsHPROFILE hProfile, cmsTagSignature sig, const void* data, cmsUInt32Number Size);

// Access header data
pub use types::profile::data_access::{
    EMBEDDED_PROFILE_FALSE, EMBEDDED_PROFILE_TRUE, USE_ANYWHERE, USE_WITH_EMBEDDED_DATA_ONLY,
};

// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetHeaderFlags(cmsHPROFILE hProfile);
// CMSAPI void              CMSEXPORT cmsGetHeaderAttributes(cmsHPROFILE hProfile, cmsUInt64Number* Flags);
// CMSAPI void              CMSEXPORT cmsGetHeaderProfileID(cmsHPROFILE hProfile, cmsUInt8Number* ProfileID);
// CMSAPI cmsBool           CMSEXPORT cmsGetHeaderCreationDateTime(cmsHPROFILE hProfile, struct tm *Dest);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetHeaderRenderingIntent(cmsHPROFILE hProfile);

// CMSAPI void              CMSEXPORT cmsSetHeaderFlags(cmsHPROFILE hProfile, cmsUInt32Number Flags);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetHeaderManufacturer(cmsHPROFILE hProfile);
// CMSAPI void              CMSEXPORT cmsSetHeaderManufacturer(cmsHPROFILE hProfile, cmsUInt32Number manufacturer);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetHeaderCreator(cmsHPROFILE hProfile);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetHeaderModel(cmsHPROFILE hProfile);
// CMSAPI void              CMSEXPORT cmsSetHeaderModel(cmsHPROFILE hProfile, cmsUInt32Number model);
// CMSAPI void              CMSEXPORT cmsSetHeaderAttributes(cmsHPROFILE hProfile, cmsUInt64Number Flags);
// CMSAPI void              CMSEXPORT cmsSetHeaderProfileID(cmsHPROFILE hProfile, cmsUInt8Number* ProfileID);
// CMSAPI void              CMSEXPORT cmsSetHeaderRenderingIntent(cmsHPROFILE hProfile, cmsUInt32Number RenderingIntent);

// CMSAPI cmsColorSpaceSignature
//                          CMSEXPORT cmsGetPCS(cmsHPROFILE hProfile);
// CMSAPI void              CMSEXPORT cmsSetPCS(cmsHPROFILE hProfile, cmsColorSpaceSignature pcs);
// CMSAPI cmsColorSpaceSignature
//                          CMSEXPORT cmsGetColorSpace(cmsHPROFILE hProfile);
// CMSAPI void              CMSEXPORT cmsSetColorSpace(cmsHPROFILE hProfile, cmsColorSpaceSignature sig);
// CMSAPI cmsProfileClassSignature
//                          CMSEXPORT cmsGetDeviceClass(cmsHPROFILE hProfile);
// CMSAPI void              CMSEXPORT cmsSetDeviceClass(cmsHPROFILE hProfile, cmsProfileClassSignature sig);
// CMSAPI void              CMSEXPORT cmsSetProfileVersion(cmsHPROFILE hProfile, cmsFloat64Number Version);
// CMSAPI cmsFloat64Number  CMSEXPORT cmsGetProfileVersion(cmsHPROFILE hProfile);

// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetEncodedICCversion(cmsHPROFILE hProfile);
// CMSAPI void              CMSEXPORT cmsSetEncodedICCversion(cmsHPROFILE hProfile, cmsUInt32Number Version);

// How profiles may be used
pub use types::profile::r#use::AS_INPUT as LCMS_USED_AS_INPUT;
pub use types::profile::r#use::AS_OUTPUT as LCMS_USED_AS_OUTPUT;
pub use types::profile::r#use::AS_PROOF as LCMS_USED_AS_PROOF;

// CMSAPI cmsBool           CMSEXPORT cmsIsIntentSupported(cmsHPROFILE hProfile, cmsUInt32Number Intent, cmsUInt32Number UsedDirection);
// CMSAPI cmsBool           CMSEXPORT cmsIsMatrixShaper(cmsHPROFILE hProfile);
// CMSAPI cmsBool           CMSEXPORT cmsIsCLUT(cmsHPROFILE hProfile, cmsUInt32Number Intent, cmsUInt32Number UsedDirection);

// Translate form/to our notation to ICC
// CMSAPI cmsColorSpaceSignature   CMSEXPORT _cmsICCcolorSpace(int OurNotation);
// CMSAPI int                      CMSEXPORT _cmsLCMScolorSpace(cmsColorSpaceSignature ProfileSpace);

// Deprecated, use cmsChannelsOfColorSpace instead
// CMSAPI cmsUInt32Number   CMSEXPORT cmsChannelsOf(cmsColorSpaceSignature ColorSpace);

// Get number of channels of color space or -1 if color space is not listed/supported
// CMSAPI cmsInt32Number CMSEXPORT cmsChannelsOfColorSpace(cmsColorSpaceSignature ColorSpace);

// Build a suitable formatter for the colorspace of this profile. nBytes=1 means 8 bits, nBytes=2 means 16 bits.
// CMSAPI cmsUInt32Number   CMSEXPORT cmsFormatterForColorspaceOfProfile(cmsHPROFILE hProfile, cmsUInt32Number nBytes, cmsBool lIsFloat);
// CMSAPI cmsUInt32Number   CMSEXPORT cmsFormatterForPCSOfProfile(cmsHPROFILE hProfile, cmsUInt32Number nBytes, cmsBool lIsFloat);

// Localized info
pub use types::profile::info_type::DESCRIPTION as INFO_DESCRIPTION;
pub use types::profile::info_type::MANUFACTURER as INFO_MANUFACTURER;
pub use types::profile::info_type::MODEL as INFO_MODEL;
pub use types::profile::info_type::COPYRIGHT as INFO_COPYRIGHT;

// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetProfileInfo(cmsHPROFILE hProfile, cmsInfoType Info,
//                                                             const char LanguageCode[3], const char CountryCode[3],
//                                                             wchar_t* Buffer, cmsUInt32Number BufferSize);

// CMSAPI cmsUInt32Number   CMSEXPORT cmsGetProfileInfoASCII(cmsHPROFILE hProfile, cmsInfoType Info,
//                                                             const char LanguageCode[3], const char CountryCode[3],
//                                                             char* Buffer, cmsUInt32Number BufferSize);

// IO handlers ----------------------------------------------------------------------------------------------------------

// typedef struct _cms_io_handler cmsIOHANDLER;

// CMSAPI cmsIOHANDLER*     CMSEXPORT cmsOpenIOhandlerFromFile(cmsContext ContextID, const char* FileName, const char* AccessMode);
// CMSAPI cmsIOHANDLER*     CMSEXPORT cmsOpenIOhandlerFromStream(cmsContext ContextID, FILE* Stream);
// CMSAPI cmsIOHANDLER*     CMSEXPORT cmsOpenIOhandlerFromMem(cmsContext ContextID, void *Buffer, cmsUInt32Number size, const char* AccessMode);
// CMSAPI cmsIOHANDLER*     CMSEXPORT cmsOpenIOhandlerFromNULL(cmsContext ContextID);
// CMSAPI cmsIOHANDLER*     CMSEXPORT cmsGetProfileIOhandler(cmsHPROFILE hProfile);
// CMSAPI cmsBool           CMSEXPORT cmsCloseIOhandler(cmsIOHANDLER* io);

// MD5 message digest --------------------------------------------------------------------------------------------------

// CMSAPI cmsBool           CMSEXPORT cmsMD5computeID(cmsHPROFILE hProfile);

// Profile high level functions ------------------------------------------------------------------------------------------

// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromFile(const char *ICCProfile, const char *sAccess);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromFileTHR(cmsContext ContextID, const char *ICCProfile, const char *sAccess);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromStream(FILE* ICCProfile, const char* sAccess);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromStreamTHR(cmsContext ContextID, FILE* ICCProfile, const char* sAccess);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromMem(const void * MemPtr, cmsUInt32Number dwSize);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromMemTHR(cmsContext ContextID, const void * MemPtr, cmsUInt32Number dwSize);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromIOhandlerTHR(cmsContext ContextID, cmsIOHANDLER* io);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsOpenProfileFromIOhandler2THR(cmsContext ContextID, cmsIOHANDLER* io, cmsBool write);
// CMSAPI cmsBool          CMSEXPORT cmsCloseProfile(cmsHPROFILE hProfile);

// CMSAPI cmsBool          CMSEXPORT cmsSaveProfileToFile(cmsHPROFILE hProfile, const char* FileName);
// CMSAPI cmsBool          CMSEXPORT cmsSaveProfileToStream(cmsHPROFILE hProfile, FILE* Stream);
// CMSAPI cmsBool          CMSEXPORT cmsSaveProfileToMem(cmsHPROFILE hProfile, void *MemPtr, cmsUInt32Number* BytesNeeded);
// CMSAPI cmsUInt32Number  CMSEXPORT cmsSaveProfileToIOhandler(cmsHPROFILE hProfile, cmsIOHANDLER* io);

// Predefined virtual profiles ------------------------------------------------------------------------------------------

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateRGBProfileTHR(cmsContext ContextID,
//                                                    const cmsCIExyY* WhitePoint,
//                                                    const cmsCIExyYTRIPLE* Primaries,
//                                                    cmsToneCurve* const TransferFunction[3]);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateRGBProfile(const cmsCIExyY* WhitePoint,
//                                                    const cmsCIExyYTRIPLE* Primaries,
//                                                    cmsToneCurve* const TransferFunction[3]);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateGrayProfileTHR(cmsContext ContextID,
//                                                     const cmsCIExyY* WhitePoint,
//                                                     const cmsToneCurve* TransferFunction);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateGrayProfile(const cmsCIExyY* WhitePoint,
//                                                     const cmsToneCurve* TransferFunction);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateLinearizationDeviceLinkTHR(cmsContext ContextID,
//                                                                 cmsColorSpaceSignature ColorSpace,
//                                                                 cmsToneCurve* const TransferFunctions[]);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateLinearizationDeviceLink(cmsColorSpaceSignature ColorSpace,
//                                                                 cmsToneCurve* const TransferFunctions[]);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateInkLimitingDeviceLinkTHR(cmsContext ContextID,
//                                                               cmsColorSpaceSignature ColorSpace, cmsFloat64Number Limit);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateInkLimitingDeviceLink(cmsColorSpaceSignature ColorSpace, cmsFloat64Number Limit);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateLab2ProfileTHR(cmsContext ContextID, const cmsCIExyY* WhitePoint);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateLab2Profile(const cmsCIExyY* WhitePoint);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateLab4ProfileTHR(cmsContext ContextID, const cmsCIExyY* WhitePoint);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateLab4Profile(const cmsCIExyY* WhitePoint);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateXYZProfileTHR(cmsContext ContextID);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateXYZProfile(void);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreate_sRGBProfileTHR(cmsContext ContextID);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreate_sRGBProfile(void);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateBCHSWabstractProfileTHR(cmsContext ContextID,
//                                                              cmsUInt32Number nLUTPoints,
//                                                              cmsFloat64Number Bright,
//                                                              cmsFloat64Number Contrast,
//                                                              cmsFloat64Number Hue,
//                                                              cmsFloat64Number Saturation,
//                                                              cmsUInt32Number TempSrc,
//                                                              cmsUInt32Number TempDest);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateBCHSWabstractProfile(cmsUInt32Number nLUTPoints,
//                                                              cmsFloat64Number Bright,
//                                                              cmsFloat64Number Contrast,
//                                                              cmsFloat64Number Hue,
//                                                              cmsFloat64Number Saturation,
//                                                              cmsUInt32Number TempSrc,
//                                                              cmsUInt32Number TempDest);

// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateNULLProfileTHR(cmsContext ContextID);
// CMSAPI cmsHPROFILE      CMSEXPORT cmsCreateNULLProfile(void);

// Converts a transform to a devicelink profile
// CMSAPI cmsHPROFILE      CMSEXPORT cmsTransform2DeviceLink(cmsHTRANSFORM hTransform, cmsFloat64Number Version, cmsUInt32Number dwFlags);

// Intents ----------------------------------------------------------------------------------------------

// // ICC Intents
pub use crate::intent::PERCEPTUAL as INTENT_PERCEPTUAL;
pub use crate::intent::RELATIVE_COLORIMETRIC as INTENT_RELATIVE_COLORIMETRIC;
pub use crate::intent::SATURATION as INTENT_SATURATION;
pub use crate::intent::ABSOLUTE_COLORIMETRIC as INTENT_ABSOLUTE_COLORIMETRIC;

// // Non-ICC intents
pub use crate::intent::PRESERVE_K_ONLY_PERCEPTUAL as INTENT_PRESERVE_K_ONLY_PERCEPTUAL;
pub use crate::intent::PRESERVE_K_ONLY_RELATIVE_COLORIMETRIC as INTENT_PRESERVE_K_ONLY_RELATIVE_COLORIMETRIC;
pub use crate::intent::PRESERVE_K_ONLY_SATURATION as INTENT_PRESERVE_K_ONLY_SATURATION;
pub use crate::intent::PRESERVE_K_PLANE_PERCEPTUAL as INTENT_PRESERVE_K_PLANE_PERCEPTUAL;
pub use crate::intent::PRESERVE_K_PLANE_RELATIVE_COLORIMETRIC as INTENT_PRESERVE_K_PLANE_RELATIVE_COLORIMETRIC;
pub use crate::intent::PRESERVE_K_PLANE_SATURATION as INTENT_PRESERVE_K_PLANE_SATURATION;

// Call with NULL as parameters to get the intent count
// CMSAPI cmsUInt32Number  CMSEXPORT cmsGetSupportedIntents(cmsUInt32Number nMax, cmsUInt32Number* Codes, char** Descriptions);
// CMSAPI cmsUInt32Number  CMSEXPORT cmsGetSupportedIntentsTHR(cmsContext ContextID, cmsUInt32Number nMax, cmsUInt32Number* Codes, char** Descriptions);

// Flags

pub use crate::flags::FLAGS_NOCACHE as FLAGS_NOCACHE;
pub use crate::flags::FLAGS_NOOPTIMIZE as FLAGS_NOOPTIMIZE;
pub use crate::flags::FLAGS_NULLTRANSFORM as FLAGS_NULLTRANSFORM;

// Proofing flags
pub use crate::flags::FLAGS_GAMUTCHECK as FLAGS_GAMUTCHECK;
pub use crate::flags::FLAGS_SOFTPROOFING as FLAGS_SOFTPROOFING;

// Misc
pub use crate::flags::FLAGS_BLACKPOINTCOMPENSATION as FLAGS_BLACKPOINTCOMPENSATION;
pub use crate::flags::FLAGS_NOWHITEONWHITEFIXUP as FLAGS_NOWHITEONWHITEFIXUP;
pub use crate::flags::FLAGS_HIGHRESPRECALC as FLAGS_HIGHRESPRECALC;
pub use crate::flags::FLAGS_LOWRESPRECALC as FLAGS_LOWRESPRECALC;

// For devicelink creation
pub use crate::flags::FLAGS_8BITS_DEVICELINK as FLAGS_8BITS_DEVICELINK;
pub use crate::flags::FLAGS_GUESSDEVICECLASS as FLAGS_GUESSDEVICECLASS;
pub use crate::flags::FLAGS_KEEP_SEQUENCE as FLAGS_KEEP_SEQUENCE;

// Specific to a particular optimizations
pub use crate::flags::FLAGS_FORCE_CLUT as FLAGS_FORCE_CLUT;
pub use crate::flags::FLAGS_CLUT_POST_LINEARIZATION as FLAGS_CLUT_POST_LINEARIZATION;
pub use crate::flags::FLAGS_CLUT_PRE_LINEARIZATION as FLAGS_CLUT_PRE_LINEARIZATION;

// Specific to unbounded mode
pub use crate::flags::FLAGS_NONEGATIVES as FLAGS_NONEGATIVES;

// Copy alpha channels when transforming
pub use crate::flags::FLAGS_COPY_ALPHA as FLAGS_COPY_ALPHA;

// Fine-tune control over number of gridpoints
pub use crate::GRIDPOINTS as FLAGS_GRIDPOINTS;

// CRD special
pub use crate::flags::FLAGS_NODEFAULTRESOURCEDEF as FLAGS_NODEFAULTRESOURCEDEF;

// Transforms ---------------------------------------------------------------------------------------------------

// CMSAPI cmsHTRANSFORM    CMSEXPORT cmsCreateTransformTHR(cmsContext ContextID,
//                                                   cmsHPROFILE Input,
//                                                   cmsUInt32Number InputFormat,
//                                                   cmsHPROFILE Output,
//                                                   cmsUInt32Number OutputFormat,
//                                                   cmsUInt32Number Intent,
//                                                   cmsUInt32Number dwFlags);

// CMSAPI cmsHTRANSFORM    CMSEXPORT cmsCreateTransform(cmsHPROFILE Input,
//                                                   cmsUInt32Number InputFormat,
//                                                   cmsHPROFILE Output,
//                                                   cmsUInt32Number OutputFormat,
//                                                   cmsUInt32Number Intent,
//                                                   cmsUInt32Number dwFlags);

// CMSAPI cmsHTRANSFORM    CMSEXPORT cmsCreateProofingTransformTHR(cmsContext ContextID,
//                                                   cmsHPROFILE Input,
//                                                   cmsUInt32Number InputFormat,
//                                                   cmsHPROFILE Output,
//                                                   cmsUInt32Number OutputFormat,
//                                                   cmsHPROFILE Proofing,
//                                                   cmsUInt32Number Intent,
//                                                   cmsUInt32Number ProofingIntent,
//                                                   cmsUInt32Number dwFlags);

// CMSAPI cmsHTRANSFORM    CMSEXPORT cmsCreateProofingTransform(cmsHPROFILE Input,
//                                                   cmsUInt32Number InputFormat,
//                                                   cmsHPROFILE Output,
//                                                   cmsUInt32Number OutputFormat,
//                                                   cmsHPROFILE Proofing,
//                                                   cmsUInt32Number Intent,
//                                                   cmsUInt32Number ProofingIntent,
//                                                   cmsUInt32Number dwFlags);

// CMSAPI cmsHTRANSFORM    CMSEXPORT cmsCreateMultiprofileTransformTHR(cmsContext ContextID,
//                                                   cmsHPROFILE hProfiles[],
//                                                   cmsUInt32Number nProfiles,
//                                                   cmsUInt32Number InputFormat,
//                                                   cmsUInt32Number OutputFormat,
//                                                   cmsUInt32Number Intent,
//                                                   cmsUInt32Number dwFlags);

// CMSAPI cmsHTRANSFORM    CMSEXPORT cmsCreateMultiprofileTransform(cmsHPROFILE hProfiles[],
//                                                   cmsUInt32Number nProfiles,
//                                                   cmsUInt32Number InputFormat,
//                                                   cmsUInt32Number OutputFormat,
//                                                   cmsUInt32Number Intent,
//                                                   cmsUInt32Number dwFlags);

// CMSAPI cmsHTRANSFORM    CMSEXPORT cmsCreateExtendedTransform(cmsContext ContextID,
//                                                    cmsUInt32Number nProfiles, cmsHPROFILE hProfiles[],
//                                                    cmsBool  BPC[],
//                                                    cmsUInt32Number Intents[],
//                                                    cmsFloat64Number AdaptationStates[],
//                                                    cmsHPROFILE hGamutProfile,
//                                                    cmsUInt32Number nGamutPCSposition,
//                                                    cmsUInt32Number InputFormat,
//                                                    cmsUInt32Number OutputFormat,
//                                                    cmsUInt32Number dwFlags);

// CMSAPI void             CMSEXPORT cmsDeleteTransform(cmsHTRANSFORM hTransform);

// CMSAPI void             CMSEXPORT cmsDoTransform(cmsHTRANSFORM Transform,
//                                                  const void * InputBuffer,
//                                                  void * OutputBuffer,
//                                                  cmsUInt32Number Size);

// CMSAPI void             CMSEXPORT cmsDoTransformStride(cmsHTRANSFORM Transform,   // Deprecated
//                                                  const void * InputBuffer,
//                                                  void * OutputBuffer,
//                                                  cmsUInt32Number Size,
//                                                  cmsUInt32Number Stride);

// CMSAPI void             CMSEXPORT cmsDoTransformLineStride(cmsHTRANSFORM  Transform,
//                                                  const void* InputBuffer,
//                                                  void* OutputBuffer,
//                                                  cmsUInt32Number PixelsPerLine,
//                                                  cmsUInt32Number LineCount,
//                                                  cmsUInt32Number BytesPerLineIn,
//                                                  cmsUInt32Number BytesPerLineOut,
//                                                  cmsUInt32Number BytesPerPlaneIn,
//                                                  cmsUInt32Number BytesPerPlaneOut);

// CMSAPI void             CMSEXPORT cmsSetAlarmCodes(const cmsUInt16Number NewAlarm[cmsMAXCHANNELS]);
// CMSAPI void             CMSEXPORT cmsGetAlarmCodes(cmsUInt16Number NewAlarm[cmsMAXCHANNELS]);

// CMSAPI void             CMSEXPORT cmsSetAlarmCodesTHR(cmsContext ContextID,
//                                                           const cmsUInt16Number AlarmCodes[cmsMAXCHANNELS]);
// CMSAPI void             CMSEXPORT cmsGetAlarmCodesTHR(cmsContext ContextID,
//                                                           cmsUInt16Number AlarmCodes[cmsMAXCHANNELS]);

// Adaptation state for absolute colorimetric intent
// CMSAPI cmsFloat64Number CMSEXPORT cmsSetAdaptationState(cmsFloat64Number d);
// CMSAPI cmsFloat64Number CMSEXPORT cmsSetAdaptationStateTHR(cmsContext ContextID, cmsFloat64Number d);

// Grab the ContextID from an open transform. Returns NULL if a NULL transform is passed
// CMSAPI cmsContext       CMSEXPORT cmsGetTransformContextID(cmsHTRANSFORM hTransform);

// Grab the input/output formats
// CMSAPI cmsUInt32Number CMSEXPORT cmsGetTransformInputFormat(cmsHTRANSFORM hTransform);
// CMSAPI cmsUInt32Number CMSEXPORT cmsGetTransformOutputFormat(cmsHTRANSFORM hTransform);

// For backwards compatibility
// CMSAPI cmsBool          CMSEXPORT cmsChangeBuffersFormat(cmsHTRANSFORM hTransform,
//                                                          cmsUInt32Number InputFormat,
//                                                          cmsUInt32Number OutputFormat);

// PostScript ColorRenderingDictionary and ColorSpaceArray ----------------------------------------------------

pub use crate::PostScriptResourceType as PSResourceType;
pub const PS_RESOURCE_CSA: PSResourceType = PSResourceType::ColorSpaceArray;
pub const PS_RESOURCE_CRD: PSResourceType = PSResourceType::ColorRenderingDictionary;

// lcms2 unified method to access postscript color resources
// CMSAPI cmsUInt32Number  CMSEXPORT cmsGetPostScriptColorResource(cmsContext ContextID,
//                                                                 cmsPSResourceType Type,
//                                                                 cmsHPROFILE hProfile,
//                                                                 cmsUInt32Number Intent,
//                                                                 cmsUInt32Number dwFlags,
//                                                                 cmsIOHANDLER* io);

// CMSAPI cmsUInt32Number  CMSEXPORT cmsGetPostScriptCSA(cmsContext ContextID, cmsHPROFILE hProfile, cmsUInt32Number Intent, cmsUInt32Number dwFlags, void* Buffer, cmsUInt32Number dwBufferLen);
// CMSAPI cmsUInt32Number  CMSEXPORT cmsGetPostScriptCRD(cmsContext ContextID, cmsHPROFILE hProfile, cmsUInt32Number Intent, cmsUInt32Number dwFlags, void* Buffer, cmsUInt32Number dwBufferLen);

// IT8.7 / CGATS.17-200x handling -----------------------------------------------------------------------------

// CMSAPI cmsHANDLE        CMSEXPORT cmsIT8Alloc(cmsContext ContextID);
// CMSAPI void             CMSEXPORT cmsIT8Free(cmsHANDLE hIT8);

// Tables
// CMSAPI cmsUInt32Number  CMSEXPORT cmsIT8TableCount(cmsHANDLE hIT8);
// CMSAPI cmsInt32Number   CMSEXPORT cmsIT8SetTable(cmsHANDLE hIT8, cmsUInt32Number nTable);

// Persistence
// CMSAPI cmsHANDLE        CMSEXPORT cmsIT8LoadFromFile(cmsContext ContextID, const char* cFileName);
// CMSAPI cmsHANDLE        CMSEXPORT cmsIT8LoadFromMem(cmsContext ContextID, const void *Ptr, cmsUInt32Number len);
// // CMSAPI cmsHANDLE        CMSEXPORT cmsIT8LoadFromIOhandler(cmsContext ContextID, cmsIOHANDLER* io);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SaveToFile(cmsHANDLE hIT8, const char* cFileName);
// CMSAPI cmsBool          CMSEXPORT cmsIT8SaveToMem(cmsHANDLE hIT8, void *MemPtr, cmsUInt32Number* BytesNeeded);

// // Properties
// CMSAPI const char*      CMSEXPORT cmsIT8GetSheetType(cmsHANDLE hIT8);
// CMSAPI cmsBool          CMSEXPORT cmsIT8SetSheetType(cmsHANDLE hIT8, const char* Type);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SetComment(cmsHANDLE hIT8, const char* cComment);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SetPropertyStr(cmsHANDLE hIT8, const char* cProp, const char *Str);
// CMSAPI cmsBool          CMSEXPORT cmsIT8SetPropertyDbl(cmsHANDLE hIT8, const char* cProp, cmsFloat64Number Val);
// CMSAPI cmsBool          CMSEXPORT cmsIT8SetPropertyHex(cmsHANDLE hIT8, const char* cProp, cmsUInt32Number Val);
// CMSAPI cmsBool          CMSEXPORT cmsIT8SetPropertyMulti(cmsHANDLE hIT8, const char* Key, const char* SubKey, const char *Buffer);
// CMSAPI cmsBool          CMSEXPORT cmsIT8SetPropertyUncooked(cmsHANDLE hIT8, const char* Key, const char* Buffer);

// CMSAPI const char*      CMSEXPORT cmsIT8GetProperty(cmsHANDLE hIT8, const char* cProp);
// CMSAPI cmsFloat64Number CMSEXPORT cmsIT8GetPropertyDbl(cmsHANDLE hIT8, const char* cProp);
// CMSAPI const char*      CMSEXPORT cmsIT8GetPropertyMulti(cmsHANDLE hIT8, const char* Key, const char *SubKey);
// CMSAPI cmsUInt32Number  CMSEXPORT cmsIT8EnumProperties(cmsHANDLE hIT8, char ***PropertyNames);
// CMSAPI cmsUInt32Number  CMSEXPORT cmsIT8EnumPropertyMulti(cmsHANDLE hIT8, const char* cProp, const char ***SubpropertyNames);

// Datasets
// CMSAPI const char*      CMSEXPORT cmsIT8GetDataRowCol(cmsHANDLE hIT8, int row, int col);
// CMSAPI cmsFloat64Number CMSEXPORT cmsIT8GetDataRowColDbl(cmsHANDLE hIT8, int row, int col);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SetDataRowCol(cmsHANDLE hIT8, int row, int col,
//                                                 const char* Val);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SetDataRowColDbl(cmsHANDLE hIT8, int row, int col,
//                                                 cmsFloat64Number Val);

// CMSAPI const char*      CMSEXPORT cmsIT8GetData(cmsHANDLE hIT8, const char* cPatch, const char* cSample);

// CMSAPI cmsFloat64Number CMSEXPORT cmsIT8GetDataDbl(cmsHANDLE hIT8, const char* cPatch, const char* cSample);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SetData(cmsHANDLE hIT8, const char* cPatch,
//                                                 const char* cSample,
//                                                 const char *Val);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SetDataDbl(cmsHANDLE hIT8, const char* cPatch,
//                                                 const char* cSample,
//                                                 cmsFloat64Number Val);

// CMSAPI int              CMSEXPORT cmsIT8FindDataFormat(cmsHANDLE hIT8, const char* cSample);
// CMSAPI cmsBool          CMSEXPORT cmsIT8SetDataFormat(cmsHANDLE hIT8, int n, const char *Sample);
// CMSAPI int              CMSEXPORT cmsIT8EnumDataFormat(cmsHANDLE hIT8, char ***SampleNames);

// CMSAPI const char*      CMSEXPORT cmsIT8GetPatchName(cmsHANDLE hIT8, int nPatch, char* buffer);
// CMSAPI int              CMSEXPORT cmsIT8GetPatchByName(cmsHANDLE hIT8, const char *cPatch);

// The LABEL extension
// CMSAPI int              CMSEXPORT cmsIT8SetTableByLabel(cmsHANDLE hIT8, const char* cSet, const char* cField, const char* ExpectedType);

// CMSAPI cmsBool          CMSEXPORT cmsIT8SetIndexColumn(cmsHANDLE hIT8, const char* cSample);

// Formatter for double
// CMSAPI void             CMSEXPORT cmsIT8DefineDblFormat(cmsHANDLE hIT8, const char* Formatter);

// Gamut boundary description routines ------------------------------------------------------------------------------

// CMSAPI cmsHANDLE        CMSEXPORT cmsGBDAlloc(cmsContext ContextID);
// CMSAPI void             CMSEXPORT cmsGBDFree(cmsHANDLE hGBD);
// CMSAPI cmsBool          CMSEXPORT cmsGDBAddPoint(cmsHANDLE hGBD, const cmsCIELab* Lab);
// CMSAPI cmsBool          CMSEXPORT cmsGDBCompute(cmsHANDLE  hGDB, cmsUInt32Number dwFlags);
// CMSAPI cmsBool          CMSEXPORT cmsGDBCheckPoint(cmsHANDLE hGBD, const cmsCIELab* Lab);

// Feature detection  ----------------------------------------------------------------------------------------------

// Estimate the black point
// CMSAPI cmsBool          CMSEXPORT cmsDetectBlackPoint(cmsCIEXYZ* BlackPoint, cmsHPROFILE hProfile, cmsUInt32Number Intent, cmsUInt32Number dwFlags);
// CMSAPI cmsBool          CMSEXPORT cmsDetectDestinationBlackPoint(cmsCIEXYZ* BlackPoint, cmsHPROFILE hProfile, cmsUInt32Number Intent, cmsUInt32Number dwFlags);

// Estimate total area coverage
// CMSAPI cmsFloat64Number CMSEXPORT cmsDetectTAC(cmsHPROFILE hProfile);

// Estimate gamma space, always positive. Returns -1 on error.
// CMSAPI cmsFloat64Number CMSEXPORT cmsDetectRGBProfileGamma(cmsHPROFILE hProfile, cmsFloat64Number threshold);

// Poor man's gamut mapping
// CMSAPI cmsBool          CMSEXPORT cmsDesaturateLab(cmsCIELab* Lab,
//                                                    double amax, double amin,
//                                                    double bmax, double bmin);
