use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Powerplay {

    pub timestamp: String,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Rank")]
    pub rank: u8,

    #[serde(rename = "Merits")]
    pub merits: u32,

    #[serde(rename = "TimePledged")]
    pub time_pledged: u64
}