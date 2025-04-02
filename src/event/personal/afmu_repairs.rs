use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct AfmuRepairs {

    pub timestamp: String,

    #[serde(rename = "Module")]
    pub module: String,

    #[serde(rename = "Module_Localised")]
    pub module_localised: String,

    #[serde(rename = "FullyRepaired")]
    pub fully_repaired: bool,

    #[serde(rename = "Health")]
    pub health: f64,
}