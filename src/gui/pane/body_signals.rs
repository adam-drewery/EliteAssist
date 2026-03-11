use crate::bordered_list_item;
use crate::gui::components::{empty_placeholder, scroll_list};
use crate::gui::pane;
use crate::message::Message;
use crate::state::fss;
use crate::state::State;
use crate::theme::{style, ORANGE, WHITE};
use iced::widget::tooltip::Position;
use iced::widget::{column, container, row, svg, text, tooltip, Row};
use iced::{Center, Element, Fill};

pub struct BodySignals;

impl pane::Type for BodySignals {
    fn title(&self) -> &'static str {
        "Body Signals"
    }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        if let Some(system_scans) = state.system_scans.get(&state.location.system_address)
            && !system_scans.bodies.is_empty() {
                let mut bodies: Vec<_> = system_scans.bodies.iter().collect();
                bodies.sort_by_key(|b| b.0);

                column![scroll_list(
                    bodies
                        .into_iter()
                        .map(|body| body_details(body.1))
                        .collect()
                )]
                .into()
        } else {
            empty_placeholder("No signals found").into()
        }
    }
}

fn body_details(body: &fss::Body) -> Row<'_, Message> {
    bordered_list_item![
        column![
            row![
                text(body.name.to_string()).size(16).color(WHITE),
                text(format!("  {:.0}ls", body.distance_ls))
                    .size(14)
                    .color(ORANGE),
            ],
            row![
                text(body.r#type.as_deref().unwrap_or_default())
                    .size(14)
                    .color(ORANGE),
                row(body
                    .signals
                    .iter()
                    .map(|sig| {
                        text(format!("{}: {}", sig.kind, sig.count))
                            .size(12)
                            .color(WHITE)
                            .into()
                    })
                    .collect::<Vec<_>>())
                .spacing(8)
                .padding([0, 10])
            ]
        ]
        .width(Fill)
        .padding([0, 6]),
        row(body
            .icons()
            .into_iter()
            .map(|icon| {
                tooltip(
                    svg(svg::Handle::from_memory(icon.data))
                        .style(style::planet_icon)
                        .width(32)
                        .height(32),
                    container(text(icon.tooltip).size(14)).padding(4),
                    Position::Top,
                )
                .style(style::tooltip)
                .into()
            })
            .collect::<Vec<_>>())
        .spacing(4)
        .align_y(Center)
        .padding([4, 10])
    ]
    .height(48)
}
