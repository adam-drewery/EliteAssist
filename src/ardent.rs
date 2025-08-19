//! Asynchronous Ardent API client
//!
//! This client wraps the Ardent API endpoints from:
//! - https://api.ardent-insight.com/v2/
//!
//! The Ardent API provides access to Elite Dangerous trading data, system information,
//! and commodity market data submitted to the Elite Dangerous Data Network.
//!
//! It uses reqwest (async) and serde for JSON, and is safe to call from the GUI without
//! blocking the UI thread. Construct once and reuse to benefit from connection pooling.
//!
//! Example
//! ```no_run
//! use EliteAssist::ardent::ArdentClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = ArdentClient::default();
//!     let version = client.get_version().await?;
//!     println!("Ardent API version: {:?}", version);
//!     Ok(())
//! }
//! ```

use std::time::Duration;
use log::info;
use reqwest::Url;
use serde::Deserialize;

#[cfg(test)]
mod test;

/// A lightweight asynchronous client for Ardent API
#[derive(Clone)]
pub struct ArdentClient {
    http: reqwest::Client,
    base: Url,
}

impl Default for ArdentClient {
    fn default() -> Self {
        Self::new("https://api.ardent-insight.com/v2/")
            .expect("Static base URL should be valid")
    }
}

impl ArdentClient {
    /// Create a new client with a custom base URL (primarily for testing).
    pub fn new(base_url: &str) -> Result<Self, ArdentError> {
        let http = reqwest::Client::builder()
            .user_agent("EliteAssist-Ardent-Client/0.1")
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
    ) -> Result<T, ArdentError> {
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
            return Err(ArdentError::Message(format!(
                "http status {}",
                status
            )));
        }
        let data = serde_json::from_str::<T>(&text)?;
        Ok(data)
    }

    // ========================= Version and Statistics =========================

    /// GET https://api.ardent-insight.com/v2/version
    pub async fn get_version(&self) -> Result<Version, ArdentError> {
        self.get_json("version", &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/stats
    pub async fn get_stats(&self) -> Result<Stats, ArdentError> {
        self.get_json("stats", &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/stats/stations/economies
    pub async fn get_station_economies_stats(&self) -> Result<StationEconomiesStats, ArdentError> {
        self.get_json("stats/stations/economies", &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/stats/stations/types
    pub async fn get_station_types_stats(&self) -> Result<StationTypeStats, ArdentError> {
        self.get_json("stats/stations/types", &[]).await
    }

    // ========================= Commodities =========================

    /// GET https://api.ardent-insight.com/v2/commodities
    pub async fn get_commodities_report(&self) -> Result<Vec<CommodityReport>, ArdentError> {
        self.get_json("commodities", &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/commodity/name/{commodityName}
    pub async fn get_commodity_info(&self, commodity_name: &str) -> Result<CommodityInfo, ArdentError> {
        let path = format!("commodity/name/{}", commodity_name);
        self.get_json(&path, &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/commodity/name/{commodityName}/imports
    pub async fn get_commodity_imports(
        &self,
        commodity_name: &str,
        params: Option<CommodityQueryParams>
    ) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("commodity/name/{}/imports", commodity_name);
        let query_params = params.unwrap_or_default();
        let query = query_params.to_query_params();
        self.get_json(&path, &query).await
    }

    /// GET https://api.ardent-insight.com/v2/commodity/name/{commodityName}/exports
    pub async fn get_commodity_exports(
        &self,
        commodity_name: &str,
        params: Option<CommodityQueryParams>
    ) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("commodity/name/{}/exports", commodity_name);
        let query_params = params.unwrap_or_default();
        let query = query_params.to_query_params();
        self.get_json(&path, &query).await
    }

    // ========================= System Information =========================

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}
    pub async fn get_system_info(&self, system_name: &str) -> Result<SystemInfo, ArdentError> {
        let path = format!("system/name/{}", system_name);
        self.get_json(&path, &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/system/address/{systemAddress}
    pub async fn get_system_info_by_address(&self, system_address: u64) -> Result<SystemInfo, ArdentError> {
        let path = format!("system/address/{}", system_address);
        self.get_json(&path, &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/nearby
    pub async fn get_nearby_systems(
        &self,
        system_name: &str,
        max_distance: Option<u32>
    ) -> Result<Vec<NearbySystem>, ArdentError> {
        let path = format!("system/name/{}/nearby", system_name);
        let mut query = Vec::new();
        if let Some(distance) = max_distance {
            query.push(("maxDistance", distance.to_string()));
        }
        self.get_json(&path, &query).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/nearest/{service}
    pub async fn get_nearest_service(
        &self,
        system_name: &str,
        service: &str,
        min_landing_pad_size: Option<u32>
    ) -> Result<Vec<NearestService>, ArdentError> {
        let path = format!("system/name/{}/nearest/{}", system_name, service);
        let mut query = Vec::new();
        if let Some(pad_size) = min_landing_pad_size {
            query.push(("minLandingPadSize", pad_size.to_string()));
        }
        self.get_json(&path, &query).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/commodities
    pub async fn get_system_commodities(&self, system_name: &str) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("system/name/{}/commodities", system_name);
        self.get_json(&path, &[]).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/commodities/imports
    pub async fn get_system_commodity_imports(
        &self,
        system_name: &str,
        params: Option<CommodityQueryParams>
    ) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("system/name/{}/commodities/imports", system_name);
        let query_params = params.unwrap_or_default();
        let query = query_params.to_query_params();
        self.get_json(&path, &query).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/commodities/exports
    pub async fn get_system_commodity_exports(
        &self,
        system_name: &str,
        params: Option<CommodityQueryParams>
    ) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("system/name/{}/commodities/exports", system_name);
        let query_params = params.unwrap_or_default();
        let query = query_params.to_query_params();
        self.get_json(&path, &query).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/commodity/name/{commodityName}
    pub async fn get_system_commodity_data(
        &self,
        system_name: &str,
        commodity_name: &str,
        max_days_ago: Option<u32>
    ) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("system/name/{}/commodity/name/{}", system_name, commodity_name);
        let mut query = Vec::new();
        if let Some(days) = max_days_ago {
            query.push(("maxDaysAgo", days.to_string()));
        }
        self.get_json(&path, &query).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/commodity/name/{commodityName}/nearby/imports
    pub async fn get_nearby_commodity_imports(
        &self,
        system_name: &str,
        commodity_name: &str,
        params: Option<NearbyCommodityQueryParams>
    ) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("system/name/{}/commodity/name/{}/nearby/imports", system_name, commodity_name);
        let query_params = params.unwrap_or_default();
        let query = query_params.to_query_params();
        self.get_json(&path, &query).await
    }

    /// GET https://api.ardent-insight.com/v2/system/name/{systemName}/commodity/name/{commodityName}/nearby/exports
    pub async fn get_nearby_commodity_exports(
        &self,
        system_name: &str,
        commodity_name: &str,
        params: Option<NearbyCommodityQueryParams>
    ) -> Result<Vec<Commodity>, ArdentError> {
        let path = format!("system/name/{}/commodity/name/{}/nearby/exports", system_name, commodity_name);
        let query_params = params.unwrap_or_default();
        let query = query_params.to_query_params();
        self.get_json(&path, &query).await
    }

    // ========================= Market Information =========================

    /// GET https://api.ardent-insight.com/v2/market/{marketId}/commodity/name/{commodityName}
    pub async fn get_market_commodity_data(
        &self,
        market_id: u64,
        commodity_name: &str
    ) -> Result<SystemCommodity, ArdentError> {
        let path = format!("market/{}/commodity/name/{}", market_id, commodity_name);
        self.get_json(&path, &[]).await
    }
}

// ========================= Data Structures =========================

#[derive(Debug, Clone, Deserialize)]
pub struct Version {
    pub version: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Stats {
    pub systems: u64,
    #[serde(rename = "pointsOfInterest")]
    pub points_of_interest: u64,
    pub stations: StationStats,
    pub trade: TradeStats,
    #[serde(rename = "updatedInLast24Hours")]
    pub updated_in_last_24_hours: u64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationStats {
    pub stations: u64,
    pub carriers: u64,
    #[serde(rename = "updatedInLast24Hours")]
    pub updated_in_last_24_hours: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TradeStats {
    pub markets: u64,
    pub orders: u64,
    #[serde(rename = "updatedInLast24Hours")]
    pub updated_in_last_24_hours: u64,
    #[serde(rename = "uniqueCommodities")]
    pub unique_commodities: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationEconomiesStats {
    pub primary: std::collections::HashMap<String, u64>,
    pub secondary: std::collections::HashMap<String, u64>,
    #[serde(rename = "fleetCarriers")]
    pub fleet_carriers: u64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationTypes {
    pub null: u64,
    #[serde(rename = "")]
    pub field: u64,
    #[serde(rename = "AsteroidBase")]
    pub asteroid_base: u64,
    #[serde(rename = "Bernal")]
    pub bernal: u64,
    #[serde(rename = "Coriolis")]
    pub coriolis: u64,
    #[serde(rename = "CraterOutpost")]
    pub crater_outpost: u64,
    #[serde(rename = "CraterPort")]
    pub crater_port: u64,
    #[serde(rename = "FleetCarrier")]
    pub fleet_carrier: u64,
    #[serde(rename = "MegaShip")]
    pub mega_ship: u64,
    #[serde(rename = "Ocellus")]
    pub ocellus: u64,
    #[serde(rename = "OnFootSettlement")]
    pub on_foot_settlement: u64,
    #[serde(rename = "Orbis")]
    pub orbis: u64,
    #[serde(rename = "Outpost")]
    pub outpost: u64,
    #[serde(rename = "PlanetaryConstructionDepot")]
    pub planetary_construction_depot: u64,
    #[serde(rename = "SpaceConstructionDepot")]
    pub space_construction_depot: u64,
    #[serde(rename = "StrongholdCarrier")]
    pub stronghold_carrier: u64,
    #[serde(rename = "SurfaceStation")]
    pub surface_station: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StationTypeStats {
    #[serde(rename = "stationTypes")]
    pub station_types: StationTypes,
    pub total: i64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommodityReport {
    #[serde(rename = "commodityName")]
    pub commodity_name: String,
    #[serde(rename = "maxBuyPrice")]
    pub max_buy_price: Option<u64>,
    #[serde(rename = "minBuyPrice")]
    pub min_buy_price: Option<u64>,
    #[serde(rename = "avgBuyPrice")]
    pub avg_buy_price: Option<u64>,
    #[serde(rename = "totalStock")]
    pub total_stock: Option<u64>,
    #[serde(rename = "maxSellPrice")]
    pub max_sell_price: Option<u64>,
    #[serde(rename = "minSellPrice")]
    pub min_sell_price: Option<u64>,
    #[serde(rename = "avgSellPrice")]
    pub avg_sell_price: Option<u64>,
    #[serde(rename = "totalDemand")]
    pub total_demand: Option<u64>,
    pub rare: Option<bool>,
    #[serde(rename = "rareMarketId")]
    pub rare_market_id: Option<u64>,
    #[serde(rename = "rareMaxCount")]
    pub rare_max_count: Option<u64>,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CommodityInfo {
    #[serde(rename = "commodityName")]
    pub commodity_name: String,
    // Add more fields as needed based on actual API response
}

#[derive(Debug, Clone, Deserialize)]
pub struct Commodity {
    #[serde(rename = "commodityName")]
    pub commodity_name: String,
    #[serde(rename = "marketId")]
    pub market_id: u64,
    #[serde(rename = "stationName")]
    pub station_name: String,
    #[serde(rename = "stationType")]
    pub station_type: String,
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: f64,
    #[serde(rename = "maxLandingPadSize")]
    pub max_landing_pad_size: i8,
    #[serde(rename = "bodyId")]
    pub body_id: Option<u64>,
    #[serde(rename = "bodyName")]
    pub body_name: Option<String>,
    #[serde(rename = "systemAddress")]
    pub system_address: u64,
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "systemX")]
    pub system_x: f64,
    #[serde(rename = "systemY")]
    pub system_y: f64,
    #[serde(rename = "systemZ")]
    pub system_z: f64,
    #[serde(rename = "buyPrice")]
    pub buy_price: u32,
    pub demand: u32,

    // todo: this doesn't work because presumably PHP jank is returning empty values as an empty string.
    // #[serde(rename = "demandBracket")]
    // pub demand_bracket: u32,
    #[serde(rename = "meanPrice")]
    pub mean_price: u32,
    #[serde(rename = "sellPrice")]
    pub sell_price: u32,
    pub stock: u32,

    // todo: this also probably doesn't work.
    // #[serde(rename = "stockBracket")]
    // pub stock_bracket: u32,

    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SystemCommodity {
    #[serde(rename = "commodityName")]
    pub commodity_name: String,
    #[serde(rename = "marketId")]
    pub market_id: u64,
    #[serde(rename = "buyPrice")]
    pub buy_price: u32,
    pub demand: u32,
    #[serde(rename = "demandBracket")]
    pub demand_bracket: String,
    #[serde(rename = "meanPrice")]
    pub mean_price: u32,
    #[serde(rename = "sellPrice")]
    pub sell_price: u32,
    pub stock: u32,
    #[serde(rename = "stockBracket")]
    pub stock_bracket: u32,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SystemInfo {
    #[serde(rename = "systemAddress")]
    pub system_address: i64,
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "systemX")]
    pub system_x: i64,
    #[serde(rename = "systemY")]
    pub system_y: i64,
    #[serde(rename = "systemZ")]
    pub system_z: i64,
    #[serde(rename = "systemSector")]
    pub system_sector: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NearbySystem {
    #[serde(rename = "systemAddress")]
    pub address: i64,
    #[serde(rename = "systemName")]
    pub name: String,
    #[serde(rename = "systemX")]
    pub x: f64,
    #[serde(rename = "systemY")]
    pub y: f64,
    #[serde(rename = "systemZ")]
    pub z: f64,
    #[serde(rename = "systemSector")]
    pub sector: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub distance: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NearestService {
    #[serde(rename = "marketId")]
    pub market_id: i64,
    #[serde(rename = "stationName")]
    pub station_name: String,
    #[serde(rename = "distanceToArrival")]
    pub distance_to_arrival: f64,
    #[serde(rename = "stationType")]
    pub station_type: String,
    pub allegiance: Option<String>,
    pub government: Option<String>,
    #[serde(rename = "controllingFaction")]
    pub controlling_faction: Option<String>,
    #[serde(rename = "primaryEconomy")]
    pub primary_economy: Option<String>,
    #[serde(rename = "secondaryEconomy")]
    pub secondary_economy: Option<String>,
    pub shipyard: u8,
    pub outfitting: u8,
    #[serde(rename = "blackMarket")]
    pub black_market: u8,
    pub contacts: u8,
    #[serde(rename = "crewLounge")]
    pub crew_lounge: u8,
    #[serde(rename = "interstellarFactors")]
    pub interstellar_factors: u8,
    #[serde(rename = "materialTrader")]
    pub material_trader: u8,
    pub missions: u8,
    pub refuel: u8,
    pub repair: u8,
    pub restock: u8,
    #[serde(rename = "searchAndRescue")]
    pub search_and_rescue: u8,
    #[serde(rename = "technologyBroker")]
    pub technology_broker: u8,
    pub tuning: u8,
    #[serde(rename = "universalCartographics")]
    pub universal_cartographics: u8,
    #[serde(rename = "systemAddress")]
    pub system_address: i64,
    #[serde(rename = "systemName")]
    pub system_name: String,
    #[serde(rename = "systemX")]
    pub system_x: f64,
    #[serde(rename = "systemY")]
    pub system_y: f64,
    #[serde(rename = "systemZ")]
    pub system_z: f64,
    #[serde(rename = "bodyId")]
    pub body_id: Option<i64>,
    #[serde(rename = "bodyName")]
    pub body_name: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    #[serde(rename = "maxLandingPadSize")]
    pub max_landing_pad_size: i64,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub distance: i64,
}

// ========================= Query Parameters =========================

#[derive(Debug, Clone, Default)]
pub struct CommodityQueryParams {
    pub min_volume: Option<u32>,
    pub min_price: Option<u32>,
    pub max_price: Option<u32>,
    pub fleet_carriers: Option<bool>,
    pub max_days_ago: Option<u32>,
}

impl CommodityQueryParams {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(vol) = self.min_volume {
            params.push(("minVolume", vol.to_string()));
        }
        if let Some(price) = self.min_price {
            params.push(("minPrice", price.to_string()));
        }
        if let Some(price) = self.max_price {
            params.push(("maxPrice", price.to_string()));
        }
        if let Some(fc) = self.fleet_carriers {
            params.push(("fleetCarriers", if fc { "1" } else { "0" }.to_string()));
        }
        if let Some(days) = self.max_days_ago {
            params.push(("maxDaysAgo", days.to_string()));
        }
        params
    }
}

#[derive(Debug, Clone, Default)]
pub struct NearbyCommodityQueryParams {
    pub min_volume: Option<u32>,
    pub min_price: Option<u32>,
    pub max_price: Option<u32>,
    pub fleet_carriers: Option<bool>,
    pub max_distance: Option<u32>,
    pub max_days_ago: Option<u32>,
}

impl NearbyCommodityQueryParams {
    fn to_query_params(&self) -> Vec<(&str, String)> {
        let mut params = Vec::new();
        if let Some(vol) = self.min_volume {
            params.push(("minVolume", vol.to_string()));
        }
        if let Some(price) = self.min_price {
            params.push(("minPrice", price.to_string()));
        }
        if let Some(price) = self.max_price {
            params.push(("maxPrice", price.to_string()));
        }
        if let Some(fc) = self.fleet_carriers {
            params.push(("fleetCarriers", if fc { "1" } else { "0" }.to_string()));
        }
        if let Some(dist) = self.max_distance {
            params.push(("maxDistance", dist.to_string()));
        }
        if let Some(days) = self.max_days_ago {
            params.push(("maxDaysAgo", days.to_string()));
        }
        params
    }
}

// ========================= Error Handling =========================

/// Error type for Ardent client operations
#[derive(thiserror::Error, Debug)]
pub enum ArdentError {
    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("ardent error: {0}")]
    Message(String),
}

