use iced::{Element, Fill};
use iced::widget::{column, container, progress_bar, row, scrollable, text};
use crate::gui::{pane, Message};
use crate::state::State;
use crate::theme::{style, GRAY, ORANGE, WHITE};

pub struct SystemScanner;

impl pane::Type for SystemScanner {
    fn title(&self) -> &'static str { "System Scanner" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        let mut content = column![];

        // Discovery progress
        if let Some(d) = &state.fss.progress {
            content = content.push(
                container(column![
                    row![text("Discovery").size(16).color(ORANGE)].padding(4),
                    row![
                        progress_bar(0f32..=1f32, d.progress)
                            .height(8)
                            .style(style::progress_bar)
                    ]
                    .padding(4),
                    row![
                        text(format!(
                            "System: {}  Bodies: {}  Non-Bodies: {}",
                            state.location.system_name, d.body_count, d.non_body_count
                        ))
                        .size(14)
                        .color(WHITE)
                    ]
                    .padding(4)
                ])
                .style(style::bordered)
            ).padding(4);
        }

        // System signals
        if !state.fss.signals.is_empty() {
            let mut items = column![];
            for s in state.fss.signals.iter() {
                let kind = s.kind.as_deref().unwrap_or("Unknown");
                items = items.push(
                    row![text(format!(
                        "{} ({}){}",
                        s.name,
                        kind,
                        if s.is_station { " [Station]" } else { "" }
                    ))
                    .size(14)
                    .color(WHITE)]
                );
            }
            content = content.push(
                container(column![
                    row![text("Signals in System").size(16).color(ORANGE)].padding(4),
                    items.padding(4)
                ]).style(style::bordered)
            ).padding(4);
        }

        column![scrollable(content).style(style::scrollable)]
            .width(Fill)
            .height(Fill)
            .into()
    }
}
