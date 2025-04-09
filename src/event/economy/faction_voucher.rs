use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct FactionVoucher {

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Amount")]
    pub amount: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RedeemVoucher {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Amount")]
    pub amount: u32,

    #[serde(rename = "Factions")]
    pub factions: Option<Vec<FactionVoucher>>,
}