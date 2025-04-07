use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TopTier {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Bonus")]
    pub bonus: String,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
pub struct CommunityGoal {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CurrentGoals")]
    pub current_goals: Vec<CurrentGoal>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommunityGoalJoin {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CGID")]
    pub cgid: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "System")]
    pub system: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CommunityGoalReward {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CGID")]
    pub cgid: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "System")]
    pub system: String,

    #[serde(rename = "Reward")]
    pub reward: u32,
}