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

pub use backpack::Backpack;
pub use buy_ammo::BuyAmmo;
pub use buy_micro_resources::BuyMicroResources;
pub use cargo::Cargo;
pub use commander::Commander;
pub use disembark::Disembark;
pub use embark::Embark;
pub use engineer_progress::EngineerProgress;
pub use file_header::FileHeader;
pub use load_game::LoadGame;
pub use loadout::Loadout;
pub use location::Location;
pub use market::Market;
pub use materials::Material;
pub use materials::Materials;
pub use missions::Missions;
pub use music::Music;
pub use npc_crew_paid_wage::NpcCrewPaidWage;
pub use powerplay::Powerplay;
pub use progress::Progress;
pub use rank::Rank;
pub use receive_text::ReceiveText;
pub use reputation::Reputation;
pub use restock_vehicle::RestockVehicle;
pub use ship_locker::ShipLocker;
pub use ship_locker::ShipLockerItem;
pub use shutdown::Shutdown;
pub use squadron_startup::SquadronStartup;
pub use statistics::Statistics;
pub use status::Status;

use crate::state::ActiveScreen;
use serde::Deserialize;
use crate::event::suit_loadout::SuitLoadout;

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

    NavigateTo(ActiveScreen),

    #[default]
    None,
}
