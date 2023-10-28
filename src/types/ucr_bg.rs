use super::{ToneCurve, MLU};

pub struct UcrBg<'a> {
    pub ucr: ToneCurve,
    pub bg: ToneCurve,
    pub desc: Box<MLU<'a>>,
}
