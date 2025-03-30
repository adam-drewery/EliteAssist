use crate::event::faction_state::FactionState;
use serde::Deserialize;

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