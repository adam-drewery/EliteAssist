use crate::bordered_list_item;
use crate::gui::components::{empty_placeholder, scroll_list};
use crate::gui::pane;
use crate::message::Message;
use crate::state;
use crate::state::State;
use crate::theme::{style, ORANGE, WHITE};
use iced::widget::{column, container, row, text, Row};
use iced::{Element, Fill};

pub struct BodySignals;

impl pane::Type for BodySignals {
    fn title(&self) -> &'static str {
        "Body Signals"
    }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        if let Some(system_scans) = state.system_scans.get(&state.location.system_address)
            && !system_scans.bodies.is_empty() {
                column![scroll_list(
                    system_scans
                        .bodies
                        .iter()
                        .map(|body| body_details(body.1))
                        .collect()
                )]
                .into()
        } else {
            empty_placeholder("No signals found").into()
        }
    }
}

fn body_details(body: &state::fss::Body) -> Row<'_, Message> {
    bordered_list_item![column![
        row![column![text(body.name.to_string()).size(16).color(WHITE)].padding([0, 6]),],
        row![
            column![
                text(body.r#type.clone().unwrap_or_default().to_string())
                    .size(16)
                    .color(ORANGE)
            ]
            .padding([0, 6])
        ]
    ]]
    .height(48)
}
