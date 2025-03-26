use serde::Deserialize;

#[derive(Deserialize)]
pub struct Smuggling {

    #[serde(rename = "Black_Markets_Traded_With")]
    pub black_markets_traded_with: u64,

    #[serde(rename = "Black_Markets_Profits")]
    pub black_markets_profits: u64,

    #[serde(rename = "Resources_Smuggled")]
    pub resources_smuggled: u64,

    #[serde(rename = "Average_Profit")]
    pub average_profit: f64,

    #[serde(rename = "Highest_Single_Transaction")]
    pub highest_single_transaction: u64,
}