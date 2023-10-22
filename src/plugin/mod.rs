use crate::types::InterpParams;

pub type InterpFn<T> = fn(Input: &[T], Output: &mut [T], p: InterpParams<T>);

pub enum InterpFunction {
    F32(InterpFn<f32>),
    U16(InterpFn<u16>)
}

impl InterpFunction {
    pub const fn is_f32(&self) -> bool {
        matches!(*self, Self::F32(_))
    }
    pub const fn is_u16(&self) -> bool {
        matches!(*self, Self::U16(_))
    }
    pub fn is_f32_and(self, f: impl FnOnce(InterpFn<f32>) -> bool) -> bool {
        match self {
            Self::U16(_) => false,
            Self::F32(x) => f(x)
        }
    }
    pub fn is_u16_and(self, f: impl FnOnce(InterpFn<u16>) -> bool) -> bool {
        match self {
            Self::U16(x) => f(x),
            Self::F32(_) => false
        }
    }
}

impl From<InterpFn<u16>> for InterpFunction {
    fn from(val: InterpFn<u16>) -> InterpFunction {
        InterpFunction::U16(val)
    }
}

impl From<InterpFn<f32>> for InterpFunction {
    fn from(val: InterpFn<f32>) -> InterpFunction {
        InterpFunction::F32(val)
    }
}
