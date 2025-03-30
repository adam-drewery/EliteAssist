mod backpack;
mod bank_account;
mod buy_ammo;
mod buy_micro_resources;
mod cargo;
mod combat;
mod commander;
mod cqc;
mod crafting;
mod crew;
mod crime;
mod disembark;
mod embark;
mod engineer;
mod engineer_progress;
mod engineering;
mod exobiology;
mod exploration;
mod faction;
mod faction_state;
mod file_header;
mod fuel_capacity;
mod load_game;
mod loadout;
mod location;
mod market;
mod material_trader_stats;
mod materials;
mod mining;
mod missions;
mod modifier;
mod module;
mod multicrew;
mod music;
mod npc_crew_paid_wage;
mod passengers;
mod powerplay;
mod progress;
mod rank;
mod receive_text;
mod reputation;
mod restock_vehicle;
mod search_and_rescue;
mod ship_locker;
mod ship_locker_item;
mod shutdown;
mod smuggling;
mod squadron_startup;
mod station_economy;
mod station_faction;
mod statistics;
mod status;
mod suit_loadout;
mod tg_encounters;
mod trading;

pub use backpack::Backpack;
pub use bank_account::BankAccount;
pub use buy_ammo::BuyAmmo;
pub use buy_micro_resources::BuyMicroResources;
pub use cargo::Cargo;
pub use combat::Combat;
pub use commander::Commander;
pub use crime::Crime;
pub use disembark::Disembark;
pub use embark::Embark;
pub use engineer_progress::EngineerProgress;
pub use exploration::Exploration;
pub use file_header::FileHeader;
pub use load_game::LoadGame;
pub use loadout::Loadout;
pub use location::Location;
pub use market::Market;
pub use materials::Materials;
pub use mining::Mining;
pub use missions::Missions;
pub use music::Music;
pub use npc_crew_paid_wage::NpcCrewPaidWage;
pub use passengers::Passengers;
pub use powerplay::Powerplay;
pub use progress::Progress;
pub use rank::Rank;
pub use receive_text::ReceiveText;
pub use reputation::Reputation;
pub use restock_vehicle::RestockVehicle;
pub use ship_locker::ShipLocker;
pub use ship_locker_item::ShipLockerItem;
pub use shutdown::Shutdown;
pub use smuggling::Smuggling;
pub use squadron_startup::SquadronStartup;
pub use statistics::Statistics;
pub use status::Status;
pub use suit_loadout::SuitLoadout;
pub use trading::Trading;

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

    NavigateTo(ActiveScreen),

    #[default]
    None,

}
