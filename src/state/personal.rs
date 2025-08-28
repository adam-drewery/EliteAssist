use crate::journal::event;

#[derive(Default)]
pub struct Rank {
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
    pub legal_state: Box<str>,
    pub active_fine: bool,
    pub wanted: bool,
}

#[derive(Default)]
pub struct Reputation {
    pub empire: f64,
    pub federation: f64,
    pub alliance: f64
}

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
            empire: value.empire.map(|v| v as f64).unwrap_or_default(),
            federation: value.federation.map(|v| v as f64).unwrap_or_default(),
            alliance: value.alliance.unwrap_or_default(),
        }
    }
}