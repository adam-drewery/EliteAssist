use crate::event::{Event, Material};
use crate::image::{COURIER_ICON, GRADE_1, GRADE_2, GRADE_3, GRADE_4, GRADE_5};
use crate::theme::{GRAY, ORANGE, YELLOW};
use iced::widget::svg::Handle;
use iced::widget::{column, row, scrollable, svg, text, Column};
use iced::{Element, Fill, Left};

pub fn materials_list(title: &str, items: Vec<Material>) -> Column<Event> {
    column![
        text(title).size(20).color(ORANGE),
        scrollable(column(
            order_items(items)
                .map(|item| {
                    
                    let svg_handle = match item.info().rarity.as_str() {
                        "Very Common" => Handle::from_memory(GRADE_1),
                        "Common" => Handle::from_memory(GRADE_2),
                        "Standard" => Handle::from_memory(GRADE_3),
                        "Rare" => Handle::from_memory(GRADE_4),
                        "Very Rare" => Handle::from_memory(GRADE_5),
                        _ => Handle::from_memory(COURIER_ICON),
                    };

                    row![
                        column![svg(svg_handle).height(16).width(16)].padding([0, 5]),
                        text(item.count).size(16).color(YELLOW).width(36),
                        text(item.display_name()).size(16),
                        column![text(item.info().category.clone()).size(16).color(GRAY)]
                            .padding([0, 5]),
                    ]
                    .padding(2)
                })
                .map(Element::from)
        ))
        .width(Fill)
    ]
    .align_x(Left)
}

fn order_items(mut items: Vec<Material>) -> impl Iterator<Item = Material> {
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items.into_iter()
}
