use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EngineerProgress {

    pub timestamp: String,

    #[serde(rename = "Engineers")]
    pub engineers: Option<Vec<Engineer>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Engineer {

    #[serde(rename = "Engineer")]
    pub engineer: String,

    #[serde(rename = "EngineerID")]
    pub engineer_id: u64,

    #[serde(rename = "Progress")]
    pub progress: String,

    #[serde(rename = "RankProgress")]
    pub rank_progress: Option<u8>,

    #[serde(rename = "Rank")]
    pub rank: Option<u8>,
}