use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TrendingState {
    pub state: String,
    pub trend: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FactionState {
    pub state: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Faction {
    pub id: u64,
    pub name: String,
    pub allegiance: String,
    pub government: String,
    pub influence: f32,
    pub state: String,
    #[serde(rename = "activeStates")]
    pub active_states: Vec<FactionState>,
    #[serde(rename = "recoveringStates")]
    pub recovering_states: Vec<TrendingState>,
    #[serde(rename = "pendingStates")]
    pub pending_states: Vec<TrendingState>,
    pub happiness: String,
    #[serde(rename = "isPlayer")]
    pub is_player: bool,
    #[serde(rename = "lastUpdate")]
    pub last_update: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ControllingFaction {
    pub id: Option<u64>,
    pub name: String,
    pub allegiance: String,
    pub government: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Factions {
    pub id: u64,
    pub id64: u64,
    pub name: String,
    pub url: String,
    #[serde(rename = "controllingFaction")]
    pub controlling_faction: ControllingFaction,
    pub factions: Vec<Faction>,
}