use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[derive(Default)]
pub struct Commander {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "FID")]
    pub fid: String,

    #[serde(rename = "Name")]
    pub name: String
}