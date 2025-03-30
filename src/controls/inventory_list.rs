use iced::{Color, Element, Fill, Left};
use iced::widget::{column, row, scrollable, text};
use crate::events::{EliteEvent, ShipLockerItem};
use crate::color::{BLUE, ORANGE, YELLOW};

pub fn inventory_list(title: &str, items: Option<Vec<ShipLockerItem>>) -> Element<EliteEvent> {
    iced::widget::column![
            text(title).size(20).color(ORANGE),
            scrollable(column(
                items
                    .unwrap_or_default()
                    .into_iter()
                    .map(|item| {
                        
                        let color = if item.mission_id.is_some() { BLUE } else { Color::WHITE };
                        row![
                            text(item.count).size(16).color(YELLOW).width(36),
                            text(item.display_name()).color(color).size(16)
                        ]
                        .padding(2)
                    })
                    .map(Element::from)
            ))
            .width(Fill)
        ]
        .align_x(Left)
        .into()
}