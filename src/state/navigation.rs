use crate::ardent;
use ed_journals::galaxy::BodyType;
use ed_journals::logs::fsd_jump_event::FSDJumpEvent;
use ed_journals::logs::location_event::LocationEvent;
use ed_journals::nav_route::NavRoute;

#[derive(Default, Clone, Debug)]
pub struct NavRouteStep {

    pub star_system: String,
    pub star_pos: [f32; 3],
    pub star_class: String,
}

impl NavRouteStep {
    pub fn vec_from(route: NavRoute) -> Vec<NavRouteStep> {
        route.route.into_iter().map(|step| NavRouteStep {
            star_system: step.star_system,
            star_pos: step.star_pos,
            star_class: step.star_class.to_string(),
        }).collect()
    }
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
    pub taxi: bool,
    pub multicrew: bool,
    pub star_system: String,
    pub system_address: u64,
    pub star_pos: [f32; 3],
    pub system_allegiance: Option<String>,
    pub system_economy: String,
    pub system_second_economy: String,
    pub system_government: String,
    pub system_security: String,
    pub system_faction: Option<SystemFaction>,
    pub population: u64,
    pub body: String,
    pub body_id: u8,
    pub body_type: String,
    pub controlling_power: Option<String>,
    pub powers: Option<Vec<String>>,
    pub powerplay_state: Option<String>,
    pub powerplay_state_conflict_progress: Option<f32>,
    pub powerplay_state_control_progress: Option<f32>,
    pub powerplay_state_reinforcement: Option<u32>,
    pub powerplay_state_undermining: Option<u32>,
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
    pub influence: f32,
    pub allegiance: String,
    pub happiness: String,
    pub my_reputation: f32,
    pub recovering_states: Vec<FactionState>,
    pub active_states: Vec<FactionState>
}

#[derive(Default, Clone, Debug)]
pub struct SystemFaction {
    pub name: String,
    pub faction_state: String,
}

#[derive(Default, Clone, Debug)]
pub struct FactionState {
    pub state: String,
    pub trend: u32,
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
    
    pub fn distance_to(&self, other: &NavRouteStep) -> f32 {
            let dx = self.star_pos[0] - other.star_pos[0];
            let dy = self.star_pos[1] - other.star_pos[1];
            let dz = self.star_pos[2] - other.star_pos[2];
            f32::sqrt(dx * dx + dy * dy + dz * dz)
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

impl From<FSDJumpEvent> for CurrentLocation {
    fn from(value: FSDJumpEvent) -> Self {
        CurrentLocation {
            docked: false,
            station_name: None,
            station_type: None,
            station_faction: None,
            station_government: None,
            station_services: None,
            station_economy: None,
            station_economies: vec![],
            star_system: value.system_info.star_system,
            system_address: value.system_info.system_address,
            star_pos: value.system_info.star_pos,
            system_allegiance: value.system_info.system_alliance.map(|x| {x.to_string()}),
            system_economy: value.system_info.system_economy_localized.unwrap_or(value.system_info.system_economy.to_string()),
            system_second_economy: value.system_info.system_second_economy_localized.unwrap_or(value.system_info.system_second_economy.to_string()),
            system_government: value.system_info.system_government_localized.unwrap_or(value.system_info.system_government.to_string()),
            system_security: value.system_info.system_security_localized.unwrap_or(value.system_info.system_security.to_string()),
            population: value.system_info.population,
            body: value.system_info.body,
            body_id: value.system_info.body_id,
            body_type: match value.system_info.body_type {
                BodyType::AsteroidCluster => "Asteroid Cluster".to_string(),
                BodyType::PlanetaryRing => "Planetary Ring".to_string(),
                BodyType::Station => "Station".to_string(),
                BodyType::Star => "Star".to_string(),
                BodyType::StellarRing => "Stellar Ring".to_string(),
                BodyType::Planet => "Planet".to_string(),
                BodyType::Null => "Unknown".to_string()
            },
            controlling_power: value.system_info.controlling_power,
            powers: value.system_info.powers,
            powerplay_state: value.system_info.powerplay_state,
            powerplay_state_conflict_progress: None,
            powerplay_state_control_progress: value.system_info.powerplay_state_control_progress,
            powerplay_state_reinforcement: value.system_info.powerplay_state_reinforcement,
            powerplay_state_undermining: value.system_info.powerplay_state_undermining,
            factions: value.system_info.factions.into_iter().map(|faction| Faction {
                name: faction.name,
                faction_state: faction.faction_state.to_string(),
                government: faction.government.to_string(),
                influence: faction.influence,
                allegiance: faction.allegiance.to_string(),
                happiness: faction.happiness_localized.unwrap_or_default(),
                my_reputation: faction.my_reputation,
                recovering_states: faction.recovering_states.into_iter().map(|state| FactionState { state: state.state.to_string(), trend: state.trend }).collect(),
                active_states: faction.active_states.into_iter().map(|state| FactionState { state: state.state.to_string(), trend: 0 }).collect(),
            }).collect(),
            system_faction: value.system_info.system_faction.map(|sf| SystemFaction { name: sf.name, faction_state: sf.faction_state.to_string() }),
            ..Default::default()
        }
    }
}

impl From<LocationEvent> for CurrentLocation {
    fn from(value: LocationEvent) -> Self {
        CurrentLocation {
            docked: value.docked,
            station_name: value.location_info.station_name,
            station_type: value.location_info.station_type,
            station_faction: value.location_info.station_faction.map(|faction| SystemFaction { name: faction.name, faction_state: faction.faction_state.to_string() }),
            station_government: value.location_info.station_government_localized,
            station_services: value.location_info.station_services,
            station_economy: value.location_info.station_economy_localized,
            station_economies: value.location_info.station_economies.unwrap_or_default().into_iter().map(|economy| StationEconomy { name: economy.name_localized.unwrap_or_default(), proportion: economy.proportion }).collect(),
            taxi: value.taxi,
            multicrew: value.multicrew,
            star_system: value.location_info.star_system,
            system_address: value.location_info.system_address,
            star_pos: value.location_info.star_pos,
            system_allegiance: value.location_info.system_alliance.map(|s|{ s.to_string() }),
            system_economy: value.location_info.system_economy_localized.unwrap_or(value.location_info.system_economy.to_string()),
            system_second_economy: value.location_info.system_second_economy_localized.unwrap_or(value.location_info.system_second_economy.to_string()),
            system_government: value.location_info.system_government_localized.unwrap_or(value.location_info.system_government.to_string()),
            system_security: value.location_info.system_security_localized.unwrap_or(value.location_info.system_security.to_string()),
            population: value.location_info.population,
            body: value.location_info.body,
            body_id: value.location_info.body_id,
            body_type: match value.location_info.body_type {
                BodyType::AsteroidCluster => "Asteroid Cluster".to_string(),
                BodyType::PlanetaryRing => "Planetary Ring".to_string(),
                BodyType::Station => "Station".to_string(),
                BodyType::Star => "Star".to_string(),
                BodyType::StellarRing => "Stellar Ring".to_string(),
                BodyType::Planet => "Planet".to_string(),
                BodyType::Null => "Unknown".to_string()
            },
            controlling_power: value.location_info.controlling_power,
            powers: value.location_info.powers,
            powerplay_state: value.location_info.powerplay_state,
            powerplay_state_conflict_progress: None,
            powerplay_state_control_progress: value.location_info.powerplay_state_control_progress,
            powerplay_state_reinforcement: value.location_info.powerplay_state_reinforcement,
            powerplay_state_undermining: value.location_info.powerplay_state_undermining,
            factions: value.location_info.factions.into_iter().map(|faction| Faction {
                name: faction.name,
                faction_state: faction.faction_state.to_string(),
                government: faction.government.to_string(),
                influence: faction.influence,
                allegiance: faction.allegiance.to_string(),
                happiness: faction.happiness_localized.unwrap_or_default(),
                my_reputation: faction.my_reputation,
                recovering_states: faction.recovering_states.into_iter().map(|state| FactionState { state: state.state.to_string(), trend: state.trend }).collect(),
                active_states: faction.active_states.into_iter().map(|state| FactionState { state: state.state.to_string(), trend: 0 }).collect(),
            }).collect(),
            system_faction: value.location_info.system_faction.map(|sf| SystemFaction { name: sf.name, faction_state: sf.faction_state.to_string() }),
            ..Default::default()
        }
    }
}