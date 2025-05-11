mod ship;

use crate::event::JournalEvent;
use crate::gui::commander_details::ship::ship;
use crate::state::State;
use crate::theme::styles::header_style;
use crate::theme::{GRAY, RED};
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
        if state.crime.active_fine {
            text("Active Fine").color(RED).width(Fill)
        } else {
            text("No Active Fine").color(GRAY).width(Fill)
        },
        if state.crime.wanted {
            text("Wanted").color(RED).width(Fill)
        } else {
            text("Not Wanted").color(GRAY).width(Fill)
        },
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
