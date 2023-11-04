use std::sync::Arc;

use crate::{Context, Result};

pub trait Dup {
    fn dup(&self, context_id: &Context) -> Result<Self>
    where
        Self: Sized;
}
