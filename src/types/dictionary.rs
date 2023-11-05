use crate::list;

use super::MLU;

pub struct Entry {
    pub next: list::List<Entry>,
    pub display_name: Box<MLU>,
    pub display_value: Box<MLU>,
    pub name: String,
    pub value: String,
}
