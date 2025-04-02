use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Rings {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "RingClass")]
    pub ring_class: String,

    #[serde(rename = "MassMT")]
    pub mass_mt: f64,

    #[serde(rename = "InnerRad")]
    pub inner_rad: f64,

    #[serde(rename = "OuterRad")]
    pub outer_rad: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Composition {

    #[serde(rename = "Ice")]
    pub ice: f64,

    #[serde(rename = "Rock")]
    pub rock: f64,

    #[serde(rename = "Metal")]
    pub metal: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScanMaterial {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Percent")]
    pub percent: f64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Parent {

    #[serde(rename = "Planet")]
    pub planet: Option<u32>,

    #[serde(rename = "Star")]
    pub star: Option<u32>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Scan {

    pub timestamp: String,

    #[serde(rename = "ScanType")]
    pub scan_type: String,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "Parents")]
    pub parents: Option<Vec<Parent>>,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "DistanceFromArrivalLS")]
    pub distance_from_arrival_ls: f64,

    #[serde(rename = "TidalLock")]
    pub tidal_lock: Option<bool>,

    #[serde(rename = "TerraformState")]
    pub terraform_state: Option<String>,

    #[serde(rename = "PlanetClass")]
    pub planet_class: Option<String>,

    #[serde(rename = "Atmosphere")]
    pub atmosphere: Option<String>,

    #[serde(rename = "AtmosphereType")]
    pub atmosphere_type: Option<String>,

    #[serde(rename = "Volcanism")]
    pub volcanism: Option<String>,

    #[serde(rename = "MassEM")]
    pub mass_em: Option<f64>,

    #[serde(rename = "Radius")]
    pub radius: Option<f64>,

    #[serde(rename = "SurfaceGravity")]
    pub surface_gravity: Option<f64>,

    #[serde(rename = "SurfaceTemperature")]
    pub surface_temperature: Option<f64>,

    #[serde(rename = "SurfacePressure")]
    pub surface_pressure: Option<f64>,

    #[serde(rename = "Landable")]
    pub landable: Option<bool>,

    #[serde(rename = "Materials")]
    pub materials: Option<Vec<ScanMaterial>>,

    #[serde(rename = "Composition")]
    pub composition: Option<Composition>,

    #[serde(rename = "SemiMajorAxis")]
    pub semi_major_axis: Option<f64>,

    #[serde(rename = "Eccentricity")]
    pub eccentricity: Option<f64>,

    #[serde(rename = "OrbitalInclination")]
    pub orbital_inclination: Option<f64>,

    #[serde(rename = "Periapsis")]
    pub periapsis: Option<f64>,

    #[serde(rename = "OrbitalPeriod")]
    pub orbital_period: Option<f64>,

    #[serde(rename = "AscendingNode")]
    pub ascending_node: Option<f64>,

    #[serde(rename = "MeanAnomaly")]
    pub mean_anomaly: Option<f64>,

    #[serde(rename = "RotationPeriod")]
    pub rotation_period: Option<f64>,

    #[serde(rename = "AxialTilt")]
    pub axial_tilt: Option<f64>,

    #[serde(rename = "Rings")]
    pub rings: Option<Vec<Rings>>,

    #[serde(rename = "ReserveLevel")]
    pub reserve_level: Option<String>,

    #[serde(rename = "WasDiscovered")]
    pub was_discovered: bool,

    #[serde(rename = "WasMapped")]
    pub was_mapped: bool,
}