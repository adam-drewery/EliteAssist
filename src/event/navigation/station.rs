use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StationEconomy {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StationFaction {

    #[serde(rename = "Name")]
    pub name: String,
}