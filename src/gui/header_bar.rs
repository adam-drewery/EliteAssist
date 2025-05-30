use crate::event::JournalEvent;
use crate::state::State;
use crate::theme::ORANGE;
use iced::widget::{column, row, text, Row};
use iced::{Fill, Left, Right};

pub fn header_bar(state: &State) -> Row<JournalEvent> {
    row![
        column![
            text(&state.commander_name).size(30).color(ORANGE),
            text(&state.credits).size(30),
        ]
        .width(Fill)
        .align_x(Left),
        column![
            text(&state.current_system).size(30),
            text(&state.current_body).size(30),
        ]
        .width(Fill)
        .align_x(Right),
    ]
}
