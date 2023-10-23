use crate::types::Transform2Fn;

use super::Base;

pub struct Parallelization {
    pub base: Base,
    pub max_workers: i32,
    pub worker_flags: u32,
    pub scheduler_fn: Transform2Fn,
}
