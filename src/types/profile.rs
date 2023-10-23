use super::{DateTime, EncodedXYZ, ProfileID, Signature};

pub struct Header {
    pub size: u32,
    pub cmm_id: Signature,
    pub version: u32,
    pub device_class: Signature,
    pub color_space: Signature,
    pub pcs: Signature,
    pub date: DateTime,
    pub magic: Signature,
    pub platform: Signature,
    pub flags: u32,
    pub manufacturer: Signature,
    pub model: u32,
    pub attributes: u64,
    pub rendering_intent: u32,
    pub illuminant: EncodedXYZ,
    pub creator: Signature,
    pub profile_id: ProfileID,
    pub reserved: [i8; 28],
}

pub struct Profile {
    pub context_id: crate::Context,
}
pub mod data_access {
    pub const EMBEDDED_PROFILE_FALSE: u32 = 0;
    pub const EMBEDDED_PROFILE_TRUE: u32 = 1;
    pub const USE_ANYWHERE: u32 = 0;
    pub const USE_WITH_EMBEDDED_DATA_ONLY: u32 = 2;
}

pub mod r#use {
    pub const AS_INPUT: u32 = 0;
    pub const AS_OUTPUT: u32 = 1;
    pub const AS_PROOF: u32 = 2;
}

pub mod info_type {
    pub const DESCRIPTION: u32 = 0;
    pub const MANUFACTURER: u32 = 1;
    pub const MODEL: u32 = 2;
    pub const COPYRIGHT: u32 = 3;
}
