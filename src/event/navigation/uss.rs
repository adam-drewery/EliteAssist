use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct USSDrop {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "USSType")]
    pub usstype: String,

    #[serde(rename = "USSType_Localised")]
    pub usstype_localised: String,

    #[serde(rename = "USSThreat")]
    pub ussthreat: u32,
}