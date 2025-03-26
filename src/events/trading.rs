use serde::Deserialize;

#[derive(Deserialize)]
pub struct Trading {

    #[serde(rename = "Markets_Traded_With")]
    pub markets_traded_with: u64,

    #[serde(rename = "Market_Profits")]
    pub market_profits: u64,

    #[serde(rename = "Resources_Traded")]
    pub resources_traded: u64,

    #[serde(rename = "Average_Profit")]
    pub average_profit: f64,

    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: u64,

    #[serde(rename = "Data_Sold")]
    pub data_sold: u64,

    #[serde(rename = "Goods_Sold")]
    pub goods_sold: u64,

    #[serde(rename = "Assets_Sold")]
    pub assets_sold: u64,
}