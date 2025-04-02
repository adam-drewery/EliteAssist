use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EscapeInterdiction {

    pub timestamp: String,

    #[serde(rename = "Interdictor")]
    pub interdictor: String,

    #[serde(rename = "IsPlayer")]
    pub is_player: bool,
}