use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WingAdd {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WingInvite {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WingJoin {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Others")]
    pub others: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WingLeave {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,
}