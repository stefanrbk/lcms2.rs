use super::XYZ;

pub struct MeasurementConditions {
    pub observer: u32,
    pub backing: XYZ,
    pub geometry: u32,
    pub flare: f64,
    pub illuminant_type: u32,
}
