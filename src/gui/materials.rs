use crate::event::JournalEvent;
use crate::gui::components::sub_header;
use crate::image::*;
use crate::state::{MaterialGroup, State};
use crate::theme::*;
use iced::widget::svg::Handle;
use iced::widget::{column, row, scrollable, svg, text, Column, Row};
use iced::{Element, Fill, Top};
use crate::gui::Message;

pub fn materials(state: &State) -> Row<Message> {
    row![
        materials_list("Raw", &state.materials.raw),
        materials_list("Manufactured", &state.materials.manufactured),
        materials_list("Encoded", &state.materials.encoded),
    ]
    .align_y(Top)
    .height(Fill)
}

fn materials_list<'a>(title: &'a str, groups: &'a [MaterialGroup]) -> Column<'a, Message> {
    iced::widget::column![
        sub_header(title),
        scrollable(column(
            groups
                .iter()
                .flat_map(|group| {
                    let mut rows = vec![row![text(&group.name).size(16).color(GRAY)].padding(2)];

                    let mut sorted_materials = group.materials.to_vec();
                    sorted_materials.sort_by_key(|item| item.rarity);

                    rows.extend(sorted_materials.into_iter().map(|item| {
                        let svg_handle = match item.rarity {
                            1 => Handle::from_memory(GRADE_1_SVG),
                            2 => Handle::from_memory(GRADE_2_SVG),
                            3 => Handle::from_memory(GRADE_3_SVG),
                            4 => Handle::from_memory(GRADE_4_SVG),
                            5 => Handle::from_memory(GRADE_5_SVG),
                            _ => Handle::from_memory(COURIER_ICON),
                        };

                        row![
                            column![svg(svg_handle).height(16).width(16)].padding([0, 5]),
                            text(item.count.to_string())
                                .size(16)
                                .color(YELLOW)
                                .width(36),
                            text(item.name).size(16),
                        ]
                        .padding(2)
                    }));

                    rows
                })
                .map(Element::from)
        ))
        .width(Fill)
    ]
}
