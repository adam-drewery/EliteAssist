use crate::event::format::prettify_date;
use crate::{event, state};

impl Into<state::GameActivity> for event::FighterRebuilt {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Rebuilt".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

impl Into<state::GameActivity> for event::DockFighter {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Docked".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

impl Into<state::GameActivity> for event::CrewLaunchFighter {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Launched".into(),
            noun: format!["Fighter by {}", self.crew],
        }
    }
}

impl Into<state::GameActivity> for event::LaunchFighter {
    fn into(self) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Launched".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

impl event::Damage {
    pub fn into(self, verb: &str, noun: &str) -> state::GameActivity {
        state::GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: verb.into(),
            noun: noun.into(),
        }
    }
}