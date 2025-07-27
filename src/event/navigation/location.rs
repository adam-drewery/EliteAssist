use crate::event::navigation::faction::Faction;
use crate::event::navigation::station::StationEconomy;
use crate::state::{CurrentLocation, FactionState};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
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
    pub system_faction: Option<SystemFaction>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemFaction {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,
}

impl Into<CurrentLocation> for Location {
    fn into(self) -> CurrentLocation {
        CurrentLocation {
            dist_from_star_ls: self.dist_from_star_ls,
            docked: self.docked,
            station_name: self.station_name.clone(),
            station_type: self.station_type.clone(),
            station_faction: self
                .station_faction
                .clone()
                .map(|sf| crate::state::SystemFaction {
                    name: sf.name,
                    faction_state: sf.faction_state,
                }),
            station_government: self.station_government_localised.clone(),
            station_services: self.station_services.clone(),
            station_economy: self.station_economy_localised.clone(),
            station_economies: self.station_economies.clone().map(|economies| {
                economies
                    .into_iter()
                    .map(|economy| crate::state::StationEconomy {
                        name: economy.name_localised.unwrap_or_default(),
                        proportion: economy.proportion,
                    })
                    .collect()
            }),
            taxi: self.taxi,
            multicrew: self.multicrew,
            star_system: self.star_system.clone(),
            system_address: self.system_address,
            star_pos: self.star_pos.clone(),
            system_allegiance: self.system_allegiance.clone(),
            system_economy: self.system_economy_localised.clone(),
            system_second_economy: self.system_second_economy_localised.clone(),
            system_government: self.system_government_localised.clone(),
            system_security: self.system_security_localised.clone(),
            population: self.population,
            body: self.body.clone(),
            body_id: self.body_id,
            body_type: self.body_type.clone(),
            controlling_power: self.controlling_power.clone(),
            powers: self.powers.clone(),
            powerplay_state: self.powerplay_state.clone(),
            powerplay_state_control_progress: self.powerplay_state_control_progress,
            powerplay_state_reinforcement: self.powerplay_state_reinforcement,
            powerplay_state_undermining: self.powerplay_state_undermining,
            factions: self.factions.clone().map(|factions| {
                factions
                    .into_iter()
                    .map(|faction| crate::state::Faction {
                        name: faction.name,
                        faction_state: faction.faction_state,
                        government: faction.government,
                        influence: faction.influence,
                        allegiance: faction.allegiance,
                        happiness: faction.happiness_localised.unwrap_or_default(),
                        my_reputation: faction.my_reputation,
                        recovering_states: faction.recovering_states.map(|states| {
                            states
                                .into_iter()
                                .map(|state| FactionState {
                                    state: state.state,
                                    trend: state.trend,
                                })
                                .collect()
                        }),
                        active_states: faction.active_states.map(|states| {
                            states
                                .into_iter()
                                .map(|state| FactionState {
                                    state: state.state,
                                    trend: state.trend,
                                })
                                .collect()
                        }),
                    })
                    .collect()
            }),
            system_faction: self
                .system_faction
                .clone()
                .map(|sf| crate::state::SystemFaction {
                    name: sf.name,
                    faction_state: sf.faction_state,
                }),
        }
    }
}
