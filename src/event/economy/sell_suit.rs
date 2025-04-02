use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SellSuit {

    pub timestamp: String,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitMods")]
    pub suit_mods: Vec<String>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Price")]
    pub price: u64,
}