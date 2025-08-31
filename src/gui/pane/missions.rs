use crate::gui::components::*;
use crate::gui::Message;
use crate::state::State;
use crate::theme::style;
use iced::widget::{column, scrollable};
use iced::Element;

pub struct MissionsPane;

impl crate::gui::pane::PaneType for MissionsPane {

    fn title(&self) -> &'static str { "Missions" }
    
    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        
        if state.missions.len() == 0 {
            return column![empty_placeholder("No Missions")].into();
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
        .into()
    }
}
