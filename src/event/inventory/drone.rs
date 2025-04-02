use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LaunchDrone {

    pub timestamp: String,

    #[serde(rename = "Type")]
    pub type_: String,
}