use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RenameSuitLoadout {

    pub timestamp: String,

    #[serde(rename = "SuitID")]
    pub suit_id: u64,

    #[serde(rename = "SuitName")]
    pub suit_name: String,

    #[serde(rename = "SuitName_Localised")]
    pub suit_name_localised: String,

    #[serde(rename = "LoadoutID")]
    pub loadout_id: u64,

    #[serde(rename = "LoadoutName")]
    pub loadout_name: String,
}