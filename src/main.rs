use crate::events::*;
use crate::subscription::*;

mod journal_poller;
mod events;
mod gui;
mod subscription;

#[tokio::main]
async fn main() {

    iced::application("EliteAssist", gui::Gui::update, gui::Gui::view)
        .subscription(subscription)
        .run()
        .unwrap();
}

#[derive(Default)]
struct State {
    load_game: LoadGame,
    commander: Commander,
    statistics: Statistics,
    file_header: FileHeader,
    materials: Materials,
    rank: Rank,
    progress: Progress,
    reputation: Reputation,
    engineer_progress: EngineerProgress,
    squadron_startup: SquadronStartup,
    receive_text: ReceiveText,
    location: Location,
    powerplay: Powerplay,
    music: Music,
    suit_loadout: SuitLoadout,
    backpack: Backpack,
    ship_locker: ShipLocker,
    missions: Missions,
    shutdown: Shutdown,
    loadout: Loadout,
    buy_ammo: BuyAmmo,
    restock_vehicle: RestockVehicle,
    buy_micro_resources: BuyMicroResources
}

