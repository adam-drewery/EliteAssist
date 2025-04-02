mod dropship;
mod fighter;
mod ship;
mod srv;
mod taxi;

pub use dropship::*;
pub use fighter::*;
use serde::Deserialize;
pub use ship::*;
pub use srv::*;
pub use taxi::*;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct VehicleSwitch {
    pub timestamp: String,

    #[serde(rename = "To")]
    pub to: String,
}
