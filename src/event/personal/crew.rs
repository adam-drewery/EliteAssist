use crate::event::format::prettify_date;
use crate::state::GameActivity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thousands::Separable;

#[derive(Clone, Debug, Deserialize)]
pub struct CrewFire {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrewHire {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "CombatRank")]
    pub combat_rank: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrewAssign {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "CrewID")]
    pub crew_id: Option<u64>,

    #[serde(rename = "Role")]
    pub role: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrewMemberJoins {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrewMemberQuits {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CrewMemberRoleChange {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EndCrewSession {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "OnCrime")]
    pub on_crime: bool,

    #[serde(rename = "Telepresence")]
    pub telepresence: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct ChangeCrewRole {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct QuitACrew {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Captain")]
    pub captain: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinACrew {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Captain")]
    pub captain: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KickCrewMember {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "OnCrime")]
    pub on_crime: bool,
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
            noun: format!("{} {}", self.crew, if self.telepresence.unwrap_or_default() { "remotely" } else { "to crew" }),
        }
    }
}

impl Into<GameActivity> for CrewMemberQuits {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Crew Left".into(),
            noun: format!("{} {}", self.crew, if self.telepresence.unwrap_or_default() { "remote session" } else { "crew" }),
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
            noun: if self.telepresence.unwrap_or_default() { "remote session".into() } else { "crew session".into() },
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