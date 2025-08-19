mod location;
mod personal;
mod ship;
mod modules;

use crate::gui::overview::location::*;
use crate::gui::overview::personal::*;
use crate::gui::overview::ship::*;
use crate::gui::overview::modules::*;
use crate::state::State;
use iced::widget::{column, row, Row};
use crate::gui::Message;


pub fn overview(state: &State) -> Row<'_, Message> {
    row![
        column![
            loadout(state),
            //claims(state),
            //missions(state),
            messages(state)
        ],
        column![
            route(state),
            location(state)
        ],
        column![
            ship_details(state),
            ship_modules(state)
        ]
    ]
}