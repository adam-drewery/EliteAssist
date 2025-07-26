use crate::event::JournalEvent;
use crate::state::State;
use crate::theme::styles::header_style;
use iced::widget::{button, row, text, Column, Row};
use iced::{Fill, Top};

pub fn navigation(state: &State) -> Row<JournalEvent> {
    row![
        route(state),
        location(state)
    ]
    .align_y(Top)
    .height(Fill)
}

fn route(state: &State) -> Column<JournalEvent> {


    iced::widget::column![
        button("ROUTE").style(header_style).width(Fill),
        row!["Legal State: ", text(&state.crime.legal_state)],
    ]
        .padding(8)
}



fn location(state: &State) -> Column<JournalEvent> {
    iced::widget::column![
        button("LOCATION").style(header_style).width(Fill),
        text(&state.commander_name),
    ]
        .padding(8)
}
