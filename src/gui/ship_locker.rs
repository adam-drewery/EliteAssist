use crate::event::JournalEvent;
use crate::state::{ShipLockerItem, State};
use crate::theme::{BLUE, ORANGE, YELLOW};
use iced::widget::{column, row, scrollable, text, Column, Row};
use iced::{Color, Element, Fill, Left, Top};

pub fn ship_locker(state: &State) -> Row<JournalEvent> {

    row![
        locker_item_list("ITEMS", &state.ship_locker.items),
        locker_item_list("COMPONENTS", &state.ship_locker.components),
        locker_item_list("DATA", &state.ship_locker.data),
        locker_item_list("CONSUMABLES", &state.ship_locker.consumables),
    ]
    .align_y(Top)
    .height(Fill)
}

pub fn locker_item_list<'a>(title: &'a str, items: &'a [ShipLockerItem]) -> Column<'a, JournalEvent> {

    column![
        text(title).size(20).color(ORANGE),
        scrollable(column(
            items
            .iter()
                .map(|item| {
                    let color = if item.for_mission { BLUE } else { Color::WHITE };
                    row![
                        text(item.count).size(16).color(YELLOW).width(36),
                        text(&item.name).color(color).size(16)
                    ]
                    .padding(2)
                })
                .map(Element::from)
        ))
        .width(Fill)
    ]
    .align_x(Left)
}