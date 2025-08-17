//! Asynchronous EDSM API client
//!
//! This client wraps selected endpoints from:
//! - https://www.edsm.net/en/api-status-v1
//! - https://www.edsm.net/en/api-system-v1
//! - https://www.edsm.net/en/api-v1
//!
//! It uses reqwest (async) and serde for JSON, and is safe to call from the GUI without
//! blocking the UI thread. Construct once and reuse to benefit from connection pooling.
//!
//! Example
//! ```no_run
//! use EliteAssist::edsm::EdsmClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = EdsmClient::default();
//!     let status = client.get_elite_server_status().await?;
//!     println!("Elite server status: {:?}", status);
//!     Ok(())
//! }
//! ```

use std::time::Duration;
use log::info;
use reqwest::Url;
use serde::{Deserialize, Serialize};

/// Error type for EDSM client operations
#[derive(thiserror::Error, Debug)]
pub enum EdsmError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("edsm error: {0}")]
    Message(String),
}

/// A lightweight asynchronous client for EDSM
#[derive(Clone)]
pub struct EdsmClient {
    http: reqwest::Client,
    base: Url,
}

impl Default for EdsmClient {
    fn default() -> Self {
        Self::new("https://www.edsm.net/")
            .expect("Static base URL should be valid")
    }
}

impl EdsmClient {
    /// Create a new client with a custom base URL (primarily for testing).
    pub fn new(base_url: &str) -> Result<Self, EdsmError> {
        let http = reqwest::Client::builder()
            .user_agent("EliteAssist-EDSM-Client/0.1")
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(4)
            .connect_timeout(Duration::from_secs(10))
            .build()?;
        let base = Url::parse(base_url)?;
        Ok(Self { http, base })
    }

    /// GET helper that merges a relative path and query params, and deserializes JSON.
    async fn get_json<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> Result<T, EdsmError> {
        let mut url = self.base.join(path)?;
        {
            let mut pairs = url.query_pairs_mut();
            for (k, v) in query {
                pairs.append_pair(k, v);
            }
        }

        info!("GET {}", url);
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(EdsmError::Message(format!(
                "http status {} body {}",
                status, text
            )));
        }
        let data = serde_json::from_str::<T>(&text)?;
        Ok(data)
    }

    /// Build system query parameters for endpoints that accept either systemId64 or systemName.
    /// If both are provided, id64 takes precedence; if neither provided, returns an error.
    fn build_system_query(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<Vec<(&'static str, String)>, EdsmError> {
        if let Some(id) = system_id64 {
            Ok(vec![("systemId64", id.to_string())])
        } else if let Some(name) = system_name {
            Ok(vec![("systemName", name.to_string())])
        } else {
            Err(EdsmError::Message(
                "either system_name or system_id64 must be provided".into(),
            ))
        }
    }

    // ========================= api-status-v1 =========================

    /// GET https://www.edsm.net/api-status-v1/elite-server
    pub async fn get_elite_server_status(&self) -> Result<EliteServerStatus, EdsmError> {
        self.get_json("api-status-v1/elite-server", &[]).await
    }

    // ========================= api-system-v1 =========================

    /// GET https://www.edsm.net/api-system-v1/system?systemName=... or systemId64=...
    /// If both name and id64 are provided, id64 takes precedence.
    pub async fn get_system(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<EdsmSystem, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-v1/system", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/bodies?systemName=... or systemId64=...
    pub async fn get_bodies(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<EdsmBodies, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/bodies", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/stations?systemName=... or systemId64=...
    pub async fn get_stations(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<EdsmStations, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/stations", &q).await
    }

    // ========================= api-v1 =========================

    /// GET https://www.edsm.net/api-v1/sphere-systems?systemName=...&radius=... (and optional params)
    /// Returns systems around a sphere centered at a system.
    pub async fn get_sphere_systems(
        &self,
        system_name: &str,
        radius_ly: u32,
        show_id: bool,
    ) -> Result<Vec<EdsmSphereSystem>, EdsmError> {
        let mut q: Vec<(&str, String)> = Vec::new();
        q.push(("systemName", system_name.to_string()));
        q.push(("radius", radius_ly.to_string()));
        if show_id {
            q.push(("showId", "1".to_string()));
        }
        self.get_json("api-v1/sphere-systems", &q).await
    }
}

// ------------------------------ Data Models ------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn live_status_call() {
        let client = EdsmClient::default();
        let status = client.get_elite_server_status().await.unwrap();
        println!("status: {:?}", status);
    }
}

// ------------------------------ Data Models ------------------------------

/// Elite server status response (shape kept minimal and tolerant).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliteServerStatus {
    #[serde(default)]
    pub status: Option<EliteServerStatusInner>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliteServerStatusInner {
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(rename = "lastUpdate")]
    pub last_update: Option<String>,
}

/// Response of api-system-v1/system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmSystem {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub coords: Option<Coords>,
    #[serde(default)]
    pub information: Option<SystemInformation>,
    #[serde(default)]
    #[serde(rename = "primaryStar")]
    pub primary_star: Option<PrimaryStar>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInformation {
    #[serde(default)]
    pub allegiance: Option<String>,
    #[serde(default)]
    pub government: Option<String>,
    #[serde(default)]
    pub economy: Option<String>,
    #[serde(default)]
    #[serde(rename = "secondEconomy")]
    pub second_economy: Option<String>,
    #[serde(default)]
    pub population: Option<u64>,
    #[serde(default)]
    pub security: Option<String>,
    #[serde(default)]
    pub faction: Option<String>,
    #[serde(default)]
    #[serde(rename = "factionState")]
    pub faction_state: Option<String>,
    #[serde(default)]
    pub permit: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryStar {
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "isScoopable")]
    pub is_scoopable: Option<bool>,
}

/// Response of api-system-v1/bodies
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmBodies {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(rename = "bodyCount")]
    pub body_count: Option<u32>,
    #[serde(default)]
    pub bodies: Option<Vec<EdsmBody>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmBody {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    #[serde(rename = "subType")]
    pub sub_type: Option<String>,
    #[serde(default)]
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: Option<f64>,
    #[serde(default)]
    #[serde(rename = "isMainStar")]
    pub is_main_star: Option<bool>,
    #[serde(default)]
    #[serde(rename = "isScoopable")]
    pub is_scoopable: Option<bool>,
}

/// Response of api-system-v1/stations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmStations {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub stations: Option<Vec<EdsmStation>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmStation {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    #[serde(rename = "distanceToStar")]
    pub distance_to_star: Option<f64>,
    #[serde(default)]
    #[serde(rename = "marketId")]
    pub market_id: Option<u64>,
    #[serde(default)]
    #[serde(rename = "haveMarket")]
    pub have_market: Option<bool>,
    #[serde(default)]
    #[serde(rename = "haveOutfitting")]
    pub have_outfitting: Option<bool>,
    #[serde(default)]
    #[serde(rename = "haveShipyard")]
    pub have_shipyard: Option<bool>,
    #[serde(default)]
    pub services: Option<Vec<String>>,
}

/// Element of api-v1/sphere-systems response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmSphereSystem {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub coords: Option<Coords>,
    #[serde(default)]
    pub distance: Option<f64>,
}


impl EdsmClient {
    /// GET https://www.edsm.net/api-system-v1/factions?systemName=... or systemId64=...
    pub async fn get_factions(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<EdsmFactions, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/factions", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/traffic?systemName=... or systemId64=...
    pub async fn get_traffic(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<EdsmTraffic, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/traffic", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/deaths?systemName=... or systemId64=...
    pub async fn get_deaths(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<EdsmDeaths, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/deaths", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/kills?systemName=... or systemId64=...
    pub async fn get_kills(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<EdsmKills, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/kills", &q).await
    }
}

// ------------------------------ Additional Data Models for api-system-v1 ------------------------------

/// Response of api-system-v1/factions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmFactions {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    #[serde(rename = "controllingFaction")]
    pub controlling_faction: Option<EdsmFaction>,
    #[serde(default)]
    pub factions: Option<Vec<EdsmFaction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmFaction {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub allegiance: Option<String>,
    #[serde(default)]
    pub government: Option<String>,
    #[serde(default)]
    pub influence: Option<f64>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub happiness: Option<String>,
    #[serde(default)]
    #[serde(rename = "isPlayer")]
    pub is_player: Option<bool>,
    #[serde(default)]
    #[serde(rename = "activeStates")]
    pub active_states: Option<Vec<EdsmFactionState>>, 
    #[serde(default)]
    #[serde(rename = "pendingStates")]
    pub pending_states: Option<Vec<EdsmFactionStateTrend>>, 
    #[serde(default)]
    #[serde(rename = "recoveringStates")]
    pub recovering_states: Option<Vec<EdsmFactionStateTrend>>, 
    #[serde(default)]
    #[serde(rename = "lastUpdate")]
    pub last_update: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmFactionState {
    #[serde(default)]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmFactionStateTrend {
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub trend: Option<i32>,
}

/// Shared counts object for traffic/deaths/kills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmCounts {
    #[serde(default)]
    pub day: Option<u64>,
    #[serde(default)]
    pub week: Option<u64>,
    #[serde(default)]
    pub total: Option<u64>,
}

/// Response of api-system-v1/traffic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmTraffic {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub traffic: Option<EdsmCounts>,
}

/// Response of api-system-v1/deaths
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmDeaths {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub deaths: Option<EdsmCounts>,
}

/// Response of api-system-v1/kills
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdsmKills {
    #[serde(default)]
    pub id: Option<u64>,
    #[serde(default)]
    pub id64: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub kills: Option<EdsmCounts>,
}
