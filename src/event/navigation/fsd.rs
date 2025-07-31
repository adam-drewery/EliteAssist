use crate::event::navigation::faction::Faction;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::event::format::prettify_date;
use crate::event::personal::Power;
use crate::state::{CurrentLocation, GameActivity};

#[derive(Clone, Debug, Deserialize)]
pub struct FSDJump {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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
    pub system_economy_localised: Option<String>,

    #[serde(rename = "SystemSecondEconomy")]
    pub system_second_economy: String,

    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: Option<String>,

    #[serde(rename = "SystemGovernment")]
    pub system_government: String,

    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: Option<String>,

    #[serde(rename = "SystemSecurity")]
    pub system_security: String,

    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: Option<String>,

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

    #[serde(rename = "PowerplayStateControlProgress")]
    pub powerplay_state_control_progress: Option<f64>,

    #[serde(rename = "PowerplayStateReinforcement")]
    pub powerplay_state_reinforcement: Option<u32>,

    #[serde(rename = "PowerplayStateUndermining")]
    pub powerplay_state_undermining: Option<u32>,

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
pub struct SystemFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,
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
    pub taxi: Option<bool>,

    #[serde(rename = "Multicrew")]
    pub multicrew: Option<bool>,

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
    pub taxi: Option<bool>,

    #[serde(rename = "Multicrew")]
    pub multicrew: Option<bool>,

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
    pub taxi: Option<bool>,

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

impl Into<CurrentLocation> for FSDJump {
    fn into(self) -> CurrentLocation {
        CurrentLocation {
            dist_from_star_ls: None,
            docked: false,
            station_name: None,
            station_type: None,
            station_faction: None,
            station_government: None,
            station_services: None,
            station_economy: None,
            station_economies: None,
            taxi: self.taxi,
            multicrew: self.multicrew,
            star_system: self.star_system,
            system_address: self.system_address,
            star_pos: self.star_pos,
            system_allegiance: self.system_allegiance,
            system_economy: self.system_economy_localised.unwrap_or_default(),
            system_second_economy: self.system_second_economy_localised.unwrap_or_default(),
            system_government: self.system_government_localised.unwrap_or_default(),
            system_security: self.system_security_localised.unwrap_or_default(),
            population: self.population,
            body: self.body,
            body_id: self.body_id,
            body_type: self.body_type,
            powers: self.powers.clone(),
            controlling_power: self.powers.and_then(|p| p.first().cloned()),
            powerplay_state: self.powerplay_state,
            powerplay_state_conflict_progress: None, // todo
            powerplay_state_control_progress: self.powerplay_state_control_progress,
            powerplay_state_reinforcement: self.powerplay_state_reinforcement,
            powerplay_state_undermining: self.powerplay_state_undermining,
            factions: self.factions.map(|factions| {
                factions.into_iter().map(|f| crate::state::Faction {
                    name: f.name,
                    faction_state: f.faction_state,
                    government: f.government,
                    influence: f.influence,
                    allegiance: f.allegiance,
                    happiness: f.happiness,
                    my_reputation: f.my_reputation,
                    recovering_states: f.recovering_states.map(|states| {
                        states.into_iter().map(|s| crate::state::FactionState {
                            state: s.state,
                            trend: s.trend,
                        }).collect()
                    }),
                    active_states: f.active_states.map(|states| {
                        states.into_iter().map(|s| crate::state::FactionState {
                            state: s.state,
                            trend: s.trend,
                        }).collect()
                    }),
                }).collect()
            }),
            system_faction: self.system_faction.map(|f| crate::state::SystemFaction {
                name: f.name,
                faction_state: f.faction_state,
            }),
        }
    }
}