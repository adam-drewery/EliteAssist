mod commander_details;
mod header_bar;
mod locker_item_list;
mod materials;
mod navigation_bar;
mod ship_locker;

use crate::event::Event;
use crate::state::{ActiveScreen, State};
pub use header_bar::header_bar;
use iced::widget::{column, row, text};
use iced::{Bottom, Element, Fill};
pub use locker_item_list::locker_item_list;
pub use materials::materials;
pub use navigation_bar::navigation_bar;
pub use ship_locker::ship_locker;

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
                ActiveScreen::Market => row![text("market shit")],
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
