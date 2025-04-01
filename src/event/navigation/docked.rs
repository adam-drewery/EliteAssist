use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct StationEconomies {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Proportion")]
    pub proportion: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StationFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Docked {

    pub timestamp: String,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "StationFaction")]
    pub station_faction: StationFaction,

    #[serde(rename = "StationGovernment")]
    pub station_government: String,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: String,

    #[serde(rename = "StationAllegiance")]
    pub station_allegiance: Option<String>,

    #[serde(rename = "StationServices")]
    pub station_services: Vec<String>,

    #[serde(rename = "StationEconomy")]
    pub station_economy: String,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: String,

    #[serde(rename = "StationEconomies")]
    pub station_economies: Vec<StationEconomies>,

    #[serde(rename = "DistFromStarLS")]
    pub dist_from_star_ls: f64,

    #[serde(rename = "ActiveFine")]
    pub active_fine: Option<bool>,

    #[serde(rename = "Wanted")]
    pub wanted: Option<bool>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Undocked {

    pub timestamp: String,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,
}