use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ClearSavedGame {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FID")]
    pub fid: String,
}