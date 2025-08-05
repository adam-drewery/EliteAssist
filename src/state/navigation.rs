#[derive(Default, Clone, Debug)]
pub struct NavRouteStep {

    pub star_system: String,
    pub system_address: u64,
    pub star_pos: Vec<f64>,
    pub star_class: String,
}

#[derive(Default, Clone, Debug)]
pub struct CurrentLocation {
    pub dist_from_star_ls: Option<f64>,
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
    pub system_faction: Option<SystemFaction>
}

#[derive(Default, Clone, Debug)]
pub struct SystemFaction {
    pub name: String,
    pub faction_state: Option<String>,
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
pub struct FactionState {
    pub state: String,
    pub trend: u64,
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