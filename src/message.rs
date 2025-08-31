use crate::gui::pane;
use crate::journal::Event;
use crate::query as query_api;
use crate::state::State;
use chrono::Utc;
use iced::Task;

mod gui;
mod query;
mod journal;

pub use gui::Gui;
pub use query::Query;

#[derive(Clone, Debug)]
pub enum Message {
    Gui(Gui),
    Query(Query),
    JournalEvent(Event),
    JournalLoaded,
    Empty,
}

impl Message {
    pub fn update(self, state: &mut State) -> Task<Message> {
        match self {

            Message::Gui(gui) => gui.update(state),

            Message::Query(q) => q.update(state),

            Message::JournalEvent(event) => event.update(state),

            Message::JournalLoaded => journal_loaded(state),

            Message::Empty => Task::none(),
        }
    }
}

fn journal_loaded(state: &mut State) ->  Task<Message> {
    
    state.journal_loaded = true;
    if state.layout.current_panes.is_none() { pane::load(&mut state.layout) }

    // some missions could have expired while we were away.
    let expired_mission_ids: Vec<_> = state.missions.iter()
        .filter(|m| m.expiry.map_or(false, |e| e < Utc::now()))
        .map(|m| m.mission_id)
        .collect();

    state.missions.retain(|m| !expired_mission_ids.contains(&m.mission_id));

    query_api::system(
        state.current_system.as_ref(),
        state.ship_loadout.max_jump_range)
}