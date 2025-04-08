use chrono::{DateTime, Utc};

#[derive(Default)]
pub struct ShipLoadout {

    pub timestamp: DateTime<Utc>,
    pub ship: String,
    pub ship_id: u64,
    pub ship_name: String,
    pub ship_ident: String,
    pub hull_value: u64,
    pub modules_value: u64,
    pub hull_health: f64,
    pub unladen_mass: f64,
    pub cargo_capacity: u64,
    pub max_jump_range: f64,
    pub fuel_capacity: FuelCapacity,
    pub rebuy: u64,
    pub modules: Vec<ShipModule>
}

pub struct ShipModule {

    pub slot: String,
    pub item: String,
    pub on: bool,
    pub priority: u8,
    pub health: f64,
    pub value: Option<u64>,
    pub ammo_in_clip: Option<u64>,
    pub ammo_in_hopper: Option<u64>,
    pub engineering: Option<Engineering>,
}

#[derive(Default)]
pub struct FuelCapacity {

    pub main: f64,
    pub reserve: f64,
}

pub struct Engineering {

    pub engineer: String,
    pub engineer_id: u64,
    pub blueprint_id: u64,
    pub blueprint_name: String,
    pub level: u8,
    pub quality: f64,
    pub experimental_effect: Option<String>,
    pub experimental_effect_localised: Option<String>,
    pub modifiers: Vec<Modifier>,
}

pub struct Modifier {

    pub label: String,
    pub value: f64,
    pub original_value: f64,
    pub less_is_good: u32,
}

#[derive(Default)]
pub struct ShipLocker {
    pub items: Vec<ShipLockerItem>,
    pub components: Vec<ShipLockerItem>,
    pub consumables: Vec<ShipLockerItem>,
    pub data: Vec<ShipLockerItem>
}

pub struct ShipLockerItem {
    pub name: String,
    pub count: u64,
    pub for_mission: bool,
}