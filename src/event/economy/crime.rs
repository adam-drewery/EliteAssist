use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CommitCrime {

    pub timestamp: String,

    #[serde(rename = "CrimeType")]
    pub crime_type: String,

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Fine")]
    pub fine: Option<u32>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PayFines {

    pub timestamp: String,

    #[serde(rename = "Amount")]
    pub amount: u32,

    #[serde(rename = "AllFines")]
    pub all_fines: bool,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ClearImpound {

    pub timestamp: String,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipMarketID")]
    pub ship_market_id: u64,

    #[serde(rename = "MarketID")]
    pub market_id: u64,
}