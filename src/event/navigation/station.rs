use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct StationEconomy {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StationFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>
}