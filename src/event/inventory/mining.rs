use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct MiningRefined {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: String,
}