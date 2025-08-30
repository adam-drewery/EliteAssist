use crate::gui::components::sub_header;
use crate::gui::Message;
use crate::image::engineering::*;
use crate::state::{MaterialGroup, State};
use crate::theme::*;
use iced::widget::svg::Handle;
use iced::widget::tooltip::Position;
use iced::widget::{column, row, scrollable, svg, text, tooltip, Column};
use iced::Fill;
use crate::font::EUROSTILE;

// Materials pane: three columns of materials lists (Raw, Manufactured, Encoded)
pub fn materials(state: &State) -> Column<'_, Message> {
    column![
        row![
            materials_list("Raw", &state.materials.raw),
            materials_list("Manufactured", &state.materials.manufactured),
            materials_list("Encoded", &state.materials.encoded),
        ]
        .height(Fill)
    ]
}

fn materials_list<'a>(title: &'a str, groups: &'a [MaterialGroup]) -> Column<'a, Message> {
    column![
        sub_header(title),
        scrollable(column(
            groups
                .iter()
                .flat_map(|group| {
                    let mut rows = vec![row![text(group.name.as_ref()).size(16).color(GRAY)].padding(2)];

                    let mut sorted_materials = group.materials.to_vec();
                    sorted_materials.sort_by_key(|item| item.rarity);

                    rows.extend(sorted_materials.into_iter().map(|item| {
                        let svg_handle = match item.rarity {
                            1 => Handle::from_memory(GRADE_1_SVG),
                            2 => Handle::from_memory(GRADE_2_SVG),
                            3 => Handle::from_memory(GRADE_3_SVG),
                            4 => Handle::from_memory(GRADE_4_SVG),
                            5 => Handle::from_memory(GRADE_5_SVG),
                            _ => panic!("Invalid rarity: {}", item.rarity),
                        };

                        row![
                            tooltip(
                                row![
                                    column![svg(svg_handle).height(16).width(16)].padding([0, 5]),
                                    text(item.count.to_string())
                                        .size(16)
                                        .color(YELLOW)
                                        .font(EUROSTILE)
                                        .width(36),
                                    text(item.name.to_string())
                                        .font(EUROSTILE)
                                        .size(16),
                                ]
                                .padding(2),
                                column(
                                    item.locations
                                        .iter()
                                        .map(|loc| row![text(loc.to_string()).size(16).font(EUROSTILE)].into())
                                        .collect::<Vec<iced::Element<Message>>>()
                            ),
                            Position::FollowCursor
                            )
                        .style(style::tooltip)
                        ]
                    }));

                    rows
                })
                .map(iced::Element::from)
        ))
        .style(style::scrollable)
        .width(Fill)
    ]
}
