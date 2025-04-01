mod backpack;
mod buy_ammo;
mod buy_micro_resources;
mod cargo;
mod commander;
mod disembark;
mod embark;
mod engineer_progress;
mod file_header;
mod load_game;
mod loadout;
mod location;
mod market;
mod materials;
mod missions;
mod music;
mod npc_crew_paid_wage;
mod powerplay;
mod progress;
mod rank;
mod receive_text;
mod reputation;
mod restock_vehicle;
mod ship_locker;
mod shutdown;
mod squadron_startup;
mod statistics;
mod status;
mod suit_loadout;
mod docked;

pub use backpack::*;
pub use buy_ammo::*;
pub use buy_micro_resources::*;
pub use cargo::*;
pub use commander::*;
pub use disembark::*;
pub use docked::*;
pub use embark::*;
pub use engineer_progress::*;
pub use file_header::*;
pub use load_game::*;
pub use loadout::*;
pub use location::*;
pub use market::*;
pub use materials::*;
pub use missions::*;
pub use music::*;
pub use npc_crew_paid_wage::*;
pub use powerplay::*;
pub use progress::*;
pub use rank::*;
pub use receive_text::*;
pub use reputation::*;
pub use restock_vehicle::*;
pub use ship_locker::*;
pub use shutdown::*;
pub use squadron_startup::*;
pub use statistics::*;
pub use status::*;

use crate::event::suit_loadout::SuitLoadout;
use crate::state::ActiveScreen;
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

    NavigateTo(ActiveScreen),

    #[default]
    None,
}
