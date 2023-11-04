use std::sync::Arc;

use crate::Context;

pub trait Dup {
    fn dup(&self, context_id: &Arc<Context>) -> Result<Self, String>
    where
        Self: Sized;
}
