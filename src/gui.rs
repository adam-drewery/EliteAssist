use crate::State;
use crate::events::EliteEvent;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Font, Left, Right};
use thousands::Separable;

pub struct Gui;

const FONT: Font = Font::with_name("Eurostile");

impl Gui {
    pub fn view(state: &State) -> Element<EliteEvent> {
        
        container(row![
            column![
                text(&state.commander_name).size(30).font(FONT),
                text(&state.credits).size(30).font(FONT),
            ]
            .width(Fill)
            .align_x(Left),
            column![
                text(&state.current_system).size(30).font(FONT),
                text(&state.current_body).size(30).font(FONT),
            ]
            .width(Fill)
            .align_x(Right),
        ])
        .padding(10)
        .center_x(Fill)
        .into()
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        println!("Handling {:?}", message);
        match message {
            EliteEvent::FileHeader(_) => {}
            EliteEvent::Commander(commander) => {
                state.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
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
            EliteEvent::Location(location) => {
                state.current_system = location.star_system;
            }
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
            EliteEvent::Status(status) => {
                state.credits = status.balance.separate_with_commas() + " CR";
                state.current_body = status.body_name
            }
            EliteEvent::None => {}
        }
    }
}
