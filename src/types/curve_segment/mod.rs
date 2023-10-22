pub struct CurveSegment<'a> {
    pub x0: f64,
    pub x1: f64,
    pub r#type: i32,
    pub params: [f64; 10],
    pub n_grid_points: u32,
    pub sampled_points: &'a [f32],
}
