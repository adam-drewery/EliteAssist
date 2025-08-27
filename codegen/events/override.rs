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

static STRUCT_NAME_OVERRIDES: OnceLock<HashMap<Vec<&'static str>, &'static str>> = OnceLock::new();

pub static FIELD_TYPES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "Rank.cqc" => "u8",
    "Rank.combat" => "u8",
    "Rank.empire" => "u8",
    "Rank.exobiologist" => "u8",
    "Rank.explore" => "u8",
    "Rank.federation" => "u8",
    "Rank.soldier" => "u8",
    "Rank.trade" => "u8",
};

pub fn struct_names() -> &'static HashMap<Vec<&'static str>, &'static str> {
    STRUCT_NAME_OVERRIDES.get_or_init(|| {
        struct_name_map! {
            ["JoinACrew", "QuitACrew"] => "Crew",
            ["FighterDestroyed", "HeatDamage"] => "Damage",
            ["BuySuit", "SellSuit"] => "Suit",
            ["PayBounties", "PayFines"] => "Payment",
            ["Backpack", "ShipLocker"] => "Inventory",
            ["CockpitBreached", "ColonisationBeaconDeployed", "HeatWarning", "Resupply", "SelfDestruct", "Shutdown", "SystemsShutdown", "WingLeave"] => "Empty",
            ["PowerplayCollect", "PowerplayDeliver"] => "PowerplayDelivery",
            ["Liftoff", "Touchdown"] => "Liftoff",
            ["LoadoutEquipModule", "LoadoutRemoveModule"] => "SuitLoadoutChange",
            ["CarrierModulePack", "CarrierShipPack"] => "CarrierPack",
            ["CarrierJumpFactionPendingState", "CarrierJumpFactionRecoveringState", "FSDJumpFactionPendingState", "FSDJumpFactionRecoveringState", "LocationFactionPendingState", "LocationFactionRecoveringState"] => "FactionState",
            ["DataScanned", "MiningRefined"] => "TypeStub",
            ["ClearSavedGame", "Commander"] => "Commander",
            ["PowerplayJoin", "PowerplayLeave"] => "PowerplayMembership",
            ["CreateSuitLoadout", "SuitLoadout", "SwitchSuitLoadout"] => "SuitLoadout",
            ["CarrierStatsModulePack", "CarrierStatsShipPack"] => "CarrierStatsPack",
            ["BuyExplorationData", "BuyTradeData"] => "BuyData",
            ["MissionAbandoned", "MissionFailed"] => "MissionFailed",
            ["DockSRV", "SRVDestroyed"] => "SRVEvent",
            ["BackpackComponent", "BackpackData", "BackpackItem", "ShipLockerComponent", "ShipLockerData", "ShipLockerItem", "ShipLockerMaterialsComponent", "ShipLockerMaterialsData", "ShipLockerMaterialsItem"] => "InventoryItem",
            ["CrewMemberJoins", "CrewMemberQuits"] => "CrewMember",
            ["CommunityGoalDiscard", "CommunityGoalJoin"] => "CommunityGoalChange",
            ["StatusFuel"] => "Fuel",
            ["ReservoirReplenished"] => "ReservoirReplenished",
            ["MaterialTradePaid", "MaterialTradeReceived"] => "MaterialTrade",
            ["RefuelAll", "RefuelPartial"] => "Refuel",
            ["MaterialCollected", "MaterialDiscarded"] => "InventoryMaterial",
            ["ApproachSettlementStationFaction", "CarrierJumpSystemFaction", "DockedStationFaction", "FSDJumpSystemFaction", "LocationStationFaction", "LocationSystemFaction"] => "Faction",
            ["BuyMicroResourcesMicroResource", "DeliverPowerMicroResourcesMicroResource", "RequestPowerMicroResourcesMicroResource", "SellMicroResourcesMicroResource", "TechnologyBrokerMaterial", "TradeMicroResourcesOffered"] => "MicroResourceTrade",
            ["BuyAmmo", "RepairAll"] => "ShipMaintenance",
            ["BuyWeapon", "SellWeapon"] => "WeaponTrade",
            ["ApproachBody", "LeaveBody"] => "Body",
            ["CarrierStatsModulePack", "CarrierStatsShipPack"] => "CarrierStatsPack",
            ["BookDropship", "BookTaxi"] => "BookTransport",
            ["CancelDropship", "CancelTaxi"] => "CancelTransport",
            ["ApproachSettlementStationEconomy", "CarrierJumpStationEconomy", "DockedStationEconomy", "LocationStationEconomy", "ProspectedAsteroidMaterial"] => "NameAndProportion",
            ["ColonisationSystemClaim", "ColonisationSystemClaimRelease"] => "ColonisationSystemClaim",
            ["ShipRedeemed", "ShipyardNew"] => "ShipyardNew",
            ["Progress", "Rank"] => "Rank",
            ["CarrierJumpFactionPendingState", "CarrierJumpFactionRecoveringState", "FSDJumpFactionPendingState", "FSDJumpFactionRecoveringState", "LocationFactionPendingState", "LocationFactionRecoveringState"] => "FactionRecoveringState",
            ["DeliverPowerMicroResources", "RequestPowerMicroResources"] => "MicroResourceDelivery",
            ["MissionsActive", "MissionsComplete", "MissionsFailed"] => "Mission",
            ["BackpackComponent", "BackpackData", "BackpackItem", "ShipLockerComponent", "ShipLockerData", "ShipLockerItem", "ShipLockerMaterialsComponent", "ShipLockerMaterialsData", "ShipLockerMaterialsItem"] => "MicroResource",
            ["EngineerCraftIngredient", "MaterialsEncoded", "MaterialsManufactured", "MaterialsRaw", "MissionCompletedCommodityReward", "SynthesisMaterial", "TechnologyBrokerCommodity", "UpgradeSuitResource", "UpgradeWeaponResource"] => "Material",
            ["CarrierJumpStationFaction", "WingAdd", "WingInvite"] => "NameStub",
            ["DeleteSuitLoadout", "RenameSuitLoadout"] => "SuitLoadoutStub",
            ["SquadronDemotion", "SquadronPromotion"] => "SquadronPromotion",
            ["CarrierCancelDecommission", "CarrierJumpCancelled"] => "CarrierStub",
            ["AppliedToSquadron", "DisbandedSquadron", "InvitedToSquadron", "JoinedSquadron", "KickedFromSquadron", "LeftSquadron", "SharedBookmarkToSquadron", "SquadronCreated"] => "Squadron",
            ["BackpackChangeAdded", "BackpackChangeRemoved"] => "BackpackChange",
            ["BackpackConsumable", "ShipLockerConsumable", "ShipLockerMaterialsConsumable"] => "Consumable",
            ["CarrierJumpThargoidWar", "FSDJumpThargoidWar", "LocationThargoidWar"] => "ThargoidWar",
            ["CreateSuitLoadoutModule", "SuitLoadoutModule", "SwitchSuitLoadoutModule"] => "SuitLoadoutModule",
            ["CarrierJumpFactionActiveState", "FSDJumpFactionActiveState", "LocationFactionActiveState"] => "FactionActiveState",
            ["CarrierJumpFaction", "FSDJumpFaction", "LocationFaction"] => "LocationFaction",
            ["CarrierJumpConflict", "LocationConflict"] => "LocationConflict",
            ["DockedLandingPad", "DockingRequestedLandingPad"] => "LandingPad",
            ["CarrierJumpPowerplayConflictProgress", "FSDJumpPowerplayConflictProgress", "LocationPowerplayConflictProgress"] => "ConflictProgress",
            ["FSSBodySignalsSignal", "SAASignalsFoundSignal"] => "Signal",
            ["CarrierJumpConflictFaction1", "FSDJumpConflictFaction1", "FSDJumpConflictFaction2", "LocationConflictFaction1"] => "ConflictFaction1",
            ["CarrierJumpConflictFaction2", "LocationConflictFaction2"] => "ConflictFaction2"
        }
    })
}