use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LaunchFighter {

    pub timestamp: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
}