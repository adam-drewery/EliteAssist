use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Passengers {

    #[serde(rename = "Passengers_Missions_Accepted")]
    pub passengers_missions_accepted: u64,

    #[serde(rename = "Passengers_Missions_Disgruntled")]
    pub passengers_missions_disgruntled: u64,

    #[serde(rename = "Passengers_Missions_Bulk")]
    pub passengers_missions_bulk: u64,

    #[serde(rename = "Passengers_Missions_VIP")]
    pub passengers_missions_vip: u64,

    #[serde(rename = "Passengers_Missions_Delivered")]
    pub passengers_missions_delivered: u64,

    #[serde(rename = "Passengers_Missions_Ejected")]
    pub passengers_missions_ejected: u64,
}