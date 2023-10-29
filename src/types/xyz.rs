use crate::s15f16;

pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct EncodedXYZ {
    pub x: s15f16,
    pub y: s15f16,
    pub z: s15f16,
}

pub struct XYZTriple {
    pub red: XYZ,
    pub green: XYZ,
    pub blue: XYZ,
}
