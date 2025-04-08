use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct EngineerProgress {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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

impl Into<crate::state::EngineerProgress> for EngineerProgress {
    fn into(self) -> crate::state::EngineerProgress {
        crate::state::EngineerProgress {
            timestamp: self.timestamp,
            engineers: self.engineers
                .unwrap_or_default()
                .into_iter()
                .map(|e| e.into())
                .collect(),
        }
    }
}

impl Into<crate::state::Engineer> for Engineer {
    fn into(self) -> crate::state::Engineer {
        crate::state::Engineer {
            engineer: self.engineer,
            engineer_id: self.engineer_id,
            progress: self.progress,
            rank_progress: self.rank_progress.unwrap_or_default(),
            rank: self.rank.unwrap_or_default(),
        }
    }
}

