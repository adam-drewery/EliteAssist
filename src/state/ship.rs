use chrono::{DateTime, Utc};
use log::warn;

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

    pub slot: ModuleSlot,
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

pub enum ModuleSlot {

    MainEngines,
    FrameShiftDrive,
    PowerDistributor,
    PowerPlant,
    LifeSupport,
    CargoHatch,
    Radar,
    ShipCockpit,
    PlanetaryApproachSuite,
    WeaponColour,
    EngineColour,
    DataLinkScanner,
    CodexScanner,
    DiscoveryScanner,
    ColonisationSuite,
    Optional { number: u8, size: u8 },
    Military(u8),
    Hardpoint { number: u8, size: HardpointSize },
    PaintJob,
    Decal(u8),
    ShipName(u8),
    ShipID(u8),
    Armour,
    FuelTank,
    Bobble(u8),
    ShipKitSpoiler,
    ShipKitWings,
    ShipKitTail,
    ShipKitBumper,
    VesselVoice,

    Unknown(String),
}

pub enum HardpointSize {
    Tiny,
    Small,
    Medium,
    Large,
    Huge,
}

impl From<String> for ModuleSlot {

    fn from(value: String) -> Self {

        // Compile regular expressions only once using lazy_static
        use once_cell::sync::Lazy;
        static SLOT_REGEX: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"Slot(\d+)_Size(\d+)").unwrap());
        static NUMBERED_REGEX: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"(Military|Decal|ShipName|ShipID|Bobble)(\d+)").unwrap());
        static HARDPOINT_REGEX: Lazy<regex::Regex> = Lazy::new(|| regex::Regex::new(r"(Tiny|Small|Medium|Large|Huge)Hardpoint(\d+)").unwrap());

        // Handle optional slots like "Slot01_Size8"
        if let Some(captures) = SLOT_REGEX.captures(&value) {
            return ModuleSlot::Optional {
                number: captures[1].parse().unwrap(),
                size: captures[2].parse().unwrap(),
            };
        }

        // Handle numbered slots like "Military02", "Decal01", etc
        if let Some(captures) = NUMBERED_REGEX.captures(&value) {
            let number = captures[2].parse().unwrap();
            return match &captures[1] {
                "Military" => ModuleSlot::Military(number),
                "Decal" => ModuleSlot::Decal(number),
                "ShipName" => ModuleSlot::ShipName(number),
                "ShipID" => ModuleSlot::ShipID(number),
                "Bobble" => ModuleSlot::Bobble(number),
                _ => unreachable!()
            };
        }

        // Handle hardpoints like "MediumHardpoint2"
        if let Some(captures) = HARDPOINT_REGEX.captures(&value) {
            let size = match &captures[1] {
                "Tiny" => HardpointSize::Tiny,
                "Small" => HardpointSize::Small,
                "Medium" => HardpointSize::Medium,
                "Large" => HardpointSize::Large,
                "Huge" => HardpointSize::Huge,
                _ => panic!("Unknown hardpoint size: {}", value)
            };
            return ModuleSlot::Hardpoint {
                number: captures[2].parse().unwrap(),
                size,
            };
        }

        // Try to match enum variant name directly
        match value.as_str() {
            "MainEngines" => ModuleSlot::MainEngines,
            "FrameShiftDrive" => ModuleSlot::FrameShiftDrive,
            "PowerDistributor" => ModuleSlot::PowerDistributor,
            "PowerPlant" => ModuleSlot::PowerPlant,
            "LifeSupport" => ModuleSlot::LifeSupport,
            "CargoHatch" => ModuleSlot::CargoHatch,
            "Radar" => ModuleSlot::Radar,
            "ShipCockpit" => ModuleSlot::ShipCockpit,
            "PlanetaryApproachSuite" => ModuleSlot::PlanetaryApproachSuite,
            "WeaponColour" => ModuleSlot::WeaponColour,
            "EngineColour" => ModuleSlot::EngineColour,
            "DataLinkScanner" => ModuleSlot::DataLinkScanner,
            "CodexScanner" => ModuleSlot::CodexScanner,
            "DiscoveryScanner" => ModuleSlot::DiscoveryScanner,
            "ColonisationSuite" => ModuleSlot::ColonisationSuite,
            "PaintJob" => ModuleSlot::PaintJob,
            "Armour" => ModuleSlot::Armour,
            "FuelTank" => ModuleSlot::FuelTank,
            "ShipKitSpoiler" => ModuleSlot::ShipKitSpoiler,
            "ShipKitWings" => ModuleSlot::ShipKitWings,
            "ShipKitTail" => ModuleSlot::ShipKitTail,
            "ShipKitBumper" => ModuleSlot::ShipKitBumper,
            "VesselVoice" => ModuleSlot::VesselVoice,
            _ => {
                warn!("Unknown module slot: {}", value);
                ModuleSlot::Unknown(value)
            }
        }
    }
}