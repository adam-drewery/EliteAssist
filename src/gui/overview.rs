mod location;
mod personal;
mod ship;

use crate::gui::overview::location::*;
use crate::gui::overview::personal::*;
use crate::gui::overview::ship::*;
use crate::state::State;
use iced::widget::{column, row, Row};
use crate::gui::Message;

pub fn overview(state: &State) -> Row<Message> {
    row![
        column![
            personal(state),
            claims(state),
            missions(state),
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