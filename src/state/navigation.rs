use crate::ardent;
use crate::journal::event;

#[derive(Default, Clone, Debug)]
pub struct NavRouteStep {

    pub star_system: String,
    pub star_pos: Vec<f64>,
    pub star_class: String,
}

#[derive(Default, Clone, Debug)]
pub struct CurrentLocation {
    pub docked: bool,
    pub station_name: Option<String>,
    pub station_type: Option<String>,
    pub station_faction: Option<SystemFaction>,
    pub station_government: Option<String>,
    pub station_services: Option<Vec<String>>,
    pub station_economy: Option<String>,
    pub station_economies: Vec<StationEconomy>,
    pub taxi: Option<bool>,
    pub multicrew: Option<bool>,
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: Vec<f64>,
    pub system_allegiance: String,
    pub system_economy: String,
    pub system_second_economy: String,
    pub system_government: String,
    pub system_security: String,
    pub system_faction: Option<SystemFaction>,
    pub population: u64,
    pub body: String,
    pub body_id: u64,
    pub body_type: String,
    pub controlling_power: Option<String>,
    pub powers: Option<Vec<String>>,
    pub powerplay_state: Option<String>,
    pub powerplay_state_conflict_progress: Option<f64>,
    pub powerplay_state_control_progress: Option<f64>,
    pub powerplay_state_reinforcement: Option<u64>,
    pub powerplay_state_undermining: Option<u64>,
    pub factions: Vec<Faction>,
    pub stations: Vec<Station>,
    pub nearby_systems: Vec<System>,
    pub known_bodies: Vec<Body>,
    pub traffic: Option<Counts>,
    pub deaths: Option<Counts>,
}

#[derive(Default, Clone, Debug)]
pub struct StationEconomy {
    pub name: String,
    pub proportion: f64,
}

#[derive(Default, Clone, Debug)]
pub struct Faction {
    pub name: String,
    pub faction_state: String,
    pub government: String,
    pub influence: f64,
    pub allegiance: String,
    pub happiness: String,
    pub my_reputation: f64,
    pub recovering_states: Vec<FactionState>,
    pub active_states: Vec<FactionState>
}

#[derive(Default, Clone, Debug)]
pub struct SystemFaction {
    pub name: String,
    pub faction_state: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct FactionState {
    pub state: String,
    pub trend: u64,
}

#[derive(Default, Clone, Debug)]
pub struct Station {
    pub id: i64,
    pub market_id: i64,
    pub type_field: String,
    pub name: String,
    pub body: Option<StationBody>,
    pub distance_to_arrival: f32,
    pub allegiance: String,
    pub government: String,
    pub economy: String,
    pub second_economy: Option<String>,
    pub have_market: bool,
    pub have_shipyard: bool,
    pub have_outfitting: bool,
    pub other_services: Vec<String>,
    pub controlling_faction: String,
    pub update_time: LastUpdated,
}

#[derive(Default, Clone, Debug)]
pub struct StationBody {
    pub id: i64,
    pub name: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Default, Clone, Debug)]
pub struct LastUpdated {
    pub information: String,
    pub market: Option<String>,
    pub shipyard: Option<String>,
    pub outfitting: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct Body {
    pub name: String,
    pub type_field: String,
    pub sub_type: String,
    pub distance_to_arrival: f64,
    pub is_main_star: bool,
    pub is_scoopable: bool,
}

#[derive(Default, Clone, Debug)]
pub struct System {
    pub address: u64,
    pub name: String
}

#[derive(Default, Clone, Debug)]
pub struct Counts {
    pub day: u64,
    pub week: u64,
    pub total: u64,
}

impl NavRouteStep {
    
    pub fn is_fuel_star(&self) -> bool {
        matches!(self.star_class.chars().next(), Some('O' | 'B' | 'A' | 'F' | 'G' | 'K' | 'M'))
    }
    
    pub fn distance_to(&self, other: &NavRouteStep) -> f64 {
            let dx = self.star_pos[0] - other.star_pos[0];
            let dy = self.star_pos[1] - other.star_pos[1];
            let dz = self.star_pos[2] - other.star_pos[2];
            f64::sqrt(dx * dx + dy * dy + dz * dz)
    }
}

impl From<ardent::NearbySystem> for System {
    fn from(value: ardent::NearbySystem) -> Self {
        System {
            name: value.name,
            address: value.address
        }
    }
}

impl From<event::FSDJump> for CurrentLocation {
    fn from(value: event::FSDJump) -> Self {
        CurrentLocation {
            docked: false,
            station_name: None,
            station_type: None,
            station_faction: None,
            station_government: None,
            station_services: None,
            station_economy: None,
            station_economies: vec![],
            taxi: value.taxi,
            multicrew: value.multicrew,
            star_system: value.star_system,
            system_address: value.system_address,
            star_pos: value.star_pos,
            system_allegiance: value.system_allegiance,
            system_economy: value.system_economy_localised.unwrap_or(value.system_economy),
            system_second_economy: value.system_second_economy_localised.unwrap_or(value.system_second_economy),
            system_government: value.system_government_localised.unwrap_or(value.system_government),
            system_security: value.system_security_localised.unwrap_or(value.system_security),
            population: value.population,
            body: value.body,
            body_id: value.body_id,
            body_type: value.body_type,
            powers: value.powers.clone(),
            controlling_power: value.powers.and_then(|p| p.first().cloned()),
            powerplay_state: value.powerplay_state,
            powerplay_state_conflict_progress: None,
            powerplay_state_control_progress: value.powerplay_state_control_progress,
            powerplay_state_reinforcement: value.powerplay_state_reinforcement,
            powerplay_state_undermining: value.powerplay_state_undermining,
            factions: value.factions.unwrap_or_default().into_iter().map(|f| Faction {
                name: f.name,
                faction_state: f.faction_state,
                government: f.government,
                influence: f.influence,
                allegiance: f.allegiance,
                happiness: f.happiness,
                my_reputation: f.my_reputation,
                recovering_states: f.recovering_states.unwrap_or_default().into_iter().map(|s| FactionState { state: s.state, trend: s.trend }).collect(),
                active_states: f.active_states.unwrap_or_default().into_iter().map(|s| FactionState { state: s.state, trend: 0 }).collect(),
            }).collect(),
            system_faction: value.system_faction.map(|f| SystemFaction { name: f.name, faction_state: f.faction_state }),
            ..Default::default()
        }
    }
}

impl From<event::Location> for CurrentLocation {
    fn from(value: event::Location) -> Self {
        CurrentLocation {
            docked: value.docked,
            station_name: value.station_name,
            station_type: value.station_type,
            station_faction: value.station_faction.map(|faction| SystemFaction { name: faction.name, faction_state: faction.faction_state }),
            station_government: value.station_government_localised,
            station_services: value.station_services,
            station_economy: value.station_economy_localised,
            station_economies: value.station_economies.unwrap_or_default().into_iter().map(|economy| StationEconomy { name: economy.name_localised.unwrap_or_default(), proportion: economy.proportion }).collect(),
            taxi: value.taxi,
            multicrew: value.multicrew,
            star_system: value.star_system,
            system_address: value.system_address,
            star_pos: value.star_pos,
            system_allegiance: value.system_allegiance,
            system_economy: value.system_economy_localised.unwrap_or(value.system_economy),
            system_second_economy: value.system_second_economy_localised.unwrap_or(value.system_second_economy),
            system_government: value.system_government_localised.unwrap_or(value.system_government),
            system_security: value.system_security_localised.unwrap_or(value.system_security),
            population: value.population,
            body: value.body,
            body_id: value.body_id,
            body_type: value.body_type,
            controlling_power: value.controlling_power,
            powers: value.powers,
            powerplay_state: value.powerplay_state,
            powerplay_state_conflict_progress: None,
            powerplay_state_control_progress: value.powerplay_state_control_progress,
            powerplay_state_reinforcement: value.powerplay_state_reinforcement,
            powerplay_state_undermining: value.powerplay_state_undermining,
            factions: value.factions.unwrap_or_default().into_iter().map(|faction| Faction {
                name: faction.name,
                faction_state: faction.faction_state,
                government: faction.government,
                influence: faction.influence,
                allegiance: faction.allegiance,
                happiness: faction.happiness_localised.unwrap_or_default(),
                my_reputation: faction.my_reputation,
                recovering_states: faction.recovering_states.unwrap_or_default().into_iter().map(|state| FactionState { state: state.state, trend: state.trend }).collect(),
                active_states: faction.active_states.unwrap_or_default().into_iter().map(|state| FactionState { state: state.state, trend: 0 }).collect(),
            }).collect(),
            system_faction: value.system_faction.map(|sf| SystemFaction { name: sf.name, faction_state: sf.faction_state }),
            ..Default::default()
        }
    }
}

impl From<event::NavRoute> for Vec<NavRouteStep> {
    fn from(value: event::NavRoute) -> Self {
        match value.route {
            Some(route) => route
                .into_iter()
                .map(|step| NavRouteStep {
                    star_system: step.star_system,
                    star_pos: step.star_pos,
                    star_class: step.star_class,
                })
                .collect(),
            None => Vec::new(),
        }
    }
}
