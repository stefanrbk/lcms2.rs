use crate::S15F16;

pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct EncodedXYZ {
    pub x: S15F16,
    pub y: S15F16,
    pub z: S15F16,
}

pub struct XYZTriple {
    pub red: XYZ,
    pub green: XYZ,
    pub blue: XYZ,
}
