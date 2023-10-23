pub struct MatrixData<'a> {
    pub double: &'a [f64],
    pub offset: Option<&'a [f64]>,
}
