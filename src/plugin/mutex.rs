use super::Base;

pub trait MutexGuard {}

pub trait IMutex {
    fn lock(&self) -> dyn MutexGuard;
}

pub type CreateMutexFn = fn(context_id: crate::Context) -> dyn IMutex;
pub type DestroyMutexFn = fn(context_id: crate::Context, mtx: dyn IMutex);
pub type LockMutexFn = fn(context_id: crate::Context, mtx: &dyn IMutex) -> dyn MutexGuard;
pub type UnlockMutexFn = fn(context_id: crate::Context, mtx: &dyn IMutex, guard: dyn MutexGuard);

pub struct Mutex {
    pub base: Base,
    pub create: CreateMutexFn,
    pub destroy: DestroyMutexFn,
    pub lock: LockMutexFn,
    pub unlock: UnlockMutexFn,
}
