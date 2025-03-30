use crate::control::*;
use crate::event::Event;
use crate::state::{ActiveScreen, State};
use iced::widget::{column, row, text};
use iced::{Bottom, Element, Fill};

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<Event> {
        column![
            commander_details(state),

            match state.active_screen{
                
                ActiveScreen::Commander => row![text("commander shit")],
                ActiveScreen::Navigation => row![text("navigation shit")],
                ActiveScreen::Materials => materials(state),
                ActiveScreen::ShipLocker => ship_locker(state),
                ActiveScreen::Market => row![text("market shit")],
            }.height(Fill),
            navigation_bar(state).align_y(Bottom),
        ]
        .width(Fill)
        .into()
    }

    pub fn update(state: &mut State, message: Event) {
        state.update_from(message);
    }
}