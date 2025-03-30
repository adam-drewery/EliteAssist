use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct StationFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: String,
}