use serde::Deserialize;

#[derive(Deserialize)]
pub struct MaterialTraderStats {

    #[serde(rename = "Trades_Completed")]
    pub trades_completed: u64,

    #[serde(rename = "Materials_Traded")]
    pub materials_traded: u64,

    #[serde(rename = "Encoded_Materials_Traded")]
    pub encoded_materials_traded: u64,

    #[serde(rename = "Raw_Materials_Traded")]
    pub raw_materials_traded: u64,

    #[serde(rename = "Grade_1_Materials_Traded")]
    pub grade_1_materials_traded: u64,

    #[serde(rename = "Grade_2_Materials_Traded")]
    pub grade_2_materials_traded: u64,

    #[serde(rename = "Grade_3_Materials_Traded")]
    pub grade_3_materials_traded: u64,

    #[serde(rename = "Grade_4_Materials_Traded")]
    pub grade_4_materials_traded: u64,

    #[serde(rename = "Grade_5_Materials_Traded")]
    pub grade_5_materials_traded: u64,

    #[serde(rename = "Assets_Traded_In")]
    pub assets_traded_in: u64,

    #[serde(rename = "Assets_Traded_Out")]
    pub assets_traded_out: u64,
}