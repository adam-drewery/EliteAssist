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
        if let Some(d) = &state.fss.discovery {
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
                            d.system_name, d.body_count, d.non_body_count
                        ))
                        .size(14)
                        .color(WHITE)
                    ]
                    .padding(4)
                ])
                .style(style::bordered)
            ).padding(4);
        }

        // All bodies found
        if let Some(abf) = &state.fss.all_bodies_found {
            content = content.push(
                container(column![
                    row![text("All Bodies Found").size(16).color(ORANGE)].padding(4),
                    row![text(format!("{} bodies in {}", abf.count, abf.system_name)).size(14).color(WHITE)].padding(4)
                ])
                .style(style::bordered)
            ).padding(4);
        }

        // Last scan summary
        if let Some(scan) = &state.fss.last_scan {
            content = content.push(
                container(column![
                    row![text("Last Scan").size(16).color(ORANGE)].padding(4),
                    row![text(format!("{} ({})", scan.body_name, scan.body_id)).size(14).color(WHITE)].padding(4),
                    if let Some(ts) = &scan.terraform_state { row![text(format!("Terraform: {}", ts)).size(14).color(GRAY)].padding(4) } else { row![] },
                    row![text(format!(
                        "Discovered: {}  Mapped: {}",
                        scan.was_discovered,
                        scan.was_mapped
                    ))
                    .size(14)
                    .color(GRAY)].padding(4)
                ])
                .style(style::bordered)
            ).padding(4);
        }

        // Body signals
        if !state.fss.body_signals.is_empty() {
            let mut items = column![];
            for (_id, bs) in state.fss.body_signals.iter() {
                let mut sigs = column![];
                for s in bs.signals.iter() {
                    sigs = sigs.push(row![text(format!("{} x{}", s.kind, s.count)).size(14).color(WHITE)]);
                }
                items = items.push(
                    container(column![
                        row![text(format!("Signals on {}", bs.body_name)).size(16).color(ORANGE)].padding(4),
                        sigs.padding(4)
                    ]).style(style::bordered)
                ).padding(4);
            }
            content = content.push(items);
        }

        // System signals
        if !state.fss.system_signals.is_empty() {
            let mut items = column![];
            for s in state.fss.system_signals.iter() {
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
