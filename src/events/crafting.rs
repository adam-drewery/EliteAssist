use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Crafting {

    #[serde(rename = "Count_Of_Used_Engineers")]
    pub count_of_used_engineers: u64,

    #[serde(rename = "Recipes_Generated")]
    pub recipes_generated: u64,

    #[serde(rename = "Recipes_Generated_Rank_1")]
    pub recipes_generated_rank_1: u64,

    #[serde(rename = "Recipes_Generated_Rank_2")]
    pub recipes_generated_rank_2: u64,

    #[serde(rename = "Recipes_Generated_Rank_3")]
    pub recipes_generated_rank_3: u64,

    #[serde(rename = "Recipes_Generated_Rank_4")]
    pub recipes_generated_rank_4: u64,

    #[serde(rename = "Recipes_Generated_Rank_5")]
    pub recipes_generated_rank_5: u64,

    #[serde(rename = "Suit_Mods_Applied")]
    pub suit_mods_applied: u64,

    #[serde(rename = "Weapon_Mods_Applied")]
    pub weapon_mods_applied: u64,

    #[serde(rename = "Suits_Upgraded")]
    pub suits_upgraded: u64,

    #[serde(rename = "Weapons_Upgraded")]
    pub weapons_upgraded: u64,

    #[serde(rename = "Suits_Upgraded_Full")]
    pub suits_upgraded_full: u64,

    #[serde(rename = "Weapons_Upgraded_Full")]
    pub weapons_upgraded_full: u64,

    #[serde(rename = "Suit_Mods_Applied_Full")]
    pub suit_mods_applied_full: u64,

    #[serde(rename = "Weapon_Mods_Applied_Full")]
    pub weapon_mods_applied_full: u64,
}