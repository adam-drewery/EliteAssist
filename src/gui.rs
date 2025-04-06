mod commander_details;
mod header_bar;
mod materials;
mod navigation_bar;
mod ship_locker;
mod messages;
mod market;

use crate::event::Event;
use crate::gui::market::market;
use crate::state::{ActiveScreen, State};
use header_bar::header_bar;
use iced::widget::{column, row, text};
use iced::{Bottom, Element, Fill};
use materials::materials;
use messages::messages;
use navigation_bar::navigation_bar;
use ship_locker::ship_locker;

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<Event> {
        column![
            header_bar(state),
            match state.active_screen {
                ActiveScreen::Commander => row![text("commander shit")],
                ActiveScreen::Navigation => row![text("navigation shit")],
                ActiveScreen::Materials => materials(state),
                ActiveScreen::ShipLocker => ship_locker(state),
                ActiveScreen::Market => market(state),
                ActiveScreen::Messages => messages(state),
            }
            .height(Fill),
            navigation_bar(state).align_y(Bottom),
        ]
        .width(Fill)
        .padding(10)
        .into()
    }

    pub fn update(state: &mut State, message: Event) {
        state.update_from(message);
    }
}
