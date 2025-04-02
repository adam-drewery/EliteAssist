use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BuySuit {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Price")]
    pub price: i64,

    #[serde(rename = "SuitID")]
    pub suit_id: i64,

    #[serde(rename = "SuitMods")]
    pub suit_mods: Vec<String>,
}