use crate::event::format::prettify_date;
use crate::state::GameActivity;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Embark {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SRV")]
    pub srv: bool,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "ID")]
    pub id: Option<u64>,

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

    #[serde(rename = "StationName")]
    pub station_name: Option<String>,

    #[serde(rename = "StationType")]
    pub station_type: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,
}

impl Embark {
    pub fn into(self, verb: &str) -> GameActivity {
        GameActivity {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            verb: verb.to_owned(),
            noun: join_location_parts(&self.star_system, &self.body, &self.station_name),
        }
    }
}

fn join_location_parts(system: &String, body: &String, station: &Option<String>) -> String {
    let mut parts = Vec::new();

    if !system.is_empty() {
        parts.push(system.as_str());
    }
    if !body.is_empty() {
        parts.push(body.as_str());
    }
    if let Some(station) = station {
        if !station.is_empty() && !Some(station.to_string()).eq(&Some(body.to_string())) {
            parts.push(station.as_str());
        }
    }
    parts.join(" | ")
}
