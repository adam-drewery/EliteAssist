use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FuelScoop {

    pub timestamp: String,

    #[serde(rename = "Scooped")]
    pub scooped: f64,

    #[serde(rename = "Total")]
    pub total: f64,
}