use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Crew {

    #[serde(rename = "NpcCrew_TotalWages")]
    pub npc_crew_total_wages: u64,

    #[serde(rename = "NpcCrew_Hired")]
    pub npc_crew_hired: u64,

    #[serde(rename = "NpcCrew_Fired")]
    pub npc_crew_fired: u64,

    #[serde(rename = "NpcCrew_Died")]
    pub npc_crew_died: u64,
}