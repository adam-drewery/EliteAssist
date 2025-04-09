use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Reputation {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Empire")]
    pub empire: f64,

    #[serde(rename = "Federation")]
    pub federation: f64,

    #[serde(rename = "Independent")]
    pub independent: f64,

    #[serde(rename = "Alliance")]
    pub alliance: f64
}

impl Into<crate::state::Reputation> for Reputation {
    fn into(self) -> crate::state::Reputation {
        crate::state::Reputation {
            timestamp: self.timestamp,
            empire: self.empire,
            federation: self.federation,
            independent: self.independent,
            alliance: self.alliance,
        }
    }
}