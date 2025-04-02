use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct TopTier {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Bonus")]
    pub bonus: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CurrentGoal {

    #[serde(rename = "CGID")]
    pub cgid: u64,

    #[serde(rename = "Title")]
    pub title: String,

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "MarketName")]
    pub market_name: String,

    #[serde(rename = "Expiry")]
    pub expiry: String,

    #[serde(rename = "IsComplete")]
    pub is_complete: bool,

    #[serde(rename = "CurrentTotal")]
    pub current_total: u64,

    #[serde(rename = "PlayerContribution")]
    pub player_contribution: u32,

    #[serde(rename = "NumContributors")]
    pub num_contributors: u32,

    #[serde(rename = "TopTier")]
    pub top_tier: TopTier,

    #[serde(rename = "TierReached")]
    pub tier_reached: String,

    #[serde(rename = "PlayerPercentileBand")]
    pub player_percentile_band: u32,

    #[serde(rename = "Bonus")]
    pub bonus: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CommunityGoal {

    pub timestamp: String,

    #[serde(rename = "CurrentGoals")]
    pub current_goals: Vec<CurrentGoal>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CommunityGoalJoin {

    pub timestamp: String,

    #[serde(rename = "CGID")]
    pub cgid: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "System")]
    pub system: String,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CommunityGoalReward {

    pub timestamp: String,

    #[serde(rename = "CGID")]
    pub cgid: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "System")]
    pub system: String,

    #[serde(rename = "Reward")]
    pub reward: u32,
}