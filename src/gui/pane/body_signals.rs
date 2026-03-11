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
use std::collections::{HashMap, HashSet};

pub struct BodySignals;

impl BodySignals {
    fn render_node<'a>(
        id: u8,
        depth: u16,
        bodies: &'a HashMap<u8, fss::Body>,
        children_map: &HashMap<Option<u8>, Vec<u8>>,
        rows: &mut Vec<Element<'a, Message>>,
    ) {
        let indentation = (depth * 16) as f32;
        let content: Element<'a, Message> = if let Some(body) = bodies.get(&id) {
            body_details(body).into()
        } else {
            unknown_body_details().into()
        };

        rows.push(
            container(content)
                .padding(iced::padding::left(indentation))
                .into(),
        );

        if let Some(children) = children_map.get(&Some(id)) {
            for &child_id in children {
                Self::render_node(child_id, depth + 1, bodies, children_map, rows);
            }
        }
    }
}

impl pane::Type for BodySignals {
    fn title(&self) -> &'static str {
        "Body Signals"
    }

    fn render<'a>(&self, state: &'a State) -> Element<'a, Message> {
        if let Some(system_scans) = state.system_scans.get(&state.location.system_address)
            && !system_scans.bodies.is_empty()
        {
            let mut all_ids: HashSet<u8> = system_scans.bodies.keys().cloned().collect();
            for body in system_scans.bodies.values() {
                if let Some(p_id) = body.parent_id {
                    all_ids.insert(p_id);
                }
            }

            let mut children_map: HashMap<Option<u8>, Vec<u8>> = HashMap::new();
            for &id in &all_ids {
                let parent_id = if let Some(body) = system_scans.bodies.get(&id) {
                    body.parent_id
                } else {
                    None
                };

                children_map.entry(parent_id).or_default().push(id);
            }

            for children in children_map.values_mut() {
                children.sort();
            }

            let mut rows = Vec::new();
            if let Some(roots) = children_map.get(&None) {
                let mut sorted_roots = roots.clone();
                sorted_roots.sort();
                for &root_id in &sorted_roots {
                    Self::render_node(root_id, 0, &system_scans.bodies, &children_map, &mut rows);
                }
            }

            column![scroll_list(rows)].into()
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

fn unknown_body_details() -> Row<'static, Message> {
    bordered_list_item![container(text("Unknown Body").size(16).color(WHITE))
        .width(Fill)
        .height(Fill)
        .align_x(Center)
        .align_y(Center)]
    .height(48)
}
