use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct NpcCrewRank {

    pub timestamp: String,

    #[serde(rename = "NpcCrewName")]
    pub npc_crew_name: String,

    #[serde(rename = "NpcCrewId")]
    pub npc_crew_id: u64,

    #[serde(rename = "RankCombat")]
    pub rank_combat: u32,
}