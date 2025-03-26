use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FuelCapacity {

    #[serde(rename = "Main")]
    pub main: f64,

    #[serde(rename = "Reserve")]
    pub reserve: f64,
}