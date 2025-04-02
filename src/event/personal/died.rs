use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Died {

    pub timestamp: String,

    #[serde(rename = "KillerName")]
    pub killer_name: Option<String>,

    #[serde(rename = "KillerShip")]
    pub killer_ship: Option<String>,

    #[serde(rename = "KillerRank")]
    pub killer_rank: Option<String>,
}