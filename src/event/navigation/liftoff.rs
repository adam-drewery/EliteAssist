use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Liftoff {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,

    #[serde(rename = "Taxi")]
    pub taxi: Option<bool>,

    #[serde(rename = "Multicrew")]
    pub multicrew: Option<bool>,

    #[serde(rename = "StarSystem")]
    pub star_system: Option<String>,

    #[serde(rename = "SystemAddress")]
    pub system_address: Option<u64>,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "OnStation")]
    pub on_station: Option<bool>,

    #[serde(rename = "OnPlanet")]
    pub on_planet: Option<bool>,

    #[serde(rename = "Latitude")]
    pub latitude: f64,

    #[serde(rename = "Longitude")]
    pub longitude: f64,

    #[serde(rename = "NearestDestination")]
    pub nearest_destination: Option<String>,

    #[serde(rename = "NearestDestination_Localised")]
    pub nearest_destination_localised: Option<String>,
}