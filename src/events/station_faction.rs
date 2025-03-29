use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct StationFaction {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: String,
}