use thousands::Separable;
use crate::{event, state};
use crate::event::format::prettify_date;

impl Into<state::GameActivity> for event::CrewAssign {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned".into(),
            noun: format!("{} as {}", self.name, self.role)
        }
    }
}

impl event::CrewMember {
    pub fn into(self, verb: &str) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: format!("Crew {}", verb).into(),
            noun: format!("{} {}", self.crew, if self.telepresence.is_some_and(|x| { x }) { "remotely" } else { "to crew" }),
        }
    }
}

impl Into<state::GameActivity> for event::CrewMemberRoleChange {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned role".into(),
            noun: format!("{} to {}", self.role, self.crew)
        }
    }
}

impl Into<state::GameActivity> for event::EndCrewSession {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Ended".into(),
            noun: if self.telepresence.is_some_and(|x| { x }) { "remote session".into() } else { "crew session".into() },
        }
    }
}

impl Into<state::GameActivity> for event::NpcCrewRank {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Promoted crew member".into(),
            noun: self.npc_crew_name,
        }
    }
}

impl Into<state::GameActivity> for event::ChangeCrewRole {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Changed role to".into(),
            noun: self.role,
        }
    }
}

impl Into<state::GameActivity> for event::NpcCrewPaidWage {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Paid".into(),
            noun: format!("{} to {}", self.amount.separate_with_commas(), self.npc_crew_name)
        }
    }
}