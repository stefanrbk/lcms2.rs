use crate::types::Signature;

pub const A_TO_B0: Signature = Signature(0x41324230); // 'A2B0'
pub const A_TO_B1: Signature = Signature(0x41324231); // 'A2B1'
pub const A_TO_B2: Signature = Signature(0x41324232); // 'A2B2'
pub const BLUE_COLORANT: Signature = Signature(0x6258595A); // 'bXYZ'
pub const BLUE_MATRIX_COLUMN: Signature = Signature(0x6258595A); // 'bXYZ'
pub const BLUE_TRC: Signature = Signature(0x62545243); // 'bTRC'
pub const B_TO_A0: Signature = Signature(0x42324130); // 'B2A0'
pub const B_TO_A1: Signature = Signature(0x42324131); // 'B2A1'
pub const B_TO_A2: Signature = Signature(0x42324132); // 'B2A2'
pub const CALIBRATION_DATE_TIME: Signature = Signature(0x63616C74); // 'calt'
pub const CHAR_TARGET: Signature = Signature(0x74617267); // 'targ'
pub const CHROMATIC_ADAPTATION: Signature = Signature(0x63686164); // 'chad'
pub const CHROMATICITY: Signature = Signature(0x6368726D); // 'chrm'
pub const COLORANT_ORDER: Signature = Signature(0x636C726F); // 'clro'
pub const COLORANT_TABLE: Signature = Signature(0x636C7274); // 'clrt'
pub const COLORANT_TABLE_OUT: Signature = Signature(0x636C6F74); // 'clot'
pub const COLORIMETRIC_INTENT_IMAGE_STATE: Signature = Signature(0x63696973); // 'ciis'
pub const COPYRIGHT: Signature = Signature(0x63707274); // 'cprt'
pub const CRD_INFO: Signature = Signature(0x63726469); // 'crdi'
pub const DATA: Signature = Signature(0x64617461); // 'data'
pub const DATE_TIME: Signature = Signature(0x6474696D); // 'dtim'
pub const DEVICE_MFG_DESC: Signature = Signature(0x646D6E64); // 'dmnd'
pub const DEVICE_MODEL_DESC: Signature = Signature(0x646D6464); // 'dmdd'
pub const DEVICE_SETTINGS: Signature = Signature(0x64657673); // 'devs'
pub const D_TO_B0: Signature = Signature(0x44324230); // 'D2B0'
pub const D_TO_B1: Signature = Signature(0x44324231); // 'D2B1'
pub const D_TO_B2: Signature = Signature(0x44324232); // 'D2B2'
pub const D_TO_B3: Signature = Signature(0x44324233); // 'D2B3'
pub const B_TO_D0: Signature = Signature(0x42324430); // 'B2D0'
pub const B_TO_D1: Signature = Signature(0x42324431); // 'B2D1'
pub const B_TO_D2: Signature = Signature(0x42324432); // 'B2D2'
pub const B_TO_D3: Signature = Signature(0x42324433); // 'B2D3'
pub const GAMUT: Signature = Signature(0x67616D74); // 'gamt'
pub const GRAY_TRC: Signature = Signature(0x6b545243); // 'kTRC'
pub const GREEN_COLORANT: Signature = Signature(0x6758595A); // 'gXYZ'
pub const GREEN_MATRIX_COLUMN: Signature = Signature(0x6758595A); // 'gXYZ'
pub const GREEN_TRC: Signature = Signature(0x67545243); // 'gTRC'
pub const LUMINANCE: Signature = Signature(0x6C756d69); // 'lumi'
pub const MEASUREMENT: Signature = Signature(0x6D656173); // 'meas'
pub const MEDIA_BLACK_POINT: Signature = Signature(0x626B7074); // 'bkpt'
pub const MEDIA_WHITE_POINT: Signature = Signature(0x77747074); // 'wtpt'
#[deprecated = "use NAMED_COLOR2"]
pub const NAMED_COLOR: Signature = Signature(0x6E636f6C); // 'ncol' // Deprecated by the ICC
pub const NAMED_COLOR2: Signature = Signature(0x6E636C32); // 'ncl2'
pub const OUTPUT_RESPONSE: Signature = Signature(0x72657370); // 'resp'
pub const PERCEPTUAL_RENDERING_INTENT_GAMUT: Signature = Signature(0x72696730); // 'rig0'
pub const PREVIEW0: Signature = Signature(0x70726530); // 'pre0'
pub const PREVIEW1: Signature = Signature(0x70726531); // 'pre1'
pub const PREVIEW2: Signature = Signature(0x70726532); // 'pre2'
pub const PROFILE_DESCRIPTION: Signature = Signature(0x64657363); // 'desc'
pub const PROFILE_DESCRIPTION_ML: Signature = Signature(0x6473636d); // 'dscm'
pub const PROFILE_SEQUENCE_DESC: Signature = Signature(0x70736571); // 'pseq'
pub const PROFILE_SEQUENCE_ID: Signature = Signature(0x70736964); // 'psid'
pub const PS2_CRD0: Signature = Signature(0x70736430); // 'psd0'
pub const PS2_CRD1: Signature = Signature(0x70736431); // 'psd1'
pub const PS2_CRD2: Signature = Signature(0x70736432); // 'psd2'
pub const PS2_CRD3: Signature = Signature(0x70736433); // 'psd3'
pub const PS2_CSA: Signature = Signature(0x70733273); // 'ps2s'
pub const PS2_RENDERING_INTENT: Signature = Signature(0x70733269); // 'ps2i'
pub const RED_COLORANT: Signature = Signature(0x7258595A); // 'rXYZ'
pub const RED_MATRIX_COLUMN: Signature = Signature(0x7258595A); // 'rXYZ'
pub const RED_TRC: Signature = Signature(0x72545243); // 'rTRC'
pub const SATURATION_RENDERING_INTENT_GAMUT: Signature = Signature(0x72696732); // 'rig2'
pub const SCREENING_DESC: Signature = Signature(0x73637264); // 'scrd'
pub const SCREENING: Signature = Signature(0x7363726E); // 'scrn'
pub const TECHNOLOGY: Signature = Signature(0x74656368); // 'tech'
pub const UCR_BG: Signature = Signature(0x62666420); // 'bfd '
pub const VIEWING_COND_DESC: Signature = Signature(0x76756564); // 'vued'
pub const VIEWING_CONDITIONS: Signature = Signature(0x76696577); // 'view'
pub const VCGT: Signature = Signature(0x76636774); // 'vcgt'
pub const META: Signature = Signature(0x6D657461); // 'meta'
pub const CICP: Signature = Signature(0x63696370); // 'cicp'
pub const ARGYLL_ARTS: Signature = Signature(0x61727473); // 'arts'
