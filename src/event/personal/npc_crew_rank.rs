use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct NpcCrewRank {

    pub timestamp: String,

    #[serde(rename = "NpcCrewName")]
    pub npc_crew_name: String,

    #[serde(rename = "NpcCrewId")]
    pub npc_crew_id: i64,

    #[serde(rename = "RankCombat")]
    pub rank_combat: i64,
}