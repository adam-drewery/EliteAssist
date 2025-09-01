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
            row![text(state.location.system_name.as_ref()).size(30)],
            
            if state.location.system_name != state.location.body_name { 
                row![text(state.location.body_name.as_ref()).size(30)] 
            }
            else { row![] },
        ]
        .width(Fill)
        .align_x(Right),
    ]
}
