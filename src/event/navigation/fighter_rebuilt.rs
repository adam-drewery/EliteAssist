use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FighterRebuilt {

    pub timestamp: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: i64,
}