use crate::controls::*;
use crate::events::EliteEvent;
use crate::state::State;
use iced::widget::{button, column, row};
use iced::{Element, Fill};

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<EliteEvent> {
        column![
            commander_details(state),
            ship_locker(state),
            row![
                button("SHIP LOCKER").on_press(EliteEvent::ShowShipLocker)
            ]
        ]
        .padding(10)
        .width(Fill)
        .into()
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        state.update_from(message);
    }
}
