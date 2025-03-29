use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct Exobiology {

    #[serde(rename = "Organic_Genus_Encountered")]
    pub organic_genus_encountered: u64,

    #[serde(rename = "Organic_Species_Encountered")]
    pub organic_species_encountered: u64,

    #[serde(rename = "Organic_Variant_Encountered")]
    pub organic_variant_encountered: u64,

    #[serde(rename = "Organic_Data_Profits")]
    pub organic_data_profits: u64,

    #[serde(rename = "Organic_Data")]
    pub organic_data: u64,

    #[serde(rename = "First_Logged_Profits")]
    pub first_logged_profits: u64,

    #[serde(rename = "First_Logged")]
    pub first_logged: u64,

    #[serde(rename = "Organic_Systems")]
    pub organic_systems: u64,

    #[serde(rename = "Organic_Planets")]
    pub organic_planets: u64,

    #[serde(rename = "Organic_Genus")]
    pub organic_genus: u64,

    #[serde(rename = "Organic_Species")]
    pub organic_species: u64,
}