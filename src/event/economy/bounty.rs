use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Reward {

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Reward")]
    pub reward: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bounty {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Rewards")]
    pub rewards: Vec<Reward>,

    #[serde(rename = "PilotName")]
    pub pilot_name: Option<String>,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localised: Option<String>,

    #[serde(rename = "Target")]
    pub target: String,

    #[serde(rename = "Target_Localised")]
    pub target_localised: Option<String>,

    #[serde(rename = "TotalReward")]
    pub total_reward: u32,

    #[serde(rename = "VictimFaction")]
    pub victim_faction: String,
    
    #[serde(rename = "SharedWithOthers")]
    pub shared_with_others: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PayBounties {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Amount")]
    pub amount: u32,

    #[serde(rename = "AllFines")]
    pub all_fines: bool,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "BrokerPercentage")]
    pub broker_percentage: f64,
}