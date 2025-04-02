use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FactionKillBond {

    pub timestamp: String,

    #[serde(rename = "Reward")]
    pub reward: i64,

    #[serde(rename = "AwardingFaction")]
    pub awarding_faction: String,

    #[serde(rename = "VictimFaction")]
    pub victim_faction: String,
}