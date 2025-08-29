use crate::gui::Message;
use crate::state::State;
use iced::Subscription;

#[cfg(test)]
mod test;
mod hotkey;

#[cfg(not(feature = "mock_events"))]
mod journal;

#[cfg(not(feature = "mock_events"))]
pub fn subscription(state: &State) -> Subscription<Message> {
    if !state.journal_loaded {
        Subscription::batch(vec![Subscription::run(journal::stream_history)])
    } else {
        Subscription::batch(vec![
            Subscription::run(journal::stream_journal),
            Subscription::run(journal::stream_status),
            Subscription::run(journal::stream_backpack),
            Subscription::run(journal::stream_cargo),
            Subscription::run(journal::stream_ship_locker),
            Subscription::run(journal::stream_market),
            Subscription::run(journal::stream_navroute),
            Subscription::run(hotkey::stream),
        ])
    }
}

#[cfg(feature = "mock_events")]
mod example_data;

#[cfg(feature = "mock_events")]
pub fn subscription(_state: &State) -> Subscription<Message> {
    Subscription::run(example_data::stream)
}
