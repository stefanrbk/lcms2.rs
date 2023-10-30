use crate::types::ToneCurve;

pub struct ToneCurvesData<'a> {
    pub n_curves: u32,
    pub the_curves: Box<[&'a ToneCurve<'a>]>,
}
