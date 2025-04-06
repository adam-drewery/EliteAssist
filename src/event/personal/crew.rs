use chrono::{DateTime, Utc};
use serde::Deserialize;
use thousands::Separable;
use crate::event::format::prettify_date;
use crate::state::JournalEntry;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewAssign {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "CrewID")]
    pub crew_id: u64,

    #[serde(rename = "Role")]
    pub role: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberJoins {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberQuits {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberRoleChange {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EndCrewSession {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "OnCrime")]
    pub on_crime: bool,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct NpcCrewRank {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "NpcCrewName")]
    pub npc_crew_name: String,

    #[serde(rename = "NpcCrewId")]
    pub npc_crew_id: u64,

    #[serde(rename = "RankCombat")]
    pub rank_combat: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ChangeCrewRole {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NpcCrewPaidWage {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "NpcCrewName")]
    pub npc_crew_name: String,

    #[serde(rename = "NpcCrewId")]
    pub npc_crew_id: u64,

    #[serde(rename = "Amount")]
    pub amount: u32,
}

impl Into<JournalEntry> for CrewAssign {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned".into(),
            noun: format!("{} as {}", self.name, self.role)
        }
    }
}

impl Into<JournalEntry> for CrewMemberJoins {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Crew Joined".into(),
            noun: format!("{} {}", self.crew, if self.telepresence { "remotely" } else { "to crew" }),
        }
    }
}

impl Into<JournalEntry> for CrewMemberQuits {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Crew Left".into(),
            noun: format!("{} {}", self.crew, if self.telepresence { "remote session" } else { "crew" }),
        }
    }
}
impl Into<JournalEntry> for CrewMemberRoleChange {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned role".into(),
            noun: format!("{} to {}", self.role, self.crew)
        }
    }
}

impl Into<JournalEntry> for EndCrewSession {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Ended".into(),
            noun: if self.telepresence { "remote session".into() } else { "crew session".into() },
        }
    }
}

impl Into<JournalEntry> for NpcCrewRank {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Promoted crew member".into(),
            noun: self.npc_crew_name,
        }
    }
}

impl Into<JournalEntry> for ChangeCrewRole {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Changed role to".into(),
            noun: self.role,
        }
    }
}

impl Into<JournalEntry> for NpcCrewPaidWage {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Paid".into(),
            noun: format!("{} to {}", self.amount.separate_with_commas(), self.npc_crew_name)
        }
    }
}