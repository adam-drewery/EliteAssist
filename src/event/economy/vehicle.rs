use chrono::{DateTime, Utc};
use serde::Deserialize;
use thousands::Separable;
use crate::event::format::prettify_date;
use crate::state::JournalEntry;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RestockVehicle {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub type_: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "Cost")]
    pub cost: u64,

    #[serde(rename = "Count")]
    pub count: u64
}

impl Into<JournalEntry> for RestockVehicle {
    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Restocked".into(),
            noun: format!("{} {} for {}CR", self.count, self.type_, self.cost.separate_with_commas()),
        }
    }
}

