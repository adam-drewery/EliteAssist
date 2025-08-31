use crate::gui::components::{details, empty_placeholder};
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::style;
use iced::widget::{column, scrollable};
use iced::{Element, Fill};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Claims;

impl pane::Type for Claims {

    fn title(&self) -> &'static str { "Claims" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        if (state.bounties.len() == 0) && (state.combat_bonds.len() == 0) {
            return iced::widget::column![empty_placeholder("No Claims")]
                .height(Fill)
                .into();
        }

        let all_claims = state
            .bounties
            .iter()
            .map(|m| details(&m.0, format!["{} CR", &m.1]))
            .chain(
                state
                    .combat_bonds
                    .iter()
                    .map(|m| details(&m.0, format!["{} CR", &m.1])),
            );

        iced::widget::column![
            scrollable(column(all_claims.map(Element::from))).style(style::scrollable)
        ]
        .height(Fill)
        .into()
    }
}
