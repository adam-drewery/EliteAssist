use crate::gui::Message;
use crate::state::State;
use crate::theme::ORANGE;
use iced::widget::{column, row, text, Row};
use iced::{Fill, Left, Right};

pub fn header_bar(state: &State) -> Row<'_, Message> {
    row![
        column![
            text(state.commander_name.as_ref()).size(30).color(ORANGE),
            text(state.credits.as_ref()).size(30),
        ]
        .width(Fill)
        .align_x(Left),
        column![
            text(state.current_system.as_ref()).size(30),
            text(state.current_body.as_ref()).size(30),
        ]
        .width(Fill)
        .align_x(Right),
    ]
}
