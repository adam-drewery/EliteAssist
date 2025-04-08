use crate::event::format::prettify_date;
use crate::state::GameActivity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thousands::Separable;

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct CrewMemberJoins {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CrewMemberQuits {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct EndCrewSession {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "OnCrime")]
    pub on_crime: bool,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
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

impl Into<GameActivity> for CrewAssign {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned".into(),
            noun: format!("{} as {}", self.name, self.role)
        }
    }
}

impl Into<GameActivity> for CrewMemberJoins {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Crew Joined".into(),
            noun: format!("{} {}", self.crew, if self.telepresence { "remotely" } else { "to crew" }),
        }
    }
}

impl Into<GameActivity> for CrewMemberQuits {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Crew Left".into(),
            noun: format!("{} {}", self.crew, if self.telepresence { "remote session" } else { "crew" }),
        }
    }
}
impl Into<GameActivity> for CrewMemberRoleChange {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Assigned role".into(),
            noun: format!("{} to {}", self.role, self.crew)
        }
    }
}

impl Into<GameActivity> for EndCrewSession {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Ended".into(),
            noun: if self.telepresence { "remote session".into() } else { "crew session".into() },
        }
    }
}

impl Into<GameActivity> for NpcCrewRank {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Promoted crew member".into(),
            noun: self.npc_crew_name,
        }
    }
}

impl Into<GameActivity> for ChangeCrewRole {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Changed role to".into(),
            noun: self.role,
        }
    }
}

impl Into<GameActivity> for NpcCrewPaidWage {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Paid".into(),
            noun: format!("{} to {}", self.amount.separate_with_commas(), self.npc_crew_name)
        }
    }
}