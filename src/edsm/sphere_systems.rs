use serde::Deserialize;
use crate::edsm::Coords;

/// Element of api-v1/sphere-systems response
#[derive(Debug, Clone, Deserialize, Default)]
pub struct SphereSystem {
    pub name: String,
    pub id: u64,
    pub id64: u64,
    pub coords: Coords,
    pub distance: f64,
}