use crate::event::Event;
use crate::gui::locker_item_list;
use crate::state::State;
use iced::widget::{row, Row};
use iced::{Fill, Top};

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