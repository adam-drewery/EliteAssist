use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ScanOrganic {

    pub timestamp: String,

    #[serde(rename = "ScanType")]
    pub scan_type: String,

    #[serde(rename = "Genus")]
    pub genus: String,

    #[serde(rename = "Genus_Localised")]
    pub genus_localised: String,

    #[serde(rename = "Species")]
    pub species: String,

    #[serde(rename = "Species_Localised")]
    pub species_localised: String,

    #[serde(rename = "Variant")]
    pub variant: String,

    #[serde(rename = "Variant_Localised")]
    pub variant_localised: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "Body")]
    pub body: i64,
}