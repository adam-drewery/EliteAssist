use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BookDropship {

    pub timestamp: String,

    #[serde(rename = "Retreat")]
    pub retreat: bool,

    #[serde(rename = "Cost")]
    pub cost: i64,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: String,

    #[serde(rename = "DestinationLocation")]
    pub destination_location: String,
}