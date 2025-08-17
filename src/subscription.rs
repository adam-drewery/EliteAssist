mod test;

use crate::state::State;
use iced::futures::Stream;
use iced::Subscription;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use crate::gui::Message;

/// Derives the active subscriptions from the current State.
/// Note: Iced calls this function on every update with the latest State,
/// so after the JournalLoaded message is received (state.journal_loaded = true),
/// stream_history will no longer be subscribed and the live streams will be started.
/// Until then, only stream_history is active to ensure JournalLoaded is emitted first.
#[cfg(not(feature = "mock_events"))]
pub fn subscription(state: &State) -> Subscription<Message> {
    if !state.journal_loaded {
        Subscription::run(stream_history)
    } else {
        Subscription::batch(vec![
            Subscription::run(stream_journal),
            Subscription::run(stream_status),
            Subscription::run(stream_backpack),
            Subscription::run(stream_cargo),
            Subscription::run(stream_ship_locker),
            Subscription::run(stream_market),
            Subscription::run(stream_navroute),
        ])
    }
}

#[cfg(feature = "mock_events")]
pub fn subscription(_state: &State) -> Subscription<Message> {
    Subscription::run(stream_events)
}

#[cfg(not(feature = "mock_events"))]
fn stream_events() -> impl Stream<Item=Message> {
    use crate::journal::JournalWatcher;

    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut watcher = JournalWatcher::new();
        loop {
            let ev = watcher.next().await;
            if sender.send(ev).await.is_err() {
                break;
            }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(feature = "mock_events")]
fn stream_events() -> impl Stream<Item = Message> {
    use tokio::fs;
    use std::path::PathBuf;
    use crate::event::JournalEvent;

    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        let example_dir = PathBuf::from("src/example_data");
        let mut files = Vec::new();
        let mut dirs = vec![example_dir];

        while let Some(dir) = dirs.pop() {
            let mut entries = fs::read_dir(dir).await.unwrap();
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().map_or(false, |ext| ext == "json") {
                    files.push(path);
                }
            }
        }

        for file in files {
            let content = fs::read_to_string(file).await.unwrap();
            let mut events: Vec<JournalEvent> = serde_json::from_str(&content).unwrap();
            use rand::seq::SliceRandom;
            events.shuffle(&mut rand::rng());
            for event in events {
                sender.send(Message::JournalEvent(event)).await.unwrap();
            }
        }
    });

    ReceiverStream::new(receiver)
}

/// Emits historical journal and snapshot messages followed by `Message::JournalLoaded`, then terminates.
/// This stream is only subscribed when `state.journal_loaded == false`.
#[cfg(not(feature = "mock_events"))]
fn stream_history() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(64);

    tokio::spawn(async move {
        use crate::journal::HistoryLoader;
        let loader = HistoryLoader::new();
        // Emit all messages synchronously into the channel
        for msg in loader.load_messages().into_iter() {
            if sender.send(msg).await.is_err() { break; }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(not(feature = "mock_events"))]
fn stream_journal() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        use crate::journal::JournalWatcher;
        let mut watcher = JournalWatcher::new();
        loop {
            let ev = watcher.next().await;
            if sender.send(ev).await.is_err() { break; }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(not(feature = "mock_events"))]
fn stream_snapshot(file_name: &'static str) -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        use crate::journal::{SnapshotWatcher, get_journal_directory};
        let path = get_journal_directory().join(file_name);
        let mut watcher = SnapshotWatcher::new(path);
        loop {
            let ev = watcher.next().await;
            if sender.send(ev).await.is_err() { break; }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(not(feature = "mock_events"))]
fn stream_status() -> impl Stream<Item=Message> { stream_snapshot("Status.json") }
#[cfg(not(feature = "mock_events"))]
fn stream_backpack() -> impl Stream<Item=Message> { stream_snapshot("Backpack.json") }
#[cfg(not(feature = "mock_events"))]
fn stream_cargo() -> impl Stream<Item=Message> { stream_snapshot("Cargo.json") }
#[cfg(not(feature = "mock_events"))]
fn stream_ship_locker() -> impl Stream<Item=Message> { stream_snapshot("ShipLocker.json") }
#[cfg(not(feature = "mock_events"))]
fn stream_market() -> impl Stream<Item=Message> { stream_snapshot("Market.json") }
#[cfg(not(feature = "mock_events"))]
fn stream_navroute() -> impl Stream<Item=Message> { stream_snapshot("NavRoute.json") }
