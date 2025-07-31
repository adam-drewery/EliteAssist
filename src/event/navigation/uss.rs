use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct USSDrop {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "USSType")]
    pub uss_type: String,

    #[serde(rename = "USSType_Localised")]
    pub uss_type_localised: Option<String>,

    #[serde(rename = "USSThreat")]
    pub uss_threat: u32,
}