mod dropship;
mod fighter;
mod ship;
mod srv;
mod taxi;

use chrono::{DateTime, Utc};
pub use dropship::*;
pub use fighter::*;
use serde::Deserialize;
pub use ship::*;
pub use srv::*;
pub use taxi::*;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct VehicleSwitch {
    
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "To")]
    pub to: String,
}
