use crate::event::JournalEvent;
use crate::image::*;
use crate::state::{MaterialGroup, State};
use crate::theme::*;
use iced::widget::svg::Handle;
use iced::widget::{column, row, scrollable, svg, text, Column, Row};
use iced::{Element, Fill, Top};
use crate::gui::components::sub_header;

pub fn materials(state: &State) -> Row<JournalEvent> {
    
    row![
        materials_list("RAW", &state.materials.raw),
        materials_list("MANUFACTURED", &state.materials.manufactured),
        materials_list("ENCODED", &state.materials.encoded),
    ]
    .align_y(Top)
    .height(Fill)
}

fn materials_list<'a>(title: &'a str, groups: &'a [MaterialGroup]) -> Column<'a, JournalEvent> {
    iced::widget::column![
        sub_header(title),
        scrollable(
            column(
                groups.iter().flat_map(|group| {

                    let mut rows = vec![row![text(&group.name).size(16).color(GRAY)].padding(2)];
                    
                    let mut sorted_materials = group.materials.to_vec();
                    sorted_materials.sort_by_key(|item| item.rarity);

                    rows.extend(sorted_materials.into_iter().map(|item| {
                        let svg_handle = match item.rarity {
                            1 => Handle::from_memory(GRADE_1),
                            2 => Handle::from_memory(GRADE_2),
                            3 => Handle::from_memory(GRADE_3),
                            4 => Handle::from_memory(GRADE_4),
                            5 => Handle::from_memory(GRADE_5),
                            _ => Handle::from_memory(COURIER_ICON),
                        };

                        row![
                            column![svg(svg_handle).height(16).width(16)].padding([0, 5]),
                            text(item.count.to_string()).size(16).color(YELLOW).width(36),
                            text(item.name).size(16),
                        ]
                        .padding(2)
                    }));

                    rows
                })
                .map(Element::from)
            )
        )
        .width(Fill)
    ]
}