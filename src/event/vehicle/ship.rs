use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::fdev_ids::outfitting_details;

#[derive(Deserialize, Debug, Clone)]
pub struct SetUserShipName {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "UserShipName")]
    pub user_ship_name: String,

    #[serde(rename = "UserShipId")]
    pub user_ship_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AfmuRepairs {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Module")]
    pub module: String,

    #[serde(rename = "Module_Localised")]
    pub module_localised: String,

    #[serde(rename = "FullyRepaired")]
    pub fully_repaired: bool,

    #[serde(rename = "Health")]
    pub health: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct HullDamage {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Health")]
    pub health: f64,

    #[serde(rename = "PlayerPilot")]
    pub player_pilot: bool,

    #[serde(rename = "Fighter")]
    pub fighter: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Ship {

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipType")]
    pub ship_type: String,

    #[serde(rename = "Name")]
    pub name: Option<String>,

    #[serde(rename = "Value")]
    pub value: u32,

    #[serde(rename = "Hot")]
    pub hot: bool,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StoredShips {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "ShipsHere")]
    pub ships_here: Vec<Ship>,

    #[serde(rename = "ShipsRemote")]
    pub ships_remote: Vec<Ship>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShipLoadout {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Ship")]
    pub ship: String,

    #[serde(rename = "ShipID")]
    pub ship_id: u64,

    #[serde(rename = "ShipName")]
    pub ship_name: String,

    #[serde(rename = "ShipIdent")]
    pub ship_ident: String,

    #[serde(rename = "HullValue")]
    pub hull_value: u64,

    #[serde(rename = "ModulesValue")]
    pub modules_value: u64,

    #[serde(rename = "HullHealth")]
    pub hull_health: f64,

    #[serde(rename = "UnladenMass")]
    pub unladen_mass: f64,

    #[serde(rename = "CargoCapacity")]
    pub cargo_capacity: u64,

    #[serde(rename = "MaxJumpRange")]
    pub max_jump_range: f64,

    #[serde(rename = "FuelCapacity")]
    pub fuel_capacity: FuelCapacity,

    #[serde(rename = "Rebuy")]
    pub rebuy: u64,

    #[serde(rename = "Modules")]
    pub modules: Vec<ShipModule>
}

impl Into<crate::state::ShipLoadout> for ShipLoadout {
    fn into(self) -> crate::state::ShipLoadout {
        crate::state::ShipLoadout {
            timestamp: self.timestamp,
            ship: self.ship,
            ship_id: self.ship_id,
            ship_name: self.ship_name,
            ship_ident: self.ship_ident,
            hull_value: self.hull_value,
            modules_value: self.modules_value,
            hull_health: self.hull_health,
            unladen_mass: self.unladen_mass,
            cargo_capacity: self.cargo_capacity,
            max_jump_range: self.max_jump_range,
            fuel_capacity: self.fuel_capacity.into(),
            rebuy: self.rebuy,
            modules: self.modules.into_iter().map(|m| m.into()).collect(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ShipModule {
    #[serde(rename = "Slot")]
    pub slot: String,

    #[serde(rename = "Item")]
    pub item: String,

    #[serde(rename = "On")]
    pub on: bool,

    #[serde(rename = "Priority")]
    pub priority: u8,

    #[serde(rename = "Health")]
    pub health: f64,

    #[serde(rename = "Value")]
    pub value: Option<u64>,

    #[serde(rename = "AmmoInClip")]
    pub ammo_in_clip: Option<u64>,

    #[serde(rename = "AmmoInHopper")]
    pub ammo_in_hopper: Option<u64>,

    #[serde(rename = "Engineering")]
    pub engineering: Option<crate::event::inventory::Engineering>,
}

impl Into<crate::state::ShipModule> for ShipModule {
    fn into(self) -> crate::state::ShipModule {
        let details = outfitting_details(&self.item);

        crate::state::ShipModule {
            slot: self.slot,
            item: details.map(|d| d.name.clone()).unwrap_or(self.item),
            on: self.on,
            priority: self.priority,
            health: self.health,
            value: self.value,
            ammo_in_clip: self.ammo_in_clip,
            ammo_in_hopper: self.ammo_in_hopper,
            engineering: self.engineering.map(|e| e.into()),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct FuelCapacity {
    #[serde(rename = "Main")]
    pub main: f64,

    #[serde(rename = "Reserve")]
    pub reserve: f64,
}

impl Into<crate::state::FuelCapacity> for FuelCapacity {
    fn into(self) -> crate::state::FuelCapacity {
        crate::state::FuelCapacity {
            main: self.main,
            reserve: self.reserve,
        }
    }
}


