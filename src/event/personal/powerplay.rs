use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Powerplay {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Rank")]
    pub rank: u8,

    #[serde(rename = "Merits")]
    pub merits: u32,

    #[serde(rename = "TimePledged")]
    pub time_pledged: u64
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayVote {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Votes")]
    pub votes: u32,

    #[serde(rename = "VoteToConsolidate")]
    pub vote_to_consolidate: u32,

    #[serde(rename = "System")]
    pub system: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayJoin {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayDefect {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "FromPower")]
    pub from_power: String,

    #[serde(rename = "ToPower")]
    pub to_power: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayDeliver {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Type")]
    pub type_name: String,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplaySalary {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Amount")]
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayLeave {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayFastTrack {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Cost")]
    pub cost: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayCollect {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Type")]
    pub type_name: String,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PowerplayVoucher {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "Systems")]
    pub systems: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Power {

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "ConflictProgress")]
    pub conflict_progress: f64,
}