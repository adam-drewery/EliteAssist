use crate::font::EURO_CAPS;
use crate::gui::Message;
use crate::state::State;
use crate::theme::ORANGE;
use iced::widget::{column, row, text, Row};
use iced::{Fill, Left, Right};

pub fn header_bar(state: &State) -> Row<'_, Message> {
    row![
        column![
            text(&state.commander_name).font(EURO_CAPS).size(30).color(ORANGE),
            text(&state.credits).size(30),
        ]
        .width(Fill)
        .align_x(Left),
        column![
            text(&state.current_system).font(EURO_CAPS).size(30),
            text(&state.current_body).font(EURO_CAPS).size(30),
        ]
        .width(Fill)
        .align_x(Right),
    ]
}
