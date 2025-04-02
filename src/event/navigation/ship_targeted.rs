use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ShipTargeted {

    pub timestamp: String,

    #[serde(rename = "TargetLocked")]
    pub target_locked: bool,

    #[serde(rename = "Ship")]
    pub ship: Option<String>,

    #[serde(rename = "ScanStage")]
    pub scan_stage: Option<i64>,
}