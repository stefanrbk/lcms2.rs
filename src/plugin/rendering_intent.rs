use crate::types::{Pipeline, Profile};

use super::Base;

pub type IntentFn = fn(
    context_id: crate::Context,
    n_profiles: u32,
    intents: &[u32],
    profiles: &[Profile],
    bpc: &[bool],
    adaptation_states: &[f64],
    flags: u32,
) -> Result<Pipeline, &'static str>;

pub struct RenderingIntent<'a> {
    pub base: Base,
    pub intent: u32,
    pub link: IntentFn,
    pub description: &'a str
}
