use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct NavRouteStep {
    #[serde(rename = "StarSystem")]
    pub star_system: String,
    #[serde(rename = "SystemAddress")]
    pub system_address: i64,
    #[serde(rename = "StarPos")]
    pub star_pos: Vec<f64>,
    #[serde(rename = "StarClass")]
    pub star_class: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NavRoute {
    pub timestamp: String,
    #[serde(rename = "Route")]
    pub route: Option<Vec<NavRouteStep>>,
}

impl NavRoute {
    pub fn into(self) -> Vec<crate::state::NavRouteStep> {
        match self.route {
            Some(route) => route
                .into_iter()
                .map(|step| crate::state::NavRouteStep {
                    star_system: step.star_system,
                    system_address: step.system_address,
                    star_pos: step.star_pos,
                    star_class: step.star_class,
                })
                .collect(),
            None => Vec::new()
        }
    }
}
