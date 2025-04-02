use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CommunityGoalReward {

    pub timestamp: String,

    #[serde(rename = "CGID")]
    pub cgid: i64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "System")]
    pub system: String,

    #[serde(rename = "Reward")]
    pub reward: i64,
}