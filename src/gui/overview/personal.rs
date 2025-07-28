use crate::event::JournalEvent;
use crate::gui::components::{details, header};
use crate::state::State;
use iced::widget::{column, Column};

pub fn personal(state: &State) -> Column<JournalEvent> {

    column![
        header("Personal"),
        details("Suit Name", &state.suit_loadout.suit_name_localised),
        details("Loadout", &state.suit_loadout.loadout_name),
        details("Empire", state.reputation.empire.to_string()),
        details("Federation", state.reputation.federation.to_string()),
        details("Alliance", state.reputation.alliance.to_string()),
    ]
    .padding(8)
}
