use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Multicrew {

    #[serde(rename = "Multicrew_Time_Total")]
    pub multicrew_time_total: u64,

    #[serde(rename = "Multicrew_Gunner_Time_Total")]
    pub multicrew_gunner_time_total: u64,

    #[serde(rename = "Multicrew_Fighter_Time_Total")]
    pub multicrew_fighter_time_total: u64,

    #[serde(rename = "Multicrew_Credits_Total")]
    pub multicrew_credits_total: u64,

    #[serde(rename = "Multicrew_Fines_Total")]
    pub multicrew_fines_total: u64,
}