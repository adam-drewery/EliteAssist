use crate::event::format::prettify_date;
use crate::state::GameActivity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thousands::Separable;

#[derive(Clone, Debug, Deserialize)]
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

impl Into<GameActivity> for RestockVehicle {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Restocked".into(),
            noun: format!("{} {} for {}CR", self.count, self.type_, self.cost.separate_with_commas()),
        }
    }
}

