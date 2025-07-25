use crate::event::navigation::faction::Faction;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::event::format::prettify_date;
use crate::state::GameActivity;

#[derive(Clone, Debug, Deserialize)]
pub struct SystemFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActiveState {

    #[serde(rename = "State")]
    pub state: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Power {

    #[serde(rename = "Power")]
    pub power: String,

    #[serde(rename = "ConflictProgress")]
    pub conflict_progress: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FSDJump {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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

#[derive(Clone, Debug, Deserialize)]
pub struct FSDTarget {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "StarClass")]
    pub star_class: String,

    #[serde(rename = "RemainingJumpsInRoute")]
    pub remaining_jumps_in_route: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SupercruiseEntry {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "Multicrew")]
    pub multicrew: bool,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SupercruiseExit {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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

#[derive(Clone, Debug, Deserialize)]
pub struct SupercruiseDestinationDrop {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,


    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Threat")]
    pub threat: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StartJump {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "JumpType")]
    pub jump_type: String,

    #[serde(rename = "Taxi")]
    pub taxi: bool,

    #[serde(rename = "StarSystem")]
    pub star_system: Option<String>,

    #[serde(rename = "SystemAddress")]
    pub system_address: Option<u64>,

    #[serde(rename = "StarClass")]
    pub star_class: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JetConeBoost {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "BoostValue")]
    pub boost_value: f64,
}

impl Into<GameActivity> for StartJump {
    fn into(self) -> GameActivity {

        match self.jump_type.as_str() {
            "Supercruise" => GameActivity {
                time: self.timestamp,
                time_display: prettify_date(&self.timestamp),
                verb: "".into(),
                noun: "Entered supercruise".into()
            },
            "Hyperspace" => GameActivity {
                time: self.timestamp,
                time_display: prettify_date(&self.timestamp),
                verb: "Jumped to".into(),
                noun: format!["{} ({})", self.star_system.unwrap(), self.star_class.unwrap()]
            },
            _ => panic!("Unknown jump type")
        }
    }
}