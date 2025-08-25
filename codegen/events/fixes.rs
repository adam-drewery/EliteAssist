use std::collections::HashMap;
use std::sync::OnceLock;

macro_rules! struct_name_map {
    ( $( [$($key:expr),*] => $value:expr ),* ) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert(vec![$($key),*], $value);
            )*
            map
        }
    };
}

static STRUCT_NAME_MERGES: OnceLock<HashMap<Vec<&'static str>, &'static str>> = OnceLock::new();

pub fn struct_name_merges() -> &'static HashMap<Vec<&'static str>, &'static str> {
    STRUCT_NAME_MERGES.get_or_init(|| {
        struct_name_map! {
            ["JoinACrew", "QuitACrew"] => "Crew",
            ["FighterDestroyed", "HeatDamage"] => "Damage",
            ["BuySuit", "SellSuit"] => "Suit",
            ["PayBounties", "PayFines"] => "Payment",
            ["Backpack", "ShipLocker"] => "Inventory",
            ["CockpitBreached", "ColonisationBeaconDeployed", "HeatWarning", "Resupply", "SelfDestruct", "Shutdown", "SystemsShutdown", "WingLeave"] => "Empty",
            ["PowerplayCollect", "PowerplayDeliver"] => "PowerplayDelivery",
            ["Liftoff", "Touchdown"] => "Liftoff",
            ["LoadoutEquipModule", "LoadoutRemoveModule"] => "LoadoutModule",
            ["CarrierModulePack", "CarrierShipPack"] => "CarrierPack",
            ["CarrierJumpFactionPendingState", "CarrierJumpFactionRecoveringState", "FSDJumpFactionPendingState", "FSDJumpFactionRecoveringState", "LocationFactionPendingState", "LocationFactionRecoveringState"] => "FactionState",
            ["DataScanned", "MiningRefined"] => "TypeStub",
            ["ClearSavedGame", "Commander"] => "Commander",
            ["PowerplayJoin", "PowerplayLeave"] => "Powerplay",
            ["CreateSuitLoadout", "SuitLoadout", "SwitchSuitLoadout"] => "SuitLoadout",
            ["CarrierStatsModulePack", "CarrierStatsShipPack"] => "CarrierStatsPack",
            ["BuyExplorationData", "BuyTradeData"] => "BuyData",
            ["MissionAbandoned", "MissionFailed"] => "MissionFailed",
            ["DockSRV", "SRVDestroyed"] => "SRVEvent",
            ["BackpackComponent", "BackpackData", "BackpackItem", "ShipLockerComponent", "ShipLockerData", "ShipLockerItem", "ShipLockerMaterialsComponent", "ShipLockerMaterialsData", "ShipLockerMaterialsItem"] => "InventoryItem"
        }
    })
}