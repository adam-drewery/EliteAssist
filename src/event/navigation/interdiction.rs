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

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EscapeInterdiction {

    pub timestamp: String,

    #[serde(rename = "Interdictor")]
    pub interdictor: String,

    #[serde(rename = "IsPlayer")]
    pub is_player: bool,
}

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