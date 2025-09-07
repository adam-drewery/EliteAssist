use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct Bodies {
    pub id: Option<i64>,
    pub id64: Option<i64>,
    pub name: Option<Box<str>>,
    pub url: Option<Box<str>>,
    #[serde(rename = "bodyCount")]
    pub body_count: Option<Option<i32>>,
    pub bodies: Option<Vec<Body>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Body {
    pub id: i64,
    pub id64: i64,
    #[serde(rename = "bodyId")]
    pub body_id: i32,
    pub name: Box<str>,
    pub discovery: Option<Discovery>,
    #[serde(rename = "type")]
    pub body_type: Box<str>,
    #[serde(rename = "subType")]
    pub sub_type: Box<str>,
    pub parents: Option<Vec<HashMap<Box<str>, i32>>>,
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: f32,
    #[serde(rename = "isMainStar")]
    pub is_main_star: Option<bool>,
    #[serde(rename = "isScoopable")]
    pub is_scoopable: Option<bool>,
    pub age: Option<i32>,
    #[serde(rename = "spectralClass")]
    pub spectral_class: Option<Box<str>>,
    pub luminosity: Option<Box<str>>,
    #[serde(rename = "absoluteMagnitude")]
    pub absolute_magnitude: Option<f32>,
    #[serde(rename = "solarMasses")]
    pub solar_masses: Option<f32>,
    #[serde(rename = "solarRadius")]
    pub solar_radius: Option<f32>,
    #[serde(rename = "surfaceTemperature")]
    pub surface_temperature: Option<i32>,
    #[serde(rename = "orbitalPeriod")]
    pub orbital_period: Option<f32>,
    #[serde(rename = "semiMajorAxis")]
    pub semi_major_axis: Option<f32>,
    #[serde(rename = "orbitalEccentricity")]
    pub orbital_eccentricity: Option<f32>,
    #[serde(rename = "orbitalInclination")]
    pub orbital_inclination: Option<f32>,
    #[serde(rename = "argOfPeriapsis")]
    pub arg_of_periapsis: Option<f32>,
    #[serde(rename = "rotationalPeriod")]
    pub rotational_period: Option<f32>,
    #[serde(rename = "rotationalPeriodTidallyLocked")]
    pub rotational_period_tidally_locked: Option<bool>,
    #[serde(rename = "axialTilt")]
    pub axial_tilt: Option<f32>,
    pub belts: Option<Vec<Belt>>,
    pub rings: Option<Vec<Ring>>,
    #[serde(rename = "updateTime")]
    pub update_time: Box<str>,
    #[serde(rename = "isLandable")]
    pub is_landable: Option<bool>,
    pub gravity: Option<f32>,
    #[serde(rename = "earthMasses")]
    pub earth_masses: Option<f32>,
    pub radius: Option<f32>,
    #[serde(rename = "surfacePressure")]
    pub surface_pressure: Option<f32>,
    #[serde(rename = "volcanismType")]
    pub volcanism_type: Option<Box<str>>,
    #[serde(rename = "atmosphereType")]
    pub atmosphere_type: Option<Box<str>>,
    #[serde(rename = "atmosphereComposition")]
    pub atmosphere_composition: Option<HashMap<Box<str>, f32>>,
    #[serde(rename = "solidComposition")]
    pub solid_composition: Option<HashMap<Box<str>, f32>>,
    #[serde(rename = "terraformingState")]
    pub terraforming_state: Option<Box<str>>,
    pub materials: Option<HashMap<Box<str>, f32>>,
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
    pub inner_radius: f32,
    #[serde(rename = "outerRadius")]
    pub outer_radius: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Ring {
    pub name: Box<str>,
    #[serde(rename = "type")]
    pub ring_type: Box<str>,
    pub mass: i64,
    #[serde(rename = "innerRadius")]
    pub inner_radius: f32,
    #[serde(rename = "outerRadius")]
    pub outer_radius: f32,
}