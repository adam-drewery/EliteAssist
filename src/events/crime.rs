use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Crime {

    #[serde(rename = "Notoriety")]
    pub notoriety: u64,

    #[serde(rename = "Fines")]
    pub fines: u64,

    #[serde(rename = "Total_Fines")]
    pub total_fines: u64,

    #[serde(rename = "Bounties_Received")]
    pub bounties_received: u64,

    #[serde(rename = "Total_Bounties")]
    pub total_bounties: u64,

    #[serde(rename = "Highest_Bounty")]
    pub highest_bounty: u64,

    #[serde(rename = "Malware_Uploaded")]
    pub malware_uploaded: u64,

    #[serde(rename = "Settlements_State_Shutdown")]
    pub settlements_state_shutdown: u64,

    #[serde(rename = "Production_Sabotage")]
    pub production_sabotage: u64,

    #[serde(rename = "Production_Theft")]
    pub production_theft: u64,

    #[serde(rename = "Total_Murders")]
    pub total_murders: u64,

    #[serde(rename = "Citizens_Murdered")]
    pub citizens_murdered: u64,

    #[serde(rename = "Omnipol_Murdered")]
    pub omnipol_murdered: u64,

    #[serde(rename = "Guards_Murdered")]
    pub guards_murdered: u64,

    #[serde(rename = "Data_Stolen")]
    pub data_stolen: u64,

    #[serde(rename = "Goods_Stolen")]
    pub goods_stolen: u64,

    #[serde(rename = "Sample_Stolen")]
    pub sample_stolen: u64,

    #[serde(rename = "Total_Stolen")]
    pub total_stolen: u64,

    #[serde(rename = "Turrets_Destroyed")]
    pub turrets_destroyed: u64,

    #[serde(rename = "Turrets_Overloaded")]
    pub turrets_overloaded: u64,

    #[serde(rename = "Turrets_Total")]
    pub turrets_total: u64,

    #[serde(rename = "Value_Stolen_StateChange")]
    pub value_stolen_state_change: u64,

    #[serde(rename = "Profiles_Cloned")]
    pub profiles_cloned: u64,
}