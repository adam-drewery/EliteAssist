use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Interdicted {

    pub timestamp: String,

    #[serde(rename = "Submitted")]
    pub submitted: bool,

    #[serde(rename = "Interdictor")]
    pub interdictor: Option<String>,

    #[serde(rename = "IsPlayer")]
    pub is_player: bool,

    #[serde(rename = "Faction")]
    pub faction: Option<String>,
}