use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ReservoirReplenished {

    pub timestamp: String,

    #[serde(rename = "FuelMain")]
    pub fuel_main: f64,

    #[serde(rename = "FuelReservoir")]
    pub fuel_reservoir: f64,
}