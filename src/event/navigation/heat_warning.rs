use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct HeatWarning {

    pub timestamp: String,
}