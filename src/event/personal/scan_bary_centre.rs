use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct ScanBaryCentre {

    pub timestamp: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: i64,

    #[serde(rename = "BodyID")]
    pub body_id: i64,

    #[serde(rename = "SemiMajorAxis")]
    pub semi_major_axis: f64,

    #[serde(rename = "Eccentricity")]
    pub eccentricity: f64,

    #[serde(rename = "OrbitalInclination")]
    pub orbital_inclination: f64,

    #[serde(rename = "Periapsis")]
    pub periapsis: f64,

    #[serde(rename = "OrbitalPeriod")]
    pub orbital_period: f64,

    #[serde(rename = "AscendingNode")]
    pub ascending_node: f64,

    #[serde(rename = "MeanAnomaly")]
    pub mean_anomaly: f64,
}