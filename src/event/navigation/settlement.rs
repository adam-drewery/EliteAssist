use crate::event::navigation::station::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct ApproachSettlement {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationFaction")]
    pub station_faction: Option<StationFaction>,

    #[serde(rename = "StationGovernment")]
    pub station_government: Option<String>,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: Option<String>,

    #[serde(rename = "StationServices")]
    pub station_services: Option<Vec<String>>,

    #[serde(rename = "StationEconomy")]
    pub station_economy: Option<String>,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: Option<String>,

    #[serde(rename = "StationEconomies")]
    pub station_economies: Option<Vec<StationEconomy>>,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "Latitude")]
    pub latitude: f64,

    #[serde(rename = "Longitude")]
    pub longitude: f64,
}