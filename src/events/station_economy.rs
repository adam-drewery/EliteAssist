use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct StationEconomy {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}