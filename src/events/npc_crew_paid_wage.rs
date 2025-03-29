use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NpcCrewPaidWage {
    
    pub timestamp: String,

    #[serde(rename = "NpcCrewName")]
    pub npc_crew_name: String,

    #[serde(rename = "NpcCrewId")]
    pub npc_crew_id: i64,

    #[serde(rename = "Amount")]
    pub amount: i64,
}