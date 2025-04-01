mod economy;
mod inventory;
mod navigation;
mod personal;
mod session;

pub use economy::*;
pub use inventory::*;
pub use navigation::*;
pub use personal::*;
pub use session::*;

use crate::state::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(tag = "event")]
pub enum Event {
    #[serde(rename = "Fileheader")]
    FileHeader(FileHeader),
    Commander(Commander),
    Materials(Materials),
    Rank(Rank),
    Progress(Progress),
    Reputation(Reputation),
    EngineerProgress(EngineerProgress),
    SquadronStartup(SquadronStartup),
    LoadGame(LoadGame),
    Statistics(Statistics),
    ReceiveText(ReceiveText),
    Location(Location),
    Powerplay(Powerplay),
    Music(Music),
    SuitLoadout(SuitLoadout),
    Backpack(Backpack),
    ShipLocker(ShipLocker),
    Missions(Missions),
    Shutdown(Shutdown),
    Loadout(Loadout),
    BuyAmmo(BuyAmmo),
    RestockVehicle(RestockVehicle),
    BuyMicroResources(BuyMicroResources),
    Status(Status),
    Embark(Embark),
    Disembark(Disembark),
    NpcCrewPaidWage(NpcCrewPaidWage),
    Cargo(Cargo),
    Market(Market),
    Docked(Docked),
    BookDropship(BookDropship),
    StartJump(StartJump),
    LaunchDrone(LaunchDrone),
    SupercruiseEntry(SupercruiseEntry),
    SupercruiseExit(SupercruiseExit),
    Resurrect(Resurrect),
    FSSSignalDiscovered(FSSSignalDiscovered),
    NavRoute(NavRoute),
    Shipyard(Shipyard),
    ApproachSettlement(ApproachSettlement),
    StoredShips(StoredShips),
    SwitchSuitLoadout(SwitchSuitLoadout),
    MissionAccepted(MissionAccepted),
    FSDTarget(FSDTarget),
    ShipyardSwap(ShipyardSwap),
    ShipyardTransfer(ShipyardTransfer),
    RefuelAll(RefuelAll),
    ClearImpound(ClearImpound),
    ModuleInfo(ModuleInfo),
    Undocked(Undocked),
    CommitCrime(CommitCrime),
    UnderAttack(UnderAttack),
    CollectItems(CollectItems),
    LeaveBody(LeaveBody),
    FSDJump(FSDJump),
    NavRouteClear(NavRouteClear),
    Bounty(Bounty),
    ReservoirReplenished(ReservoirReplenished),
    UseConsumable(UseConsumable),
    Outfitting(Outfitting),
    DockingDenied(DockingDenied),
    MissionFailed(MissionFailed),
    SupercruiseDestinationDrop(SupercruiseDestinationDrop),
    MissionAbandoned(MissionAbandoned),
    EngineerCraft(EngineerCraft),
    DropshipDeploy(DropshipDeploy),
    FuelScoop(FuelScoop),
    ApproachBody(ApproachBody),
    Scan(Scan),
    StoredModules(StoredModules),
    SAASignalsFound(SAASignalsFound),
    DockingRequested(DockingRequested),
    DockingGranted(DockingGranted),

    NavigateTo(ActiveScreen),

    #[default]
    None,
}
