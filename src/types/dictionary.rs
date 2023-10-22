use crate::list;

use super::MLU;

pub struct Entry<'a> {
    pub next: list::Link<Entry<'a>>,
    pub display_name: Box<MLU>,
    pub display_value: Box<MLU>,
    pub name: &'a str,
    pub value: &'a str,
}
