use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Friends {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,
    
    #[serde(rename = "Status")]
    pub status: String,
    
    #[serde(rename = "Name")]
    pub name: String,
}