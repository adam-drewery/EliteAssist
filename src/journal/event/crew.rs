use crate::journal::event;
use crate::state;
use crate::journal::format::prettify_date;
use thousands::Separable;

impl Into<state::GameEventLog> for event::CrewAssign {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned".into(),
            noun: format!("{} as {}", self.name, self.role)
        }
    }
}

impl event::CrewMember {
    pub fn into(self, verb: &str) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: format!("Crew {}", verb).into(),
            noun: format!("{} {}", self.crew, if self.telepresence.is_some_and(|x| { x }) { "remotely" } else { "to crew" }),
        }
    }
}

impl Into<state::GameEventLog> for event::CrewMemberRoleChange {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned role".into(),
            noun: format!("{} to {}", self.role, self.crew)
        }
    }
}

impl Into<state::GameEventLog> for event::EndCrewSession {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Ended".into(),
            noun: if self.telepresence.is_some_and(|x| { x }) { "remote session".into() } else { "crew session".into() },
        }
    }
}

impl Into<state::GameEventLog> for event::NpcCrewRank {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Promoted crew member".into(),
            noun: self.npc_crew_name,
        }
    }
}

impl Into<state::GameEventLog> for event::ChangeCrewRole {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Changed role to".into(),
            noun: self.role,
        }
    }
}

impl Into<state::GameEventLog> for event::NpcCrewPaidWage {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Paid".into(),
            noun: format!("{} to {}", self.amount.separate_with_commas(), self.npc_crew_name)
        }
    }
}