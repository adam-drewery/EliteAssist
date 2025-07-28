mod location;
mod personal;
mod ship;

use crate::event::JournalEvent;
use crate::gui::overview::location::*;
use crate::gui::overview::personal::*;
use crate::gui::overview::ship::*;
use crate::state::State;
use iced::widget::{column, row, Row};

pub fn overview(state: &State) -> Row<JournalEvent> {
    row![
        column![
            personal(state),
            inventory(state),
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