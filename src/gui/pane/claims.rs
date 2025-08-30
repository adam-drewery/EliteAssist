use iced::{Element, Fill};
use iced::widget::{column, scrollable, Column};
use crate::gui::components::{details, empty_text};
use crate::gui::Message;
use crate::state::State;

pub fn claims(state: &State) -> Column<'_, Message> {

    if (state.bounties.len() == 0) && (state.combat_bonds.len() == 0) {
        return iced::widget::column![
            empty_text("No Claims"),
        ].height(Fill)
    }

    let all_claims = state.bounties.iter().map(|m| {
        details(&m.0, format!["{} CR", &m.1])
    }).chain(
        state.combat_bonds.iter().map(|m| {
            details(&m.0, format!["{} CR", &m.1])
        })
    );

    iced::widget::column![
        scrollable(column(all_claims.map(Element::from)))
    ].height(Fill)
}