mod ship;

use crate::event::JournalEvent;
use crate::gui::commander_details::ship::ship;
use crate::state::State;
use crate::theme::styles::header_style;
use iced::widget::{button, column, row, scrollable, text, Column, Row};
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
        button("LEGAL").style(header_style).width(Fill),
        row!["Legal State: ", text(&state.crime.legal_state)],
    ]
    .padding(8)
}



fn location(state: &State) -> Column<JournalEvent> {
    column![
        button("LOCATION").style(header_style).width(Fill),
        text(&state.commander_name),
    ]
    .padding(8)
}
