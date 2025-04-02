use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Interdiction {

    pub timestamp: String,

    #[serde(rename = "Success")]
    pub success: bool,

    #[serde(rename = "IsPlayer")]
    pub is_player: bool,

    #[serde(rename = "Faction")]
    pub faction: String,
}