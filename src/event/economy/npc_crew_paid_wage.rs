use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NpcCrewPaidWage {
    
    pub timestamp: String,

    #[serde(rename = "NpcCrewName")]
    pub npc_crew_name: String,

    #[serde(rename = "NpcCrewId")]
    pub npc_crew_id: u64,

    #[serde(rename = "Amount")]
    pub amount: u32,
}