use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Resurrect {

    pub timestamp: String,

    #[serde(rename = "Option")]
    pub option: String,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "Bankrupt")]
    pub bankrupt: bool,
}