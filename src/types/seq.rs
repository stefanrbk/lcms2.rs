use crate::Context;

use super::{ProfileID, Signature, MLU};

pub struct PSeqDesc {
    pub device_mfg: Signature,
    pub device_model: Signature,
    pub attributes: u64,
    pub technology: Signature,
    pub profile_id: ProfileID,
    pub manufacturer: Box<MLU>,
    pub model: Box<MLU>,
    pub description: Box<MLU>,
}

pub struct Seq {
    pub n: u32,
    pub context_id: Context,
    pub seq: PSeqDesc,
}
