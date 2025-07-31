use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct Composition {

    #[serde(rename = "Ice")]
    pub ice: f64,

    #[serde(rename = "Rock")]
    pub rock: f64,

    #[serde(rename = "Metal")]
    pub metal: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScanMaterial {

    #[serde(rename = "Name")]
    pub name: String,
    
    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Percent")]
    pub percent: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct AtmosphereComponent {

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Percent")]
    pub percent: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Parent {

    #[serde(rename = "Planet")]
    pub planet: Option<u32>,

    #[serde(rename = "Star")]
    pub star: Option<u32>,
    
    #[serde(rename = "Ring")]
    pub ring: Option<u32>,
    
    #[serde(rename = "Null")]
    pub null: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Scan {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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
    
    #[serde(rename = "AtmosphereComposition")]
    pub atmosphere_composition: Option<Vec<AtmosphereComponent>>,

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
    
    #[serde(rename = "StarType")]
    pub star_type: Option<String>,
    
    #[serde(rename = "Subclass")]
    pub subclass: Option<i32>,
    
    #[serde(rename = "StellarMass")]
    pub stellar_mass: Option<f64>,
    
    #[serde(rename = "AbsoluteMagnitude")]
    pub absolute_magnitude: Option<f64>,
    
    #[serde(rename = "Age_MY")]
    pub age_my: Option<i32>,
    
    #[serde(rename = "Luminosity")]
    pub luminosity: Option<String>,

    #[serde(rename = "WasDiscovered")]
    pub was_discovered: bool,

    #[serde(rename = "WasMapped")]
    pub was_mapped: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ScanBaryCentre {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

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

#[derive(Clone, Debug, Deserialize)]
pub struct ScanOrganic {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ScanType")]
    pub scan_type: String,

    #[serde(rename = "Genus")]
    pub genus: String,

    #[serde(rename = "Genus_Localised")]
    pub genus_localised: Option<String>,

    #[serde(rename = "Species")]
    pub species: String,

    #[serde(rename = "Species_Localised")]
    pub species_localised: Option<String>,

    #[serde(rename = "Variant")]
    pub variant: String,

    #[serde(rename = "Variant_Localised")]
    pub variant_localised: Option<String>,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Body")]
    pub body: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Scanned {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "ScanType")]
    pub scan_type: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DataScanned {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiscoveryScan {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Bodies")]
    pub bodies: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct NavBeaconScan {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "NumBodies")]
    pub num_bodies: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Screenshot {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Filename")]
    pub filename: String,

    #[serde(rename = "Width")]
    pub width: u32,

    #[serde(rename = "Height")]
    pub height: u32,

    #[serde(rename = "System")]
    pub system: Option<String>,

    #[serde(rename = "Body")]
    pub body: Option<String>,

    #[serde(rename = "Latitude")]
    pub latitude: Option<f64>,

    #[serde(rename = "Longitude")]
    pub longitude: Option<f64>,

    #[serde(rename = "Heading")]
    pub heading: Option<u32>,

    #[serde(rename = "Altitude")]
    pub altitude: Option<f64>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FSSSignal {
    
    #[serde(rename = "Type")]
    pub signal_type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FSSBodySignals {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "BodyName")]
    pub body_name: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Signals")]
    pub signals: Vec<FSSSignal>,
}