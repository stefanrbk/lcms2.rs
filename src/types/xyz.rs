use crate::S15Fixed16Number;

pub struct XYZ {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub struct EncodedXYZ {
    pub x: S15Fixed16Number,
    pub y: S15Fixed16Number,
    pub z: S15Fixed16Number,
}

pub struct XYZTriple {
    pub red: XYZ,
    pub green: XYZ,
    pub blue: XYZ,
}
