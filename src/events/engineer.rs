use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Engineer {

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u32,

    #[serde(rename = "Progress")]
    pub progress: String,

    #[serde(rename = "RankProgress")]
    pub rank_progress: Option<u8>,

    #[serde(rename = "Rank")]
    pub rank: Option<u8>,
}