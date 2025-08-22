#[derive(Default)]
pub struct Rank {
    pub combat: u64,
    pub trade: u64,
    pub explore: u64,
    pub soldier: u64,
    pub exobiologist: u64,
    pub empire: u64,
    pub federation: u64,
    pub cqc: u64
}

#[derive(Default)]
pub struct CrimeStats {
    pub legal_state: String,
    pub active_fine: bool,
    pub wanted: bool,
}

#[derive(Default)]
pub struct Reputation {
    pub empire: f64,
    pub federation: f64,
    pub alliance: f64
}

use crate::journal::event;

impl From<event::Rank> for Rank {
    fn from(value: event::Rank) -> Self {
        Rank {
            combat: value.combat,
            trade: value.trade,
            explore: value.explore,
            soldier: value.soldier.unwrap_or_default(),
            exobiologist: value.exobiologist.unwrap_or_default(),
            empire: value.empire,
            federation: value.federation,
            cqc: value.cqc,
        }
    }
}

impl From<event::Reputation> for Reputation {
    fn from(value: event::Reputation) -> Self {
        Reputation {
            empire: value.empire.unwrap_or_default(),
            federation: value.federation.unwrap_or_default(),
            alliance: value.alliance.unwrap_or_default(),
        }
    }
}