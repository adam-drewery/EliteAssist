use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DropshipDeploy {
    
    pub timestamp: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "OnStation")]
    pub on_station: bool,

    #[serde(rename = "OnPlanet")]
    pub on_planet: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct BookDropship {
    
    pub timestamp: String,

    #[serde(rename = "Retreat")]
    pub retreat: bool,

    #[serde(rename = "Cost")]
    pub cost: u32,

    #[serde(rename = "DestinationSystem")]
    pub destination_system: String,

    #[serde(rename = "DestinationLocation")]
    pub destination_location: String,
}
