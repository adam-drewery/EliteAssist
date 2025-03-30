use crate::event::{Event, Material};
use crate::image::GRADE_1;
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
                    row![
                        column![svg(Handle::from_memory(GRADE_1)).height(16).width(16)].padding([0, 5]),
                        text(item.count).size(16).color(YELLOW).width(36),
                        text(item.display_name()).size(16),
                        column![text(item.info().category.clone()).size(16).color(GRAY)].padding([0, 5]),
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
