use crate::{
    types::{Pipeline, Profile},
    Context,
};

use super::Base;

pub type IntentFn = fn(
    context_id: Context,
    n_profiles: u32,
    intents: Box<[u32]>,
    profiles: Box<[Profile]>,
    bpc: Box<[bool]>,
    adaptation_states: Box<[f64]>,
    flags: u32,
) -> Result<Pipeline, String>;

pub struct RenderingIntent {
    pub base: Base,
    pub intent: u32,
    pub link: IntentFn,
    pub description: String,
}
