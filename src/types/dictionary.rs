use crate::list;

use super::MLU;

pub struct Entry {
    pub next: list::Link<Entry>,
    pub display_name: Box<MLU>,
    pub display_value: Box<MLU>,
    pub name: String,
    pub value: String,
}
