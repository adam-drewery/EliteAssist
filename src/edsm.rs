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

mod into;
mod stations;
mod factions;
mod bodies;
mod deaths;
mod traffic;
mod system;
mod sphere_systems;
mod server_status;

pub use stations::*;
pub use factions::*;
pub use bodies::*;
pub use traffic::*;
pub use deaths::*;
pub use system::*;
pub use sphere_systems::*;
pub use server_status::*;

use std::time::Duration;
use log::info;
use reqwest::Url;
use serde::Deserialize;

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
        let resp = self.http.get(url.clone()).send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            info!("GET {} for {}", status, url);
            return Err(EdsmError::Message(format!(
                "http status {}",
                status
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
    ) -> Result<System, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-v1/system", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/bodies?systemName=... or systemId64=...
    pub async fn get_bodies(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<Bodies, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/bodies", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/stations?systemName=... or systemId64=...
    pub async fn get_stations(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<Stations, EdsmError> {
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
    ) -> Result<Vec<SphereSystem>, EdsmError> {
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

impl EdsmClient {
    /// GET https://www.edsm.net/api-system-v1/factions?systemName=... or systemId64=...
    pub async fn get_factions(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<Factions, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/factions", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/traffic?systemName=... or systemId64=...
    pub async fn get_traffic(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<Traffic, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/traffic", &q).await
    }

    /// GET https://www.edsm.net/api-system-v1/deaths?systemName=... or systemId64=...
    pub async fn get_deaths(
        &self,
        system_name: Option<&str>,
        system_id64: Option<u64>,
    ) -> Result<Deaths, EdsmError> {
        let q = self.build_system_query(system_name, system_id64)?;
        self.get_json("api-system-v1/deaths", &q).await
    }
}

/// Shared counts object for traffic/deaths
#[derive(Debug, Clone, Deserialize, Default)]
pub struct Counts {
    pub day: u64,
    pub week: u64,
    pub total: u64,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}