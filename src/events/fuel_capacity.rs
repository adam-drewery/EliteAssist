use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FuelCapacity {

    #[serde(rename = "Main")]
    pub main: f64,

    #[serde(rename = "Reserve")]
    pub reserve: f64,
}