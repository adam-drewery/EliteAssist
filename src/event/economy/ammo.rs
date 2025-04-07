use crate::event::format::prettify_date;
use crate::state::JournalEntry;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thousands::Separable;

#[derive(Deserialize, Debug, Clone)]
pub struct BuyAmmo {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Cost")]
    pub cost: u32
}

impl Into<JournalEntry> for BuyAmmo {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Bought ammo for".into(),
            noun: format!("{}CR", &self.cost.separate_with_commas())
        }
    }
}

