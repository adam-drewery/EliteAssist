use serde::Deserialize;

#[derive(Deserialize)]
pub struct SquadronStartup {

    pub timestamp: String,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,

    #[serde(rename = "CurrentRank")]
    pub current_rank: u8,
}