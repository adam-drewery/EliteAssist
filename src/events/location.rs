use serde::Deserialize;
use crate::events::faction::Faction;
use crate::events::station_economy::StationEconomy;
use crate::events::station_faction::StationFaction;

#[derive(Debug, Deserialize)]
pub struct Location {

    pub timestamp: String,

    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,

    #[serde(rename = "Docked")]
    pub docked: bool,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationFaction")]
    pub station_faction: StationFaction,

    #[serde(rename = "StationGovernment")]
    pub station_government: String,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: String,

    #[serde(rename = "StationServices")]
    pub station_services: Vec<String>,

    #[serde(rename = "StationEconomy")]
    pub station_economy: String,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: String,

    #[serde(rename = "StationEconomies")]
    pub station_economies: Vec<StationEconomy>,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

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
    pub controlling_power: String,

    #[serde(rename = "Powers")]
    pub powers: Vec<String>,

    #[serde(rename = "PowerplayState")]
    pub powerplay_state: String,

    #[serde(rename = "PowerplayStateControlProgress")]
    pub powerplay_state_control_progress: f64,

    #[serde(rename = "PowerplayStateReinforcement")]
    pub powerplay_state_reinforcement: u64,

    #[serde(rename = "PowerplayStateUndermining")]
    pub powerplay_state_undermining: u64,

    #[serde(rename = "Factions")]
    pub factions: Vec<Faction>,

    #[serde(rename = "SystemFaction")]
    pub system_faction: StationFaction
}