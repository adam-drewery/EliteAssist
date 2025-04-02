use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FighterDestroyed {

    pub timestamp: String,

    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FighterRebuilt {

    pub timestamp: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct DockFighter {

    pub timestamp: String,

    #[serde(rename = "ID")]
    pub id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewLaunchFighter {

    pub timestamp: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,

    #[serde(rename = "Crew")]
    pub crew: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct LaunchFighter {

    pub timestamp: String,

    #[serde(rename = "Loadout")]
    pub loadout: String,

    #[serde(rename = "ID")]
    pub id: u64,

    #[serde(rename = "PlayerControlled")]
    pub player_controlled: bool,
}