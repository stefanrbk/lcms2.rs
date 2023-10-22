use super::XYZ;

pub struct ViewingConditions {
    pub white_point: XYZ,
    pub yb: f64,
    pub la: f64,
    pub surround: u32,
    pub d_value: f64,
}

pub const D_CALCULATE: u32 = u32::MAX;

pub mod surround {
    pub const AVG: u32 = 1;
    pub const DIM: u32 = 2;
    pub const DARK: u32 = 3;
    pub const CUTSHEET: u32 = 4;
}
