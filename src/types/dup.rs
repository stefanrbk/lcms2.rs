use crate::Context;

pub trait Dup<'a, 'b: 'a> {
    fn dup(&self, context_id: &'b Context) -> Result<Self, String>
    where
        Self: Sized;
}
