use log::warn;
use regex::Regex;

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
    pub max_jump_range: f64,
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