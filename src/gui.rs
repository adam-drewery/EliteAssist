mod commander_details;
mod header_bar;
mod materials;
mod navigation_bar;
mod ship_locker;
mod messages;
mod market;

use crate::event::JournalEvent;
use crate::state::{ActiveScreen, State};
use commander_details::commander_details;
use header_bar::header_bar;
use iced::widget::{column, row, text};
use iced::{Bottom, Element, Fill};
use market::market;
use materials::materials;
use messages::messages;
use navigation_bar::navigation_bar;
use ship_locker::ship_locker;

pub struct Gui;

impl Gui {
    pub fn view(state: &State) -> Element<JournalEvent> {
        column![
            header_bar(state),
            match state.active_screen {
                ActiveScreen::Commander => commander_details(state),
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

    pub fn update(state: &mut State, message: JournalEvent) {
        state.update_from(message);
    }
}
