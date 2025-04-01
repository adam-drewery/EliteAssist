use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Faction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: Option<String>,

    #[serde(rename = "Government")]
    pub government: String,

    #[serde(rename = "Influence")]
    pub influence: f64,

    #[serde(rename = "Allegiance")]
    pub allegiance: String,

    #[serde(rename = "Happiness")]
    pub happiness: String,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: Option<String>,

    #[serde(rename = "MyReputation")]
    pub my_reputation: f64,

    #[serde(rename = "RecoveringStates")]
    pub recovering_states: Option<Vec<FactionState>>,

    #[serde(rename = "ActiveStates")]
    pub active_states: Option<Vec<FactionState>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct FactionState {

    #[serde(rename = "State")]
    pub state: String,

    #[serde(rename = "Trend")]
    pub trend: Option<u8>,
}