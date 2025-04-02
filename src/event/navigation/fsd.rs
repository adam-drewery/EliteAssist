use crate::event::navigation::faction::Faction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct SystemFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActiveState {

    #[serde(rename = "State")]
    pub state: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Power {

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "ConflictProgress")]
    pub conflict_progress: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FSDJump {

    pub timestamp: String,

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

    #[serde(rename = "Powers")]
    pub powers: Option<Vec<String>>,

    #[serde(rename = "PowerplayState")]
    pub powerplay_state: Option<String>,

    #[serde(rename = "PowerplayConflictProgress")]
    pub powerplay_conflict_progress: Option<Vec<Power>>,

    #[serde(rename = "JumpDist")]
    pub jump_dist: f64,

    #[serde(rename = "FuelUsed")]
    pub fuel_used: f64,

    #[serde(rename = "FuelLevel")]
    pub fuel_level: f64,

    #[serde(rename = "Factions")]
    pub factions: Option<Vec<Faction>>,

    #[serde(rename = "SystemFaction")]
    pub system_faction: Option<SystemFaction>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FSDTarget {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "StarClass")]
    pub star_class: String,

    #[serde(rename = "RemainingJumpsInRoute")]
    pub remaining_jumps_in_route: Option<u32>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SupercruiseEntry {

    pub timestamp: String,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SupercruiseExit {

    pub timestamp: String,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "BodyType")]
    pub body_type: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SupercruiseDestinationDrop {

    pub timestamp: String,


    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Threat")]
    pub threat: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct StartJump {

    pub timestamp: String,

    #[serde(rename = "JumpType")]
    pub jump_type: String,

    #[serde(rename = "Taxi")]
    pub taxi: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct JetConeBoost {

    pub timestamp: String,

    #[serde(rename = "BoostValue")]
    pub boost_value: f64,
}