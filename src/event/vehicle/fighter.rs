use crate::event::format::prettify_date;
use crate::state::JournalEntry;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FighterDestroyed {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ID")]
    pub id: u64,
}

impl Into<JournalEntry> for FighterDestroyed {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Destroyed fighter".into(),
            noun: self.id.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FighterRebuilt {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,
}

impl Into<JournalEntry> for FighterRebuilt {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Rebuilt fighter".into(),
            noun: self.id.to_string()
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct DockFighter {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ID")]
    pub id: u64,
}

impl Into<JournalEntry> for DockFighter {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Docked fighter".into(),
            noun: self.id.to_string(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct CrewLaunchFighter {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,

    #[serde(rename = "Crew")]
    pub crew: String,
}

impl Into<JournalEntry> for CrewLaunchFighter {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Launched fighter by".into(),
            noun: format!("{}", self.crew),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
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

impl Into<JournalEntry> for LaunchFighter {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Launched fighter".into(),
            noun: self.id.to_string(),
        }
    }
}

