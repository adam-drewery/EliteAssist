use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Reward {

    #[serde(rename = "Faction")]
    pub faction: String,

    #[serde(rename = "Reward")]
    pub reward: i64,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Bounty {

    pub timestamp: String,

    #[serde(rename = "Rewards")]
    pub rewards: Vec<Reward>,

    #[serde(rename = "PilotName")]
    pub pilot_name: String,

    #[serde(rename = "PilotName_Localised")]
    pub pilot_name_localised: String,

    #[serde(rename = "Target")]
    pub target: String,

    #[serde(rename = "Target_Localised")]
    pub target_localised: Option<String>,

    #[serde(rename = "TotalReward")]
    pub total_reward: i64,

    #[serde(rename = "VictimFaction")]
    pub victim_faction: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct PayBounties {

    pub timestamp: String,

    #[serde(rename = "Amount")]
    pub amount: i64,

    #[serde(rename = "AllFines")]
    pub all_fines: bool,

    #[serde(rename = "ShipID")]
    pub ship_id: i64,

    #[serde(rename = "BrokerPercentage")]
    pub broker_percentage: f64,
}