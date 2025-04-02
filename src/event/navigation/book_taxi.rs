use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BookTaxi {
    pub timestamp: String,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: String,

    #[serde(rename = "DestinationLocation")]
    pub destination_location: String,
}