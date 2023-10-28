use crate::Context;

use super::{ProfileID, Signature, MLU};

pub struct PSeqDesc<'a> {
    pub device_mfg: Signature,
    pub device_model: Signature,
    pub attributes: u64,
    pub technology: Signature,
    pub profile_id: ProfileID,
    pub manufacturer: Box<MLU<'a>>,
    pub model: Box<MLU<'a>>,
    pub description: Box<MLU<'a>>,
}

pub struct Seq<'a> {
    pub n: u32,
    pub context_id: &'a Context,
    pub seq: PSeqDesc<'a>,
}
