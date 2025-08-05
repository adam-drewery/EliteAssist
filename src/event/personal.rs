use crate::{event, state};

impl Into<state::Rank> for event::Rank {

    fn into(self) -> state::Rank {

        state::Rank {
            timestamp: self.timestamp,
            combat: self.combat,
            trade: self.trade,
            explore: self.explore,
            soldier: self.soldier.unwrap_or_default(),
            exobiologist: self.exobiologist.unwrap_or_default(),
            empire: self.empire,
            federation: self.federation,
            cqc: self.cqc,
        }
    }
}

impl Into<state::Reputation> for event::Reputation {
    fn into(self) -> state::Reputation {
        state::Reputation {
            timestamp: self.timestamp,
            empire: self.empire.unwrap_or_default(),
            federation: self.federation.unwrap_or_default(),
            independent: self.independent.unwrap_or_default(),
            alliance: self.alliance.unwrap_or_default(),
        }
    }
}