use crate::events::EliteEvent;
use crate::State;
use iced::widget::{column, text, Column};
use iced::{self};

pub struct MainView;

impl MainView {
    pub fn view(state: &State) -> Column<EliteEvent> {
        column![text(&state.commander.name).size(50),]
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        match message {
            EliteEvent::FileHeader(_) => {}
            EliteEvent::Commander(commander) => {
                state.commander = commander;
            }
            EliteEvent::Materials(_) => {}
            EliteEvent::Rank(_) => {}
            EliteEvent::Progress(_) => {}
            EliteEvent::Reputation(_) => {}
            EliteEvent::EngineerProgress(_) => {}
            EliteEvent::SquadronStartup(_) => {}
            EliteEvent::LoadGame(_) => {}
            EliteEvent::Statistics(_) => {}
            EliteEvent::ReceiveText(_) => {}
            EliteEvent::Location(_) => {}
            EliteEvent::Powerplay(_) => {}
            EliteEvent::Music(_) => {}
            EliteEvent::SuitLoadout(_) => {}
            EliteEvent::Backpack(_) => {}
            EliteEvent::ShipLocker(_) => {}
            EliteEvent::Missions(_) => {}
            EliteEvent::Shutdown(_) => {}
            EliteEvent::Loadout(_) => {}
            EliteEvent::BuyAmmo(_) => {}
            EliteEvent::RestockVehicle(_) => {}
            EliteEvent::BuyMicroResources(_) => {}
            EliteEvent::None => {}
        }
    }
}


