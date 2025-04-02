use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewAssign {

    pub timestamp: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "CrewID")]
    pub crew_id: u64,

    #[serde(rename = "Role")]
    pub role: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberJoins {

    pub timestamp: String,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberQuits {

    pub timestamp: String,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CrewMemberRoleChange {

    pub timestamp: String,

    #[serde(rename = "Crew")]
    pub crew: String,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct EndCrewSession {

    pub timestamp: String,

    #[serde(rename = "OnCrime")]
    pub on_crime: bool,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

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

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ChangeCrewRole {

    pub timestamp: String,

    #[serde(rename = "Role")]
    pub role: String,

    #[serde(rename = "Telepresence")]
    pub telepresence: bool,
}

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