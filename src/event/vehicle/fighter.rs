use crate::event::format::prettify_date;
use crate::state::GameActivity;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FighterDestroyed {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ID")]
    pub id: u64,
}

impl Into<GameActivity> for FighterDestroyed {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Destroyed".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FighterRebuilt {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,
}

impl Into<GameActivity> for FighterRebuilt {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Rebuilt".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct DockFighter {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ID")]
    pub id: u64,
}

impl Into<GameActivity> for DockFighter {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Docked".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrewLaunchFighter {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,

    #[serde(rename = "Crew")]
    pub crew: String,
}

impl Into<GameActivity> for CrewLaunchFighter {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Launched".into(),
            noun: format!["Fighter by {}", self.crew],
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct LaunchFighter {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
}

impl Into<GameActivity> for LaunchFighter {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Launched".into(),
            noun: format!["Fighter {}", self.id.to_string()],
        }
    }
}

