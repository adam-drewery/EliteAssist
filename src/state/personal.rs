use ed_journals::logs::rank_event::RankEvent;
use ed_journals::logs::reputation_event::ReputationEvent;
use ed_journals::logs::progress_event::ProgressEvent;

#[derive(Default)]
pub struct Rank {
    pub combat: String,
    pub trade: String,
    pub explore: String,
    pub mercenary: String,
    pub exobiology: String,
    pub empire: String,
    pub federation: String,
    pub cqc: String
}


#[derive(Default)]
pub struct Progress {
    pub combat: u8,
    pub trade: u8,
    pub explore: u8,
    pub soldier: u8,
    pub exobiologist: u8,
    pub empire: u8,
    pub federation: u8,
    pub cqc: u8
}

#[derive(Default)]
pub struct CrimeStats {
    pub legal_state: String,
    pub active_fine: bool,
    pub wanted: bool,
}

#[derive(Default)]
pub struct Reputation {
    pub empire: f32,
    pub federation: f32,
    pub alliance: f32
}

impl From<RankEvent> for Rank {
    fn from(value: RankEvent) -> Self {
        Rank {
            combat: value.combat.to_string(),
            trade: value.trade.to_string(),
            explore: value.explore.to_string(),
            mercenary: value.mercenary.to_string(),
            exobiology: value.exobiology.to_string(),
            empire: value.empire.to_string(),
            federation: value.federation.to_string(),
            cqc: value.cqc.to_string(),
        }
    }
}


impl From<ProgressEvent> for Progress {
    fn from(value: ProgressEvent) -> Self {
        Progress {
            combat: value.combat,
            trade: value.trade,
            explore: value.explore,
            soldier: value.mercenary,
            exobiologist: value.exobiology,
            empire: value.empire,
            federation: value.federation,
            cqc: value.cqc,
        }
    }
}

impl From<ReputationEvent> for Reputation {
    fn from(value: ReputationEvent) -> Self {
        Reputation {
            empire: value.empire.unwrap_or_default(),
            federation: value.federation.unwrap_or_default(),
            alliance: value.alliance.unwrap_or_default(),
        }
    }
}