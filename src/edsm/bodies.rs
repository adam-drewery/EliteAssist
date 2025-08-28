use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Bodies {
    pub id: i64,
    pub id64: i64,
    pub name: Box<str>,
    pub url: Box<str>,
    #[serde(rename = "bodyCount")]
    pub body_count: i32,
    pub bodies: Vec<Body>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Body {
    pub id: i64,
    pub id64: i64,
    #[serde(rename = "bodyId")]
    pub body_id: i32,
    pub name: Box<str>,
    pub discovery: Discovery,
    #[serde(rename = "type")]
    pub body_type: Box<str>,
    #[serde(rename = "subType")]
    pub sub_type: Box<str>,
    pub parents: Option<Vec<HashMap<Box<str>, i32>>>,
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: f64,
    #[serde(rename = "isMainStar")]
    pub is_main_star: Option<bool>,
    #[serde(rename = "isScoopable")]
    pub is_scoopable: Option<bool>,
    pub age: Option<i32>,
    #[serde(rename = "spectralClass")]
    pub spectral_class: Option<Box<str>>,
    pub luminosity: Option<Box<str>>,
    #[serde(rename = "absoluteMagnitude")]
    pub absolute_magnitude: Option<f64>,
    #[serde(rename = "solarMasses")]
    pub solar_masses: Option<f64>,
    #[serde(rename = "solarRadius")]
    pub solar_radius: Option<f64>,
    #[serde(rename = "surfaceTemperature")]
    pub surface_temperature: Option<i32>,
    #[serde(rename = "orbitalPeriod")]
    pub orbital_period: Option<f64>,
    #[serde(rename = "semiMajorAxis")]
    pub semi_major_axis: Option<f64>,
    #[serde(rename = "orbitalEccentricity")]
    pub orbital_eccentricity: Option<f64>,
    #[serde(rename = "orbitalInclination")]
    pub orbital_inclination: Option<f64>,
    #[serde(rename = "argOfPeriapsis")]
    pub arg_of_periapsis: Option<f64>,
    #[serde(rename = "rotationalPeriod")]
    pub rotational_period: Option<f64>,
    #[serde(rename = "rotationalPeriodTidallyLocked")]
    pub rotational_period_tidally_locked: Option<bool>,
    #[serde(rename = "axialTilt")]
    pub axial_tilt: Option<f64>,
    pub belts: Option<Vec<Belt>>,
    pub rings: Option<Vec<Ring>>,
    #[serde(rename = "updateTime")]
    pub update_time: Box<str>,
    #[serde(rename = "isLandable")]
    pub is_landable: Option<bool>,
    pub gravity: Option<f64>,
    #[serde(rename = "earthMasses")]
    pub earth_masses: Option<f64>,
    pub radius: Option<f64>,
    #[serde(rename = "surfacePressure")]
    pub surface_pressure: Option<f64>,
    #[serde(rename = "volcanismType")]
    pub volcanism_type: Option<Box<str>>,
    #[serde(rename = "atmosphereType")]
    pub atmosphere_type: Option<Box<str>>,
    #[serde(rename = "atmosphereComposition")]
    pub atmosphere_composition: Option<HashMap<Box<str>, f64>>,
    #[serde(rename = "solidComposition")]
    pub solid_composition: Option<HashMap<Box<str>, f64>>,
    #[serde(rename = "terraformingState")]
    pub terraforming_state: Option<Box<str>>,
    pub materials: Option<HashMap<Box<str>, f64>>,
    #[serde(rename = "reserveLevel")]
    pub reserve_level: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Discovery {
    pub commander: Box<str>,
    pub date: Box<str>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Belt {
    pub name: Box<str>,
    #[serde(rename = "type")]
    pub belt_type: Box<str>,
    pub mass: i64,
    #[serde(rename = "innerRadius")]
    pub inner_radius: i64,
    #[serde(rename = "outerRadius")]
    pub outer_radius: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Ring {
    pub name: Box<str>,
    #[serde(rename = "type")]
    pub ring_type: Box<str>,
    pub mass: i64,
    #[serde(rename = "innerRadius")]
    pub inner_radius: i64,
    #[serde(rename = "outerRadius")]
    pub outer_radius: i64,
}