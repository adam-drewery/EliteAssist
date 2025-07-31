use crate::fdev_ids::{Outfitting, Shipyard};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct AfmuRepairs {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Module")]
    pub module: String,

    #[serde(rename = "Module_Localised")]
    pub module_localised: Option<String>,

    #[serde(rename = "FullyRepaired")]
    pub fully_repaired: bool,

    #[serde(rename = "Health")]
    pub health: f64,
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
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
    pub hot: Option<bool>,

    #[serde(rename = "ShipType_Localised")]
    pub ship_type_localised: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct StoredShips {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "MarketID")]
    pub market_id: Option<u64>,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "ShipsHere")]
    pub ships_here: Vec<Ship>,

    #[serde(rename = "ShipsRemote")]
    pub ships_remote: Vec<Ship>,
}

#[derive(Clone, Debug, Deserialize)]
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

        let ship_type = Shipyard::metadata(&self.ship);

        crate::state::ShipLoadout {
            timestamp: self.timestamp,
            ship: ship_type.map(|s| s.name.clone()).unwrap_or(self.ship),
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

#[derive(Clone, Debug, Deserialize)]
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

        let (class, rating, name, mount) = Outfitting::metadata(&self.item)
            .map(|details| (
                details.class.parse().unwrap_or(0),
                details.rating.chars().next().unwrap_or('X'),
                details.name.clone(),
                details.mount.clone()
            ))
            .unwrap_or((0, 'X', self.item.clone(), "".to_string()));

        crate::state::ShipModule {
            slot: self.slot.into(),
            name,
            on: self.on,
            priority: self.priority,
            health: self.health,
            value: self.value,
            ammo_in_clip: self.ammo_in_clip,
            ammo_in_hopper: self.ammo_in_hopper,
            engineering: self.engineering.map(|e| e.into()),
            class,
            rating,
            mount
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
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

#[derive(Clone, Debug, Deserialize)]
pub struct RepairDrone {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "HullRepaired")]
    pub hull_repaired: f64,

    #[serde(rename = "CockpitRepaired")]
    pub cockpit_repaired: f64,

    #[serde(rename = "CorrosionRepaired")]
    pub corrosion_repaired: f64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RebootRepair {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Modules")]
    pub modules: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct CockpitBreached {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct JetConeDamage {
    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

    #[serde(rename = "Module")]
    pub module: String,

    #[serde(rename = "Module_Localised")]
    pub module_localised: String,
}


