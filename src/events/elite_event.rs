use crate::events::*;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
#[serde(tag = "event")]
pub enum EliteEvent {

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

    #[default]
    None
}
