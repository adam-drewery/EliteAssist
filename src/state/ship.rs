use log::warn;
use regex::Regex;
use crate::journal::event;
use crate::lookup::fdev_ids::Outfitting;

#[derive(Default)]
pub struct ShipLoadout {

    pub ship_type: String,
    pub ship_name: String,
    pub ship_ident: String,
    pub hull_value: u64,
    pub modules_value: u64,
    pub hull_health: f64,
    pub unladen_mass: f64,
    pub cargo_capacity: u64,
    pub max_jump_range: f32,
    pub fuel_capacity: FuelCapacity,
    pub rebuy: u64,
    pub hardpoints: Vec<ShipModule>,
    pub utilities: Vec<ShipModule>,
    pub core_internals: Vec<ShipModule>,
    pub optional_internals: Vec<ShipModule>,
}

pub struct ShipModule {

    pub slot: SlotType,
    pub name: String,
    pub on: bool,
    pub priority: u64,
    pub health: f64,
    pub value: Option<u64>,
    pub class: u8,
    pub rating: char,
    pub ammo_in_clip: Option<u64>,
    pub ammo_in_hopper: Option<u64>,
    pub engineering: Option<Engineering>,
    pub mount: String,
}

#[derive(Default)]
pub struct FuelCapacity {

    pub main: f64,
    pub reserve: f64,
}

pub struct Engineering {

    pub engineer: String,
    pub blueprint_name: String,
    pub level: u64,
    pub quality: f64,
    pub experimental_effect: Option<String>,
    pub modifiers: Vec<Modifier>,
}

pub struct Modifier {

    pub label: String,
    pub value: f64,
    pub original_value: f64,
    pub less_is_good: u64,
}

impl From<event::LoadoutModuleEngineering> for Engineering {
    fn from(value: event::LoadoutModuleEngineering) -> Self {
        Engineering {
            engineer: value.engineer.unwrap_or_default(),
            blueprint_name: value
                .blueprint_name
                .split('_')
                .skip(1)
                .next()
                .unwrap_or_default()
                .to_string(),
            level: value.level,
            quality: value.quality,
            experimental_effect: value.experimental_effect_localised.or(value.experimental_effect),
            modifiers: value.modifiers.into_iter().map(|m| m.into()).collect(),
        }
    }
}

impl From<event::LoadoutModuleEngineeringModifier> for Modifier {
    fn from(value: event::LoadoutModuleEngineeringModifier) -> Self {
        Modifier {
            label: value.label,
            value: value.value.unwrap_or_default(),
            original_value: value.original_value.unwrap_or_default(),
            less_is_good: value.less_is_good.unwrap_or_default(),
        }
    }
}

impl From<event::LoadoutModule> for ShipModule {
    fn from(value: event::LoadoutModule) -> Self {
        let (class, rating, name, mount) = Outfitting::metadata(&value.item)
            .map(|details| (
                details.class.parse().unwrap_or(0),
                details.rating.chars().next().unwrap_or('X'),
                details.name.clone(),
                details.mount.clone(),
            ))
            .unwrap_or((0, 'X', value.item.clone(), "".to_string()));

        ShipModule {
            slot: value.slot.into(),
            name,
            on: value.on,
            priority: value.priority,
            health: value.health,
            value: value.value,
            ammo_in_clip: value.ammo_in_clip,
            ammo_in_hopper: value.ammo_in_hopper,
            engineering: value.engineering.map(|e| e.into()),
            class,
            rating,
            mount,
        }
    }
}

impl From<event::LoadoutFuelCapacity> for FuelCapacity {
    fn from(value: event::LoadoutFuelCapacity) -> Self {
        FuelCapacity {
            main: value.main,
            reserve: value.reserve,
        }
    }
}

#[derive(Default)]
pub struct ShipLocker {

    pub items: Vec<ShipLockerItem>,
    pub components: Vec<ShipLockerItem>,
    pub consumables: Vec<ShipLockerItem>,
    pub data: Vec<ShipLockerItem>
}

#[derive(Default)]
pub struct ShipLockerItem {

    pub name: String,
    pub count: u64,
    pub for_mission: bool,
    pub locations: Vec<String>
}

pub enum SlotType {
    Hardpoints { number: u8, size: u8 },
    CoreInternal(CoreInternalType),
    OptionalInternal(OptionalInternalType),
    Cosmetic(CosmeticType),
    Miscellaneous(MiscellaneousType),
    Unknown,
}

pub enum CoreInternalType {
    MainEngines,
    FrameShiftDrive,
    PowerDistributor,
    PowerPlant,
    LifeSupport,
    Radar,
    FuelTank,
    Armour,
}

pub enum OptionalInternalType {
    Optional { number: u8, size: u8 },
    Military(u8),
}

pub enum CosmeticType {
    ShipID,
    Bobble(u8),
    ShipKitSpoiler,
    ShipKitWings,
    ShipKitTail,
    ShipKitBumper,
    VesselVoice,
    PaintJob,
    Decal(u8),
    ShipName(u8),
    WeaponColour,
    EngineColour,
}

pub enum MiscellaneousType {
    ColonisationSuite,
    ShipCockpit,
    PlanetaryApproachSuite,
    DataLinkScanner,
    CodexScanner,
    DiscoveryScanner,
    CargoHatch,
}

impl From<String> for SlotType {

    fn from(value: String) -> Self {

        // Compile regular expressions only once using lazy_static
        use once_cell::sync::Lazy;
        static SLOT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"Slot(\d+)_Size(\d+)").unwrap());
        static NUMBERED_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(Military|Decal|ShipName|ShipID|Bobble)(\d+)").unwrap());
        static HARDPOINT_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"(Tiny|Small|Medium|Large|Huge)Hardpoint(\d+)").unwrap());

        // Handle optional slots like "Slot01_Size8"
        if let Some(captures) = SLOT_REGEX.captures(&value) {
            return SlotType::OptionalInternal(
                OptionalInternalType::Optional {
                    number: captures[1].parse().unwrap(),
                    size: captures[2].parse().unwrap(),
            });
        }

        // Handle numbered slots like "Military02", "Decal01", etc
        if let Some(captures) = NUMBERED_REGEX.captures(&value) {
            let number = captures[2].parse().unwrap();
            return match &captures[1] {
                "Military" => SlotType::OptionalInternal(OptionalInternalType::Military(number)),
                "Decal" => SlotType::Cosmetic(CosmeticType::Decal(number)),
                "ShipName" => SlotType::Cosmetic(CosmeticType::ShipName(number)),
                "ShipID" => SlotType::Cosmetic(CosmeticType::ShipID),
                "Bobble" => SlotType::Cosmetic(CosmeticType::Bobble(number)),
                _ => unreachable!()
            };
        }

        // Handle hardpoints like "MediumHardpoint2"
        if let Some(captures) = HARDPOINT_REGEX.captures(&value) {
            let size = match &captures[1] {
                "Tiny" => 0,
                "Small" => 1,
                "Medium" => 2,
                "Large" => 3,
                "Huge" => 4,
                _ => panic!("Unknown hardpoint size: {}", value)
            };
            return SlotType::Hardpoints {
                number: captures[2].parse().unwrap(),
                size,
            };
        }
                
        // Try to match enum variant name directly
        match value.as_str() {
            "MainEngines" => SlotType::CoreInternal(CoreInternalType::MainEngines),
            "FrameShiftDrive" => SlotType::CoreInternal(CoreInternalType::FrameShiftDrive),
            "PowerDistributor" => SlotType::CoreInternal(CoreInternalType::PowerDistributor),
            "PowerPlant" => SlotType::CoreInternal(CoreInternalType::PowerPlant),
            "LifeSupport" => SlotType::CoreInternal(CoreInternalType::LifeSupport),
            "Radar" => SlotType::CoreInternal(CoreInternalType::Radar),
            "Armour" => SlotType::CoreInternal(CoreInternalType::Armour),
            "FuelTank" => SlotType::CoreInternal(CoreInternalType::FuelTank),
            
            "CargoHatch" => SlotType::Miscellaneous(MiscellaneousType::CargoHatch),
            "ShipCockpit" => SlotType::Miscellaneous(MiscellaneousType::ShipCockpit),
            "PlanetaryApproachSuite" => SlotType::Miscellaneous(MiscellaneousType::PlanetaryApproachSuite),
            "DataLinkScanner" => SlotType::Miscellaneous(MiscellaneousType::DataLinkScanner),
            "CodexScanner" => SlotType::Miscellaneous(MiscellaneousType::CodexScanner),
            "DiscoveryScanner" => SlotType::Miscellaneous(MiscellaneousType::DiscoveryScanner),
            "ColonisationSuite" => SlotType::Miscellaneous(MiscellaneousType::ColonisationSuite),
            
            "WeaponColour" => SlotType::Cosmetic(CosmeticType::WeaponColour),
            "EngineColour" => SlotType::Cosmetic(CosmeticType::EngineColour),
            "PaintJob" => SlotType::Cosmetic(CosmeticType::PaintJob),
            "ShipKitSpoiler" => SlotType::Cosmetic(CosmeticType::ShipKitSpoiler),
            "ShipKitWings" => SlotType::Cosmetic(CosmeticType::ShipKitWings),
            "ShipKitTail" => SlotType::Cosmetic(CosmeticType::ShipKitTail),
            "ShipKitBumper" => SlotType::Cosmetic(CosmeticType::ShipKitBumper),
            "VesselVoice" => SlotType::Cosmetic(CosmeticType::VesselVoice),
            _ => {
                warn!("Unknown module slot: {}", value);
                SlotType::Unknown
            }
        }
    }
}
use crate::lookup::fdev_ids::Shipyard;
use crate::lookup;
use std::collections::HashMap;

impl From<event::Inventory> for ShipLocker {
    fn from(value: event::Inventory) -> Self {
        ShipLocker {
            items: map_vec(value.items),
            consumables: value.consumables.unwrap_or_default().into_iter().map(|c| c.into()).collect(),
            data: map_vec(value.data),
            components: value.components.unwrap_or_default().into_iter().map(|c| {
                ShipLockerItem {
                    name: c.name_localised.clone().unwrap_or(crate::journal::format::title_case(&c.name)),
                    for_mission: c.mission_id.is_some(),
                    count: c.count,
                    locations: lookup::locations_for_material(&c.name_localised.unwrap_or(c.name))
                }
            }).collect(),
        }
    }
}

impl From<event::Item> for ShipLockerItem {
    fn from(value: event::Item) -> Self {
        ShipLockerItem {
            name: value.name_localised.clone().unwrap_or(crate::journal::format::title_case(&value.name)),
            for_mission: value.mission_id.is_some(),
            count: value.count,
            locations: lookup::locations_for_item(&value.name_localised.unwrap_or(value.name))
        }
    }
}

impl From<event::Consumable> for ShipLockerItem {
    fn from(value: event::Consumable) -> Self {
        ShipLockerItem {
            name: value.name_localised.clone().unwrap_or(crate::journal::format::title_case(&value.name)),
            count: value.count,
            for_mission: false,
            locations: lookup::locations_for_item(&value.name_localised.unwrap_or(value.name))
        }
    }
}

fn group_and_sort(items: Vec<event::Item>) -> Vec<event::Item> {
    let mut grouped_items: HashMap<(String, Option<u64>), event::Item> = HashMap::new();
    for item in items {
        grouped_items
            .entry((item.name.clone(), item.mission_id))
            .and_modify(|e| e.count += item.count)
            .or_insert(item);
    }
    let mut items: Vec<_> = grouped_items.into_values().collect();
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

fn map_vec(vec: Option<Vec<event::Item>>) -> Vec<ShipLockerItem> {
    group_and_sort(vec.unwrap_or_default())
        .into_iter()
        .map(|f| f.into())
        .collect()
}

impl From<event::Loadout> for ShipLoadout {
    fn from(value: event::Loadout) -> Self {
        let ship_type = Shipyard::metadata(&value.ship);

        // Convert and categorize modules by slot type
        let mut hardpoints: Vec<ShipModule> = Vec::new();
        let mut utilities: Vec<ShipModule> = Vec::new();
        let mut core_internals: Vec<ShipModule> = Vec::new();
        let mut optional_internals: Vec<ShipModule> = Vec::new();

        for m in value.modules.into_iter() {
            let module: ShipModule = m.into();
            match &module.slot {
                SlotType::Hardpoints { size, .. } => {
                    if *size == 0 { utilities.push(module); } else { hardpoints.push(module); }
                }
                SlotType::CoreInternal(_) => core_internals.push(module),
                SlotType::OptionalInternal(_) => optional_internals.push(module),
                SlotType::Cosmetic(_) | SlotType::Miscellaneous(_) | SlotType::Unknown => {}
            }
        }

        ShipLoadout {
            ship_type: ship_type.map(|s| s.name.clone()).unwrap_or(value.ship),
            ship_name: value.ship_name,
            ship_ident: value.ship_ident,
            hull_value: value.hull_value.unwrap_or_default(),
            modules_value: value.modules_value.unwrap_or_default(),
            hull_health: value.hull_health,
            unladen_mass: value.unladen_mass,
            cargo_capacity: value.cargo_capacity,
            max_jump_range: value.max_jump_range as f32,
            fuel_capacity: value.fuel_capacity.into(),
            rebuy: value.rebuy,
            hardpoints,
            utilities,
            core_internals,
            optional_internals,
        }
    }
}
