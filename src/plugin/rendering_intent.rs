use crate::types::{Pipeline, Profile};

use super::Base;

pub type IntentFn = for<'ctx, 'io, 'mtx, 'a> fn(
    context_id: &'ctx crate::Context,
    n_profiles: u32,
    intents: &'a [u32],
    profiles: &'a [Profile<'ctx, 'io, 'ctx, 'a, 'a, 'a>],
    bpc: &'a [bool],
    adaptation_states: &'a [f64],
    flags: u32,
) -> Result<Pipeline<'ctx, 'a, 'a>, String>;

pub struct RenderingIntent<'a> {
    pub base: Base,
    pub intent: u32,
    pub link: IntentFn,
    pub description: &'a str,
}
