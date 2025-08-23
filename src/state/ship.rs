use ed_journals::logs::loadout_event::LoadoutEvent;
use ed_journals::logs::ship_locker_event::ShipLockerEventContents;
use ed_journals::ship::{CoreSlot, ShipSlotKind};

#[derive(Default)]
pub struct ShipLoadout {

    pub ship_type: String,
    pub ship_name: String,
    pub ship_ident: String,
    pub hull_value: u32,
    pub modules_value: u32,
    pub hull_health: f32,
    pub unladen_mass: f32,
    pub cargo_capacity: u16,
    pub max_jump_range: f32,
    pub fuel_capacity: FuelCapacity,
    pub rebuy: u32,
    pub hardpoints: Vec<ShipModule>,
    pub utilities: Vec<ShipModule>,
    pub core_internals: Vec<ShipModule>,
    pub optional_internals: Vec<ShipModule>,
}

pub struct ShipModule {

    pub slot: SlotType,
    pub name: String,
    pub on: bool,
    pub priority: u8,
    pub health: f32,
    pub value: Option<u32>,
    pub class: u8,
    pub rating: char,
    pub ammo_in_clip: Option<u32>,
    pub ammo_in_hopper: Option<u32>,
    pub engineering: Option<Engineering>,
    pub mount: String,
}

#[derive(Default)]
pub struct FuelCapacity {

    pub main: f32,
    pub reserve: f32,
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
    pub count: u16,
    pub mission_id: Option<u64>,
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
    PlanetaryApproachSuite,
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
    EngineColour
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

impl From <ed_journals::ship_locker::ShipLocker> for ShipLocker {
    fn from(value: ed_journals::ship_locker::ShipLocker) -> Self {
        ShipLocker {
            items: value.items.into_iter().map(|i| ShipLockerItem {
                name: i.name.to_string(),
                count: i.count,
                mission_id: i.mission_id,
                locations: i.locations.unwrap_or_default(),
            }).collect(),
            components: value.components.into_iter().map(|i| ShipLockerItem {
                name: i.name.to_string(),
                count: i.count,
                mission_id: i.mission_id,
                locations: i.locations.unwrap_or_default(),
            }).collect(),
            consumables: value.consumables.into_iter().map(|i| ShipLockerItem {
                name: i.name.to_string(),
                count: i.count,
                mission_id: i.mission_id,
                locations: i.locations.unwrap_or_default(),
            }).collect(),
            data: value.data.into_iter().map(|i| ShipLockerItem {
                name: i.name.to_string(),
                count: i.count,
                mission_id: i.mission_id,
                locations: i.locations.unwrap_or_default(),
            }).collect(),
        }
    }
}

impl From<ShipLockerEventContents> for ShipLocker {
    
    fn from(event: ShipLockerEventContents) -> Self {
        ShipLocker {
                items: event.items.into_iter().map(|i| ShipLockerItem {
                    name: i.name,
                    count: i.count,
                    mission_id: i.mission_id,
                    locations: i.locations.unwrap_or_default(),
                }).collect(),
                components: event.components.into_iter().map(|i| ShipLockerItem {
                    name: i.name,
                    count: i.count,
                    mission_id: i.mission_id,
                    locations: i.locations.unwrap_or_default(),
                }).collect(),
                consumables: event.consumables.into_iter().map(|i| ShipLockerItem {
                    name: i.name,
                    count: i.count,
                    mission_id: i.mission_id,
                    locations: i.locations.unwrap_or_default(),
                }).collect(),
                data: event.data.into_iter().map(|i| ShipLockerItem {
                    name: i.name,
                    count: i.count,
                    mission_id: i.mission_id,
                    locations: i.locations.unwrap_or_default(),
                }).collect(),
            }
        }
}

impl From<LoadoutEvent> for ShipLoadout {
    fn from(value: LoadoutEvent) -> Self {

        // Convert and categorize modules by slot type
        let mut hardpoints: Vec<ShipModule> = Vec::new();
        let mut utilities: Vec<ShipModule> = Vec::new();
        let mut core_internals: Vec<ShipModule> = Vec::new();
        let mut optional_internals: Vec<ShipModule> = Vec::new();

        for m in value.modules.into_iter() {
            let module = ShipModule {

                name: m.item.to_string(),
                slot: match m.slot.kind {
                    ShipSlotKind::ShipCockpit => SlotType::Miscellaneous(MiscellaneousType::ShipCockpit),
                    ShipSlotKind::CargoHatch => SlotType::Miscellaneous(MiscellaneousType::CargoHatch),
                    ShipSlotKind::UtilityMount => SlotType::Hardpoints { number: 0, size: 0 },
                    ShipSlotKind::Hardpoint(size) => SlotType::Hardpoints { number: 0, size: size.size_nr() },
                    ShipSlotKind::OptionalInternal(size) => SlotType::OptionalInternal(OptionalInternalType::Optional { number: 0, size }),
                    ShipSlotKind::Military => SlotType::OptionalInternal(OptionalInternalType::Military(0)),
                    ShipSlotKind::CoreInternal(kind) => SlotType::CoreInternal(match kind {
                        CoreSlot::Armour => CoreInternalType::Armour,
                        CoreSlot::PowerPlant => CoreInternalType::PowerPlant,
                        CoreSlot::MainEngines => CoreInternalType::MainEngines,
                        CoreSlot::FrameShiftDrive => CoreInternalType::FrameShiftDrive,
                        CoreSlot::LifeSupport => CoreInternalType::LifeSupport,
                        CoreSlot::PowerDistributor => CoreInternalType::PowerDistributor,
                        CoreSlot::Sensors => CoreInternalType::Radar,
                        CoreSlot::FuelTank => CoreInternalType::FuelTank,
                        CoreSlot::PlanetaryApproachSuite => CoreInternalType::PlanetaryApproachSuite
                    }),
                    ShipSlotKind::DataLinkScanner => SlotType::Miscellaneous(MiscellaneousType::DataLinkScanner),
                    ShipSlotKind::CodexScanner => SlotType::Miscellaneous(MiscellaneousType::CodexScanner),
                    ShipSlotKind::DiscoveryScanner => SlotType::Miscellaneous(MiscellaneousType::DiscoveryScanner),
                    ShipSlotKind::PaintJob => SlotType::Cosmetic(CosmeticType::PaintJob),
                    ShipSlotKind::Decal => SlotType::Cosmetic(CosmeticType::Decal(0)),
                    ShipSlotKind::VesselVoice => SlotType::Cosmetic(CosmeticType::VesselVoice),
                    ShipSlotKind::Nameplate => SlotType::Cosmetic(CosmeticType::ShipName(0)),
                    ShipSlotKind::IDPlate => SlotType::Cosmetic(CosmeticType::ShipID),
                    ShipSlotKind::Bobble => SlotType::Cosmetic(CosmeticType::Bobble(0)),
                    ShipSlotKind::StringLights => SlotType::Unknown,
                    ShipSlotKind::EngineColor => SlotType::Cosmetic(CosmeticType::EngineColour),
                    ShipSlotKind::WeaponColor => SlotType::Cosmetic(CosmeticType::WeaponColour),
                    ShipSlotKind::ShipKitSpoiler => SlotType::Cosmetic(CosmeticType::ShipKitSpoiler),
                    ShipSlotKind::ShipKitWings => SlotType::Cosmetic(CosmeticType::ShipKitWings),
                    ShipSlotKind::ShipKitTail => SlotType::Cosmetic(CosmeticType::ShipKitTail),
                    ShipSlotKind::ShipKitBumper => SlotType::Cosmetic(CosmeticType::ShipKitBumper),
                },

                on: m.on,
                priority: m.priority,
                health: m.health,
                value: m.value,
                class: m.class,
                rating: m.rating,
                ammo_in_clip: m.ammo_in_clip,
                ammo_in_hopper: m.ammo_in_hopper,
                engineering: None,
                mount: m.mount.unwrap_or_default(),
            };

            match module.slot {
                SlotType::Hardpoints { .. } => hardpoints.push(module),
                SlotType::CoreInternal(_) => core_internals.push(module),
                SlotType::OptionalInternal(_) => optional_internals.push(module),
                _ => utilities.push(module),
            }
        };

        ShipLoadout {
            ship_type: value.ship.to_string(),
            ship_name: value.ship_name,
            ship_ident: value.ship_ident,
            hull_value: value.hull_value.unwrap_or_default(),
            modules_value: value.modules_value.unwrap_or_default(),
            hull_health: value.hull_health,
            unladen_mass: value.unladen_mass,
            cargo_capacity: value.cargo_capacity,
            max_jump_range: value.max_jump_range,
            fuel_capacity: FuelCapacity {
                main: value.fuel_capacity.main,
                reserve: value.fuel_capacity.reserve,
            },
            rebuy: value.rebuy,
            hardpoints,
            utilities,
            core_internals,
            optional_internals,
        }
    }
}
