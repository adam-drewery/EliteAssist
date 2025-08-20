use crate::journal::event;
use crate::state;
use crate::journal::format::prettify_date;

impl Into<state::GameEventLog> for event::FighterRebuilt {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time_display: prettify_date(&self.timestamp),
            verb: "Rebuilt".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

impl Into<state::GameEventLog> for event::DockFighter {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time_display: prettify_date(&self.timestamp),
            verb: "Docked".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

impl Into<state::GameEventLog> for event::CrewLaunchFighter {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time_display: prettify_date(&self.timestamp),
            verb: "Launched".into(),
            noun: format!["Fighter by {}", self.crew],
        }
    }
}

impl Into<state::GameEventLog> for event::LaunchFighter {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time_display: prettify_date(&self.timestamp),
            verb: "Launched".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

impl event::Damage {
    pub fn into(self, verb: &str, noun: &str) -> state::GameEventLog {
        state::GameEventLog {
            time_display: prettify_date(&self.timestamp),
            verb: verb.into(),
            noun: match self.id {
                None => noun.into(),
                Some(id) => format!["{} {}", noun, id],
            },
        }
    }
}

impl Into<state::GameEventLog> for event::VehicleSwitch {
    fn into(self) -> state::GameEventLog {
        state::GameEventLog {
            time_display: prettify_date(&self.timestamp),
            verb: "Switched to".into(),
            noun: self.to,
        }
    }
}
