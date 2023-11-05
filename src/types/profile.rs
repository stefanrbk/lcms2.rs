use std::{any::Any, sync::Arc};

use chrono::{DateTime as dt, Utc};

use super::{DateTime, EncodedXYZ, ProfileID, Signature};
use crate::{
    io::IoHandler,
    plugin::{IMutex, TagTypeHandler},
    Context,
};

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
    pub context_id: Context,
    pub io_handler: Option<Arc<dyn IoHandler>>,
    pub created: dt<Utc>,

    pub version: u32,
    pub device_class: Signature,
    pub color_space: Signature,
    pub pcs: Signature,
    pub rendering_intent: u32,

    pub flags: u32,
    pub manufacturer: u32,
    pub model: u32,
    pub attributes: u64,
    pub creator: u32,

    pub profile_id: ProfileID,

    pub tags: Vec<TagEntry>,

    pub is_write: bool,

    pub user_mutex: Option<Box<dyn IMutex>>,
}

pub struct TagEntry {
    pub name: Signature,
    pub linked: Option<Signature>,
    pub size: usize,
    pub offset: usize,
    pub save_as_raw: bool,
    pub tag_object: Option<Box<dyn Any>>,
    pub type_handler: Arc<TagTypeHandler>,
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
