use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ShipTargeted {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "TargetLocked")]
    pub target_locked: bool,

    #[serde(rename = "Ship")]
    pub ship: Option<String>,

    #[serde(rename = "ScanStage")]
    pub scan_stage: Option<u32>,
}