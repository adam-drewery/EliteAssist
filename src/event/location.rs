use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Location {

    pub timestamp: String,

    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,

    #[serde(rename = "Docked")]
    pub docked: bool,

    #[serde(rename = "StationName")]
    pub station_name: Option<String>,

    #[serde(rename = "StationType")]
    pub station_type: Option<String>,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

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

#[derive(Deserialize, Debug, Clone)]
pub struct StationEconomy {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct StationFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Faction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: String,

    #[serde(rename = "Government")]
    pub government: String,

    #[serde(rename = "Influence")]
    pub influence: f64,

    #[serde(rename = "Allegiance")]
    pub allegiance: String,

    #[serde(rename = "Happiness")]
    pub happiness: String,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: String,

    #[serde(rename = "MyReputation")]
    pub my_reputation: f64,

    #[serde(rename = "RecoveringStates")]
    pub recovering_states: Option<Vec<FactionState>>,

    #[serde(rename = "ActiveStates")]
    pub active_states: Option<Vec<FactionState>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct FactionState {

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Trend")]
    pub trend: Option<u8>,
}