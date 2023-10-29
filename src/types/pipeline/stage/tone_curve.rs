use crate::types::ToneCurve;

pub struct ToneCurvesData<'a, 'b, 'c, 'd, 'e, 'f, 'g> {
    pub n_curves: u32,
    pub the_curves: &'a [&'b ToneCurve<'c, 'd, 'e, 'f, 'g>],
}
