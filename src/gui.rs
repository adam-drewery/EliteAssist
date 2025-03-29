use crate::events::EliteEvent;
use crate::State;
use iced::widget::{column, text, Column};
use thousands::Separable;

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Column<EliteEvent> {
        let commander_name = "CMDR ".to_owned() + &state.commander.name.to_uppercase();
        let credits = state.load_game.credits.separate_with_commas() + " CR";
        
        column![
            
            text(commander_name).size(50),
            text(credits).size(50),
        ]
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        println!("Handling {:?}", message);
        match message {
            EliteEvent::FileHeader(file_header) => { state.file_header = file_header; }
            EliteEvent::Commander(commander) => { state.commander = commander; }
            EliteEvent::Materials(materials) => { state.materials = materials; }
            EliteEvent::Rank(rank) => { state.rank = rank; }
            EliteEvent::Progress(progress) => { state.progress = progress; }
            EliteEvent::Reputation(reputation) => { state.reputation = reputation; }
            EliteEvent::EngineerProgress(engineer_progress) => { state.engineer_progress = engineer_progress; }
            EliteEvent::SquadronStartup(squadron_startup) => { state.squadron_startup = squadron_startup; }
            EliteEvent::LoadGame(load_game) => { state.load_game = load_game; }
            EliteEvent::Statistics(statistics) => { state.statistics = statistics; }
            EliteEvent::ReceiveText(receive_text) => { state.receive_text = receive_text; }
            EliteEvent::Location(location) => { state.location = location; }
            EliteEvent::Powerplay(powerplay) => { state.powerplay = powerplay; }
            EliteEvent::Music(music) => { state.music = music; }
            EliteEvent::SuitLoadout(suit_loadout) => { state.suit_loadout = suit_loadout; }
            EliteEvent::Backpack(backpack) => { state.backpack = backpack; }
            EliteEvent::ShipLocker(ship_locker) => { state.ship_locker = ship_locker; }
            EliteEvent::Missions(missions) => { state.missions = missions; }
            EliteEvent::Shutdown(shutdown) => { state.shutdown = shutdown; }
            EliteEvent::Loadout(loadout) => { state.loadout = loadout; }
            EliteEvent::BuyAmmo(buy_ammo) => { state.buy_ammo = buy_ammo; }
            EliteEvent::RestockVehicle(restock_vehicle) => { state.restock_vehicle = restock_vehicle; }
            EliteEvent::BuyMicroResources(buy_micro_resources) => { state.buy_micro_resources = buy_micro_resources; }
            EliteEvent::None => {}
        }
    }
}


