use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TgEncounters {

    #[serde(rename = "TG_ENCOUNTER_KILLED")]
    pub tg_encounter_killed: u64,

    #[serde(rename = "TG_ENCOUNTER_TOTAL")]
    pub tg_encounter_total: u64,

    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SYSTEM")]
    pub tg_encounter_total_last_system: String,

    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_TIMESTAMP")]
    pub tg_encounter_total_last_timestamp: String,

    #[serde(rename = "TG_ENCOUNTER_TOTAL_LAST_SHIP")]
    pub tg_encounter_total_last_ship: String,
}