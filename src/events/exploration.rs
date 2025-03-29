use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Exploration {

    #[serde(rename = "Systems_Visited")]
    pub systems_visited: u64,

    #[serde(rename = "Exploration_Profits")]
    pub exploration_profits: u64,

    #[serde(rename = "Planets_Scanned_To_Level_2")]
    pub planets_scanned_to_level_2: u64,

    #[serde(rename = "Planets_Scanned_To_Level_3")]
    pub planets_scanned_to_level_3: u64,

    #[serde(rename = "Efficient_Scans")]
    pub efficient_scans: u64,

    #[serde(rename = "Highest_Payout")]
    pub highest_payout: u64,

    #[serde(rename = "Total_Hyperspace_Distance")]
    pub total_hyperspace_distance: u64,

    #[serde(rename = "Total_Hyperspace_Jumps")]
    pub total_hyperspace_jumps: u64,

    #[serde(rename = "Greatest_Distance_From_Start")]
    pub greatest_distance_from_start: f64,

    #[serde(rename = "Time_Played")]
    pub time_played: u64,

    #[serde(rename = "OnFoot_Distance_Travelled")]
    pub on_foot_distance_travelled: u64,

    #[serde(rename = "Shuttle_Journeys")]
    pub shuttle_journeys: u64,

    #[serde(rename = "Shuttle_Distance_Travelled")]
    pub shuttle_distance_travelled: f64,

    #[serde(rename = "Spent_On_Shuttles")]
    pub spent_on_shuttles: u64,

    #[serde(rename = "First_Footfalls")]
    pub first_footfalls: u64,

    #[serde(rename = "Planet_Footfalls")]
    pub planet_footfalls: u64,

    #[serde(rename = "Settlements_Visited")]
    pub settlements_visited: u64,
}