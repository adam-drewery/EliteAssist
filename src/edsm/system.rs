use serde::Deserialize;
use crate::edsm::Coords;

#[derive(Clone, Debug, Deserialize)]
pub struct System {
    pub id: i64,
    pub id64: i64,
    pub name: String,
    pub coords: Coords,
    #[serde(rename = "coordsLocked")]
    pub coords_locked: bool,
    pub information: SystemInformation,
    #[serde(rename = "primaryStar")]
    pub primary_star: PrimaryStar,
    #[serde(rename = "requirePermit")]
    pub require_permit: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemInformation {
    pub allegiance: Option<String>,
    pub government: Option<String>,
    pub economy: Option<String>,
    #[serde(rename = "secondEconomy")]
    pub second_economy: Option<String>,
    pub population: Option<u64>,
    pub security: Option<String>,
    pub faction: Option<String>,
    #[serde(rename = "factionState")]
    pub faction_state: Option<String>
}

#[derive(Clone, Debug, Deserialize)]
pub struct PrimaryStar {
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    #[serde(rename = "isScoopable")]
    pub is_scoopable: bool,
}