use crate::event::{Event, ShipLockerItem};
use crate::state::State;
use crate::theme::{BLUE, ORANGE, YELLOW};
use iced::widget::{column, row, scrollable, text, Column, Row};
use iced::{Color, Element, Fill, Left, Top};
use std::collections::HashMap;

pub fn ship_locker(state: &State) -> Row<Event> {
    
    row![
        locker_item_list("ITEMS", state.ship_locker.items.clone()),
        locker_item_list("COMPONENTS", state.ship_locker.components.clone()),
        locker_item_list("DATA", state.ship_locker.data.clone()),
        locker_item_list("CONSUMABLES", state.ship_locker.consumables.clone()),
    ]
    .align_y(Top)
    .height(Fill)
}

pub fn locker_item_list(title: &str, items: Option<Vec<ShipLockerItem>>) -> Column<Event> {
    
    iced::widget::column![
        text(title).size(20).color(ORANGE),
        scrollable(column(
            sanitize_items(items)
                .map(|item| {
                    let color = if item.mission_id.is_some() { BLUE
                    } else {
                        Color::WHITE
                    };
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
}

fn sanitize_items(items: Option<Vec<ShipLockerItem>>) -> impl Iterator<Item = ShipLockerItem> {
    
    let mut grouped_items: HashMap<(String, Option<u64>), ShipLockerItem> = HashMap::new();

    for item in items.unwrap_or_default() {
        grouped_items
            .entry((item.name.clone(), item.mission_id))
            .and_modify(|e| e.count += item.count)
            .or_insert(item);
    }

    let mut items: Vec<_> = grouped_items.into_values().collect();
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items.into_iter()
}
