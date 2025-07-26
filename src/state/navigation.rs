#[derive(Default, Clone, Debug)]
pub struct NavRouteStep {

    pub star_system: String,
    pub system_address: i64,
    pub star_pos: Vec<f64>,
    pub star_class: String,
}