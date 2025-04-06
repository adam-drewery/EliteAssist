use crate::event::format::prettify_date;
use crate::state::JournalEntry;
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

impl Into<JournalEntry> for Embark {

    fn into(self) -> JournalEntry {

        JournalEntry {
            time: self.timestamp,
            time_display: prettify_date(&self.timestamp),
            text: "Disembarked".to_owned(),
            star_system: self.star_system,
            station: self.station_name,
            body: self.body,
        }
    }
}