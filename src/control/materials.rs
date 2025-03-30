use crate::control::materials_list::materials_list;
use crate::event::Event;
use crate::state::State;
use iced::widget::{row, Row};
use iced::{Fill, Top};

pub fn materials(state: &State) -> Row<Event> {
    
    row![
        materials_list("RAW", state.materials.raw.clone()),
        materials_list("MANUFACTURED", state.materials.manufactured.clone()),
        materials_list("ENCODED", state.materials.encoded.clone()),
    ]
    .align_y(Top)
    .height(Fill)
}