use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct SearchAndRescue {

    #[serde(rename = "SearchRescue_Traded")]
    pub search_rescue_traded: u64,

    #[serde(rename = "SearchRescue_Profit")]
    pub search_rescue_profit: u64,

    #[serde(rename = "SearchRescue_Count")]
    pub search_rescue_count: u64,

    #[serde(rename = "Salvage_Legal_POI")]
    pub salvage_legal_poi: u64,

    #[serde(rename = "Salvage_Legal_Settlements")]
    pub salvage_legal_settlements: u64,

    #[serde(rename = "Salvage_Illegal_POI")]
    pub salvage_illegal_poi: u64,

    #[serde(rename = "Salvage_Illegal_Settlements")]
    pub salvage_illegal_settlements: u64,

    #[serde(rename = "Maglocks_Opened")]
    pub maglocks_opened: u64,

    #[serde(rename = "Panels_Opened")]
    pub panels_opened: u64,

    #[serde(rename = "Settlements_State_FireOut")]
    pub settlements_state_fire_out: u64,

    #[serde(rename = "Settlements_State_Reboot")]
    pub settlements_state_reboot: u64,
}