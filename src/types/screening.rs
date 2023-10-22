use crate::MAX_CHANNELS;

pub struct ScreeningChannel {
    pub frequency: f64,
    pub screen_angle: f64,
    pub spot_shape: u32,
}

pub struct Screening {
    pub flag: u32,
    pub n_channels: u32,
    pub channels: [ScreeningChannel; MAX_CHANNELS as usize],
}
