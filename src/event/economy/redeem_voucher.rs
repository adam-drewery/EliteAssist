use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct FactionVoucher {

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Amount")]
    pub amount: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct RedeemVoucher {

    pub timestamp: String,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Amount")]
    pub amount: i64,

    #[serde(rename = "Factions")]
    pub factions: Option<Vec<FactionVoucher>>,
}