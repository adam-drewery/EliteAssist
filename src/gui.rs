use crate::controls::*;
use crate::events::EliteEvent;
use crate::state::State;
use iced::widget::column;
use iced::{Element, Fill};

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<EliteEvent> {
        column![
            commander_details(state),
            ship_locker(state),
        ]
        .padding(10)
        .width(Fill)
        .into()
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        state.update_from(message);
    }
}
