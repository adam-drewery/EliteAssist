use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct AppliedToSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SquadronStartup {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,

    #[serde(rename = "CurrentRank")]
    pub current_rank: u8,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DisbandedSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct InvitedToSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JoinedSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct KickedFromSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LeftSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SharedBookmarkToSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SquadronCreated {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SquadronDemotion {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,

    #[serde(rename = "OldRank")]
    pub old_rank: u8,

    #[serde(rename = "NewRank")]
    pub new_rank: u8,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SquadronPromotion {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,

    #[serde(rename = "OldRank")]
    pub old_rank: u8,

    #[serde(rename = "NewRank")]
    pub new_rank: u8,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WonATrophyForSquadron {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SquadronName")]
    pub squadron_name: String,
}