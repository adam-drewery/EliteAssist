use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct LaunchSRV {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SRVType")]
    pub srvtype: String,

    #[serde(rename = "SRVType_Localised")]
    pub srvtype_localised: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DockSRV {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SRVType")]
    pub srvtype: String,

    #[serde(rename = "SRVType_Localised")]
    pub srvtype_localised: String,

    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SrvDestroyed {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "SRVType")]
    pub srvtype: String,

    #[serde(rename = "SRVType_Localised")]
    pub srvtype_localised: String,
}