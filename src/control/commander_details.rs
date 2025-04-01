use crate::event::Event;
use crate::state::State;
use iced::widget::{column, row, Row};
use iced::{Fill, Top};

pub fn commander_details(state: &State) -> Row<Event> {

    row![
        column![
        legal_status(state),
        missions(state),
        messages(state),
        ],
    ]
    .align_y(Top)
    .height(Fill)
}

fn legal_status(state: &State) -> Row<Event> {
    row![ state.commander_name.as_ref() ]
}

fn missions(state: &State) -> Row<Event> {
    row![ state.commander_name.as_ref() ]
}

fn messages(state: &State) -> Row<Event> {
    row![ state.commander_name.as_ref() ]
}