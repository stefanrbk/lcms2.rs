use super::{ToneCurve, MLU};

pub struct UcrBg<'a, 'b, 'c, 'd, 'e, 'f, 'g, 'h, 'i, 'j, 'k> {
    pub ucr: ToneCurve<'b, 'c, 'd, 'e, 'f>,
    pub bg: ToneCurve<'g, 'h, 'i, 'j, 'k>,
    pub desc: Box<MLU<'a>>,
}
