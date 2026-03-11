use crate::bordered_list_item;
use crate::gui::components::{empty_placeholder, scroll_list};
use crate::gui::{pane, Message};
use crate::state::fss;
use crate::state::State;
use crate::theme::{style, ORANGE, WHITE};
use iced::widget::{column, container, progress_bar, row, svg, text, Row};
use iced::{Element, Fill, Right};

pub struct SystemScanner;

impl SystemScanner {
    fn signal_details(signal: &fss::Signal) -> Row<'_, Message> {
        let icon = signal.get_icon();

        let left_side = column![
            row![
                text(signal.name.to_string()).size(16).color(WHITE),
                if let Some(threat) = signal.threat_level {
                    text(format!("  Threat {}", threat)).size(14).color(ORANGE)
                } else {
                    text("")
                }
            ],
            row![
                text(signal.kind.as_deref().unwrap_or("Unknown")).size(14).color(ORANGE),
                if let Some(uss) = &signal.uss_type {
                    text(format!("  {}", uss)).size(14).color(WHITE)
                } else {
                    text("")
                },
                if let Some(state) = &signal.spawning_state {
                    text(format!("  ({})", state)).size(14).color(WHITE)
                } else {
                    text("")
                },
            ].spacing(4)
        ]
        .width(Fill)
        .padding([0, 6]);

        let right_side = column![
            if let Some(faction) = &signal.spawning_faction {
                text(faction.to_string()).size(12).color(WHITE)
            } else {
                text("")
            },
            if let Some(time) = signal.time_remaining {
                text(format_time(time)).size(12).color(ORANGE)
            } else if signal.is_station {
                text("STATION").size(12).color(ORANGE)
            } else {
                text("")
            }
        ]
        .align_x(Right)
        .padding([4, 10]);

        bordered_list_item![
            if let Some(data) = icon {
                column![svg(svg::Handle::from_memory(data))
                    .width(32)
                    .height(32)
                    .style(style::planet_icon)]
                .padding([4, 0])
            } else {
                column![].width(0)
            },
            left_side,
            right_side
        ]
        .height(48)
    }


}

impl pane::Type for SystemScanner {
    fn title(&self) -> &'static str { "System Scanner" }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        if let Some(system_scans) = state.system_scans.get(&state.location.system_address) {
            let mut rows: Vec<Element<'a, Message>> = Vec::new();

            if let Some(progress) = &system_scans.progress {
                rows.push(
                    row![
                        container(column![
                            row![text("Discovery").size(16).color(ORANGE)].padding(4),
                            row![
                                progress_bar(0f32..=1f32, progress.progress as f32)
                                    .girth(8)
                                    .style(style::progress_bar)
                            ]
                            .padding(4),
                            row![
                                text(format!(
                                    "System: {}  Bodies: {}  Non-Bodies: {}",
                                    state.location.system_name, progress.body_count, progress.non_body_count
                                ))
                                .size(14)
                                .color(WHITE)
                            ]
                            .padding(4)
                        ])
                        .style(style::bordered)
                        .padding(4)
                        .width(Fill)
                    ]
                    .padding([4, 8])
                    .into()
                );
            }

            let mut bodies: Vec<_> = system_scans.bodies.values().collect();
            bodies.sort_by_key(|b| b.id);

            for s in system_scans.signals.iter() {
                rows.push(Self::signal_details(s).into());
            }

            column![scroll_list(rows)].into()
        }
        else {
            empty_placeholder("No signals").into()
        }
    }
}

fn format_time(seconds: f64) -> String {
    let minutes = (seconds / 60.0).floor();
    let seconds = (seconds % 60.0).floor();
    format!("{:02.0}:{:02.0}", minutes, seconds)
}
