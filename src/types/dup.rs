use crate::Context;

pub trait Dup {
    fn dup(&self, context_id: &'static Context) -> Result<Self, String>
    where
        Self: Sized;
}
