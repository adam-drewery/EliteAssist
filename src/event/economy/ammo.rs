use crate::event::format::prettify_date;
use crate::state::GameActivity;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use thousands::Separable;

#[derive(Clone, Debug, Deserialize)]
pub struct BuyAmmo {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Cost")]
    pub cost: u32
}

impl Into<GameActivity> for BuyAmmo {
    fn into(self) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: "Bought ammo for".into(),
            noun: format!("{}CR", &self.cost.separate_with_commas())
        }
    }
}

