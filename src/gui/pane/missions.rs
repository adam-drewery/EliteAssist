use crate::gui::components::*;
use crate::gui::Message;
use crate::state::State;
use crate::theme::style;
use iced::widget::{column, scrollable, Column};
use iced::Element;

pub fn missions(state: &State) -> Column<'_, Message> {
    if state.missions.len() == 0 {
        return column![empty_placeholder("No Missions")];
    }

    column![
        scrollable(column(
            state
                .missions
                .iter()
                .map(|m| { column![details(&m.faction, m.name.as_ref())] })
                .map(Element::from)
        ))
        .style(style::scrollable)
    ]
}
