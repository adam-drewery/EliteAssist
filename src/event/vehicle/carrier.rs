use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierMaterial {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Quantity")]
    pub quantity: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FCMaterials {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "CarrierName")]
    pub carrier_name: String,

    #[serde(rename = "CallSign")]
    pub callsign: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Materials")]
    pub materials: Vec<CarrierMaterial>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierJump {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Docked")]
    pub docked: bool,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationFaction")]
    pub station_faction: StationFaction,

    #[serde(rename = "StationGovernment")]
    pub station_government: String,

    #[serde(rename = "StationGovernment_Localised")]
    pub station_government_localised: Option<String>,

    #[serde(rename = "StationServices")]
    pub station_services: Vec<String>,

    #[serde(rename = "StationEconomy")]
    pub station_economy: String,

    #[serde(rename = "StationEconomy_Localised")]
    pub station_economy_localised: Option<String>,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "StarPos")]
    pub star_pos: Vec<f64>,

    #[serde(rename = "SystemAllegiance")]
    pub system_allegiance: String,

    #[serde(rename = "SystemEconomy")]
    pub system_economy: String,

    #[serde(rename = "SystemEconomy_Localised")]
    pub system_economy_localised: Option<String>,

    #[serde(rename = "SystemSecondEconomy")]
    pub system_second_economy: String,

    #[serde(rename = "SystemSecondEconomy_Localised")]
    pub system_second_economy_localised: Option<String>,

    #[serde(rename = "SystemGovernment")]
    pub system_government: String,

    #[serde(rename = "SystemGovernment_Localised")]
    pub system_government_localised: Option<String>,

    #[serde(rename = "SystemSecurity")]
    pub system_security: String,

    #[serde(rename = "SystemSecurity_Localised")]
    pub system_security_localised: Option<String>,

    #[serde(rename = "Population")]
    pub population: u64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "BodyType")]
    pub body_type: String,

    #[serde(rename = "Powers")]
    pub powers: Option<Vec<String>>,

    #[serde(rename = "PowerplayState")]
    pub powerplay_state: Option<String>,

    #[serde(rename = "Factions")]
    pub factions: Vec<Faction>,

    #[serde(rename = "SystemFaction")]
    pub system_faction: SystemFaction,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StationFaction {
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Faction {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: String,

    #[serde(rename = "Government")]
    pub government: String,

    #[serde(rename = "Influence")]
    pub influence: f64,

    #[serde(rename = "Allegiance")]
    pub allegiance: String,

    #[serde(rename = "Happiness")]
    pub happiness: String,

    #[serde(rename = "Happiness_Localised")]
    pub happiness_localised: Option<String>,

    #[serde(rename = "MyReputation")]
    pub my_reputation: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SystemFaction {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "FactionState")]
    pub faction_state: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierBuy {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "BoughtAtMarket")]
    pub bought_at_market: u64,

    #[serde(rename = "Location")]
    pub location: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Price")]
    pub price: u64,

    #[serde(rename = "Variant")]
    pub variant: String,

    #[serde(rename = "Callsign")]
    pub callsign: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ShipPack {
    #[serde(rename = "PackTheme")]
    pub pack_theme: String,

    #[serde(rename = "PackTier")]
    pub pack_tier: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ModulePack {
    #[serde(rename = "PackTheme")]
    pub pack_theme: String,

    #[serde(rename = "PackTier")]
    pub pack_tier: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierStats {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "Callsign")]
    pub callsign: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "DockingAccess")]
    pub docking_access: String,

    #[serde(rename = "AllowNotorious")]
    pub allow_notorious: bool,

    #[serde(rename = "FuelLevel")]
    pub fuel_level: u64,

    #[serde(rename = "JumpRangeCurr")]
    pub jump_range_curr: f64,

    #[serde(rename = "JumpRangeMax")]
    pub jump_range_max: f64,

    #[serde(rename = "PendingDecommission")]
    pub pending_decommission: bool,

    #[serde(rename = "ShipPacks")]
    pub ship_packs: Vec<ShipPack>,

    #[serde(rename = "ModulePacks")]
    pub module_packs: Vec<ModulePack>,

    #[serde(rename = "SpaceUsage")]
    pub space_usage: SpaceUsage,

    #[serde(rename = "Finance")]
    pub finance: Finance,

    #[serde(rename = "Crew")]
    pub crew: Vec<Crew>,

    #[serde(rename = "Services")]
    pub services: Services,
}

#[derive(Clone, Debug, Deserialize)]
pub struct SpaceUsage {
    #[serde(rename = "TotalCapacity")]
    pub total_capacity: u64,

    #[serde(rename = "Crew")]
    pub crew: u64,

    #[serde(rename = "Cargo")]
    pub cargo: u64,

    #[serde(rename = "CargoSpaceReserved")]
    pub cargo_space_reserved: u64,

    #[serde(rename = "ShipPacks")]
    pub ship_packs: u64,

    #[serde(rename = "ModulePacks")]
    pub module_packs: u64,

    #[serde(rename = "FreeSpace")]
    pub free_space: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Finance {
    #[serde(rename = "CarrierBalance")]
    pub carrier_balance: u64,

    #[serde(rename = "ReserveBalance")]
    pub reserve_balance: u64,

    #[serde(rename = "AvailableBalance")]
    pub available_balance: u64,

    #[serde(rename = "ReservePercent")]
    pub reserve_percent: u64,

    #[serde(rename = "TaxRate")]
    pub tax_rate: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Crew {
    #[serde(rename = "CrewRole")]
    pub crew_role: String,

    #[serde(rename = "Activated")]
    pub activated: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Services {
    #[serde(rename = "refuel")]
    pub refuel: ServiceStatus,

    #[serde(rename = "repair")]
    pub repair: ServiceStatus,

    #[serde(rename = "rearm")]
    pub rearm: ServiceStatus,

    #[serde(rename = "shipyard")]
    pub shipyard: ServiceStatus,

    #[serde(rename = "outfitting")]
    pub outfitting: ServiceStatus,

    #[serde(rename = "blackmarket")]
    pub blackmarket: ServiceStatus,

    #[serde(rename = "voucherredemption")]
    pub voucherredemption: ServiceStatus,

    #[serde(rename = "exploration")]
    pub exploration: ServiceStatus,

    #[serde(rename = "commodities")]
    pub commodities: ServiceStatus,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServiceStatus {
    #[serde(rename = "Status")]
    pub status: String,

    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierJumpRequest {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "SystemName")]
    pub system_name: String,

    #[serde(rename = "SystemAddress")]
    pub system_address: u64,

    #[serde(rename = "Body")]
    pub body: String,

    #[serde(rename = "BodyID")]
    pub body_id: u64,

    #[serde(rename = "DepartureTime")]
    pub departure_time: String,

    #[serde(rename = "Cancelled")]
    pub cancelled: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierDecommission {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "ScrapRefund")]
    pub scrap_refund: u64,

    #[serde(rename = "ScrapTime")]
    pub scrap_time: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierCancelDecommission {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierBankTransfer {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "Deposit")]
    pub deposit: u64,

    #[serde(rename = "PlayerBalance")]
    pub player_balance: u64,

    #[serde(rename = "CarrierBalance")]
    pub carrier_balance: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierDepositFuel {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "Amount")]
    pub amount: u64,

    #[serde(rename = "Total")]
    pub total: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierCrewServices {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "Operation")]
    pub operation: String,

    #[serde(rename = "CrewRole")]
    pub crew_role: String,

    #[serde(rename = "CrewName")]
    pub crew_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierFinance {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "TaxRate")]
    pub tax_rate: u64,

    #[serde(rename = "CarrierBalance")]
    pub carrier_balance: u64,

    #[serde(rename = "ReserveBalance")]
    pub reserve_balance: u64,

    #[serde(rename = "AvailableBalance")]
    pub available_balance: u64,

    #[serde(rename = "ReservePercent")]
    pub reserve_percent: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierShipPack {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "Operation")]
    pub operation: String,

    #[serde(rename = "PackTheme")]
    pub pack_theme: String,

    #[serde(rename = "PackTier")]
    pub pack_tier: u64,

    #[serde(rename = "Cost")]
    pub cost: u64,

    #[serde(rename = "Ships")]
    pub ships: Vec<Ship>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Ship {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Quantity")]
    pub quantity: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierModulePack {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "Operation")]
    pub operation: String,

    #[serde(rename = "PackTheme")]
    pub pack_theme: String,

    #[serde(rename = "PackTier")]
    pub pack_tier: u64,

    #[serde(rename = "Cost")]
    pub cost: u64,

    #[serde(rename = "Modules")]
    pub modules: Vec<Module>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Module {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Quantity")]
    pub quantity: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierTradeOrder {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "BlackMarket")]
    pub black_market: bool,

    #[serde(rename = "Commodity")]
    pub commodity: String,

    #[serde(rename = "CommodityLocalised")]
    pub commodity_localised: Option<String>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "CategoryLocalised")]
    pub category_localised: Option<String>,

    #[serde(rename = "PurchaseOrder")]
    pub purchase_order: u64,

    #[serde(rename = "Price")]
    pub price: u64,

    #[serde(rename = "BlackMarketPrice")]
    pub black_market_price: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierDockingPermission {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "DockingAccess")]
    pub docking_access: String,

    #[serde(rename = "AllowNotorious")]
    pub allow_notorious: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierNameChange {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,

    #[serde(rename = "Callsign")]
    pub callsign: String,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "OldName")]
    pub old_name: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CarrierJumpCancelled {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "CarrierID")]
    pub carrier_id: u64,
}