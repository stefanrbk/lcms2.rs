use crate::list;

use super::MLU;

pub struct Entry<'a> {
    pub next: list::Link<Entry<'a>>,
    pub display_name: Box<MLU<'a>>,
    pub display_value: Box<MLU<'a>>,
    pub name: &'a str,
    pub value: &'a str,
}
