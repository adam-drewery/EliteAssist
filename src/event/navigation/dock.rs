use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct DockingCancelled {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StationEconomies {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StationFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Docked {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationFaction")]
    pub station_faction: StationFaction,

    #[serde(rename = "StationGovernment")]
    pub station_government: String,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: Option<String>,

    #[serde(rename = "StationAllegiance")]
    pub station_allegiance: Option<String>,

    #[serde(rename = "StationServices")]
    pub station_services: Vec<String>,

    #[serde(rename = "StationEconomy")]
    pub station_economy: String,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: Option<String>,

    #[serde(rename = "StationEconomies")]
    pub station_economies: Vec<StationEconomies>,

    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,

    #[serde(rename = "ActiveFine")]
    pub active_fine: Option<bool>,

    #[serde(rename = "Wanted")]
    pub wanted: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Undocked {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Taxi")]
    pub taxi: Option<bool>,

    #[serde(rename = "Multicrew")]
    pub multicrew: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DockingDenied {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Reason")]
    pub reason: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DockingGranted {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "LandingPad")]
    pub landing_pad: u32,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct LandingPadSize {

    #[serde(rename = "Small")]
    pub small: u32,

    #[serde(rename = "Medium")]
    pub medium: u32,

    #[serde(rename = "Large")]
    pub large: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DockingRequested {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "LandingPads")]
    pub landing_pads: LandingPadSize,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DockingTimeout {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}