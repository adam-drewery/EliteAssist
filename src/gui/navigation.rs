use crate::gui::components::header;
use crate::event::JournalEvent;
use crate::state::State;
use iced::widget::{row, text, Column, Row};
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
        header("ROUTE"),
        row!["Legal State: ", text(&state.crime.legal_state)],
    ]
        .padding(8)
}



fn location(state: &State) -> Column<JournalEvent> {
    iced::widget::column![
        header("LOCATION"),
        text(&state.commander_name),
    ]
        .padding(8)
}
