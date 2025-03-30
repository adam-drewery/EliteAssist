use crate::controls::inventory_list;
use crate::events::EliteEvent;
use crate::state::State;
use iced::widget::{row, Row};
use iced::{Fill, Top};

pub fn ship_locker(state: &State) -> Row<EliteEvent> {
    
    row![
        inventory_list("ITEMS", state.ship_locker.items.clone()),
        inventory_list("COMPONENTS", state.ship_locker.components.clone()),
        inventory_list("DATA", state.ship_locker.data.clone()),
        inventory_list("CONSUMABLES", state.ship_locker.consumables.clone()),
    ]
    .align_y(Top)
    .height(Fill)
}