use crate::events::EliteEvent;
use crate::State;
use iced::widget::{column, text, Column};
use thousands::Separable;

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Column<EliteEvent> {

        column![
            
            text(&state.commander_name).size(50),
            text(&state.credits).size(50),
        ]
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        println!("Handling {:?}", message);
        match message {
            EliteEvent::FileHeader(_) => { }
            EliteEvent::Commander(commander) => {
                state.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
            }
            EliteEvent::Materials(_) => { }
            EliteEvent::Rank(_) => { }
            EliteEvent::Progress(_) => { }
            EliteEvent::Reputation(_) => { }
            EliteEvent::EngineerProgress(_) => { }
            EliteEvent::SquadronStartup(_) => { }
            EliteEvent::LoadGame(_) => {
            }
            EliteEvent::Statistics(_) => { }
            EliteEvent::ReceiveText(_) => {  }
            EliteEvent::Location(_) => { }
            EliteEvent::Powerplay(_) => { }
            EliteEvent::Music(_) => { }
            EliteEvent::SuitLoadout(_) => { }
            EliteEvent::Backpack(_) => { }
            EliteEvent::ShipLocker(_) => { }
            EliteEvent::Missions(_) => { }
            EliteEvent::Shutdown(_) => { }
            EliteEvent::Loadout(_) => { }
            EliteEvent::BuyAmmo(_) => { }
            EliteEvent::RestockVehicle(_) => { }
            EliteEvent::BuyMicroResources(_) => { }
            EliteEvent::Status(status) => {
                state.credits = status.balance.separate_with_commas() + " CR";
            }
            EliteEvent::None => {}
        }
    }
}


