#[derive(Copy, Clone)]
pub struct XYY {
    pub x: f64,
    pub y: f64,
    pub y_lum: f64,
}

#[derive(Copy, Clone)]
pub struct XYYTriple {
    pub red: XYY,
    pub green: XYY,
    pub blue: XYY,
}
