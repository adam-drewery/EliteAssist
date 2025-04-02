use serde::Deserialize;

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