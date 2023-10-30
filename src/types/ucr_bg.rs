use super::{ToneCurve, MLU};

pub struct UcrBg<'a> {
    pub ucr: ToneCurve<'a>,
    pub bg: ToneCurve<'a>,
    pub desc: Box<MLU>,
}
