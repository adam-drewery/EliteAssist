#[derive(Default)]
pub struct Engineer {
    pub engineer: String,
    pub engineer_id: u64,
    pub progress: String,
    pub rank_progress: u64,
    pub rank: u64,
}

use crate::journal::event;

impl From<event::EngineerProgressEngineer> for Engineer {
    fn from(value: event::EngineerProgressEngineer) -> Self {
        Engineer {
            engineer: value.engineer,
            engineer_id: value.engineer_id,
            progress: value.progress,
            rank_progress: value.rank_progress.unwrap_or_default(),
            rank: value.rank.unwrap_or_default(),
        }
    }
}

impl From<event::EngineerProgress> for Vec<Engineer> {
    fn from(value: event::EngineerProgress) -> Self {
        value
            .engineers
            .unwrap_or_default()
            .into_iter()
            .map(|e| e.into())
            .collect()
    }
}
