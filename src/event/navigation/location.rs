use crate::event::navigation::faction::Faction;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::event::format::prettify_date;
use crate::state::JournalEntry;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Location {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: Option<f64>,

    #[serde(rename = "Docked")]
    pub docked: bool,

    #[serde(rename = "StationName")]
    pub station_name: Option<String>,

    #[serde(rename = "StationType")]
    pub station_type: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    #[serde(rename = "StationFaction")]
    pub station_faction: Option<SystemFaction>,

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

    #[serde(rename = "Taxi")]
    pub taxi: Option<bool>,

    #[serde(rename = "Multicrew")]
    pub multicrew: Option<bool>,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "StarPos")]
    pub star_pos: Vec<f64>,

    #[serde(rename = "SystemAllegiance")]
    pub system_allegiance: String,

    #[serde(rename = "SystemEconomy")]
    pub system_economy: String,

    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: String,

    #[serde(rename = "SystemSecondEconomy")]
    pub system_second_economy: String,

    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: String,

    #[serde(rename = "SystemGovernment")]
    pub system_government: String,

    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: String,

    #[serde(rename = "SystemSecurity")]
    pub system_security: String,

    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: String,

    #[serde(rename = "Population")]
    pub population: u64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "BodyType")]
    pub body_type: String,

    #[serde(rename = "ControllingPower")]
    pub controlling_power: Option<String>,

    #[serde(rename = "Powers")]
    pub powers: Option<Vec<String>>,

    #[serde(rename = "PowerplayState")]
    pub powerplay_state: Option<String>,

    #[serde(rename = "PowerplayStateControlProgress")]
    pub powerplay_state_control_progress: Option<f64>,

    #[serde(rename = "PowerplayStateReinforcement")]
    pub powerplay_state_reinforcement: Option<u32>,

    #[serde(rename = "PowerplayStateUndermining")]
    pub powerplay_state_undermining: Option<u32>,

    #[serde(rename = "Factions")]
    pub factions: Option<Vec<Faction>>,

    #[serde(rename = "SystemFaction")]
    pub system_faction: Option<SystemFaction>
}

#[derive(Deserialize, Debug, Clone)]
pub struct StationEconomy {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SystemFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,
}

impl Into<JournalEntry> for Location {

    fn into(self) -> JournalEntry {
        JournalEntry {
            time: self.timestamp,
            time_display : prettify_date(&self.timestamp),
            star_system: self.star_system,
            station: self.station_name.clone(),
            body: self.body,
            text: format!("New location {}", self.station_name.unwrap_or("Unknown Station".to_string())),
        }
    }
}