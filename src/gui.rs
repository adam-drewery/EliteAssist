use crate::controls::*;
use crate::events::EliteEvent;
use crate::state::{ActiveScreen, State};
use iced::widget::{column, row, text};
use iced::{Bottom, Element, Fill};

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<EliteEvent> {
        column![
            commander_details(state),

            match state.active_screen{
                ActiveScreen::ShipLocker => ship_locker(state),
                ActiveScreen::Commander => row![text("commander shit")],
                ActiveScreen::Navigation => row![text("navigation shit")],
                ActiveScreen::Market => row![text("market shit")],
            }.height(Fill),
            navigation_bar(state).align_y(Bottom),
        ]
        .width(Fill)
        .into()
    }

    pub fn update(state: &mut State, message: EliteEvent) {
        state.update_from(message);
    }
}