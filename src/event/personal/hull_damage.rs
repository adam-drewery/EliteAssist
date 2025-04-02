use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct HullDamage {

    pub timestamp: String,

    #[serde(rename = "Health")]
    pub health: f64,

    #[serde(rename = "PlayerPilot")]
    pub player_pilot: bool,

    #[serde(rename = "Fighter")]
    pub fighter: Option<bool>,
}