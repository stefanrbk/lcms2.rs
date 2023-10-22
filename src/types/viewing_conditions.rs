use super::XYZ;

pub struct ViewingConditions {
    pub illuminant: XYZ,
    pub surround: XYZ,
    pub illuminant_type: u32,
}

pub mod cam02;
