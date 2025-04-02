use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
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

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PowerplayJoin {

    pub timestamp: String,

    #[serde(rename = "Power")]
    pub power: String,
}