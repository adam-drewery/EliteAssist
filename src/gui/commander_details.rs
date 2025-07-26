mod ship;

use crate::gui::components::header;
use crate::event::JournalEvent;
use crate::gui::commander_details::ship::ship;
use crate::state::State;
use iced::widget::{column, row, scrollable, text, Column, Row};
use iced::{Fill, Top};

pub fn commander_details(state: &State) -> Row<JournalEvent> {
    row![
        legal_status(state),
        location(state),
        scrollable(ship(state)),
    ]
    .align_y(Top)
    .height(Fill)
}

fn legal_status(state: &State) -> Column<JournalEvent> {
    column![
        header("LEGAL"),
        row!["Legal State: ", text(&state.crime.legal_state)],
    ]
    .padding(8)
}



fn location(state: &State) -> Column<JournalEvent> {
    column![
        header("LOCATION"),
        text(&state.commander_name),
    ]
    .padding(8)
}
