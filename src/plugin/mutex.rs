use super::Base;

pub trait MutexGuard {}

pub trait IMutex<'a> {
    fn lock(&'a self) -> Box<dyn MutexGuard + 'a>;
}

pub type CreateMutexFn = fn(context_id: crate::Context) -> dyn IMutex<'static>;
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

impl<'a> IMutex<'a> for std::sync::Mutex<()> {
    fn lock(&'a self) -> Box<dyn MutexGuard + 'a> {
        let result = self.lock();
        Box::new(result.unwrap())
    }
}

impl<'a> MutexGuard for std::sync::MutexGuard<'a, ()> {}
