use crate::types::{Pipeline, Profile};

use super::Base;

pub type IntentFn = for<'a> fn(
    context_id: &'static crate::Context,
    n_profiles: u32,
    intents: Box<[u32]>,
    profiles: Box<[Profile<'a, 'a, 'a>]>,
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
