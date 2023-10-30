use crate::s15f16;

#[derive(Copy, Clone)]
pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Copy, Clone)]
pub struct EncodedXYZ {
    pub x: s15f16,
    pub y: s15f16,
    pub z: s15f16,
}

#[derive(Copy, Clone)]
pub struct XYZTriple {
    pub red: XYZ,
    pub green: XYZ,
    pub blue: XYZ,
}
