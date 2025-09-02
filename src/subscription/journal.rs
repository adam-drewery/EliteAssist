use iced::futures::Stream;
use log::error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use crate::message::Message;

pub fn stream_history() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(64);

    tokio::spawn(async move {
        use crate::journal::HistoryLoader;
        let loader = match HistoryLoader::new() {
            Ok(l) => l,
            Err(e) => { error!("Failed to start history loader: {}", e); return; }
        };
        match loader.load_messages() {
            Ok(messages) => {
                for msg in messages.into_iter() {
                    if sender.send(msg).await.is_err() { break; }
                }
            }
            Err(e) => {
                error!("Failed to load historical messages: {}", e);
            }
        }
    });

    ReceiverStream::new(receiver)
}

pub fn stream_journal() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        use crate::journal::JournalWatcher;
        let mut watcher = match JournalWatcher::new() {
            Ok(w) => w,
            Err(e) => { error!("Failed to start journal watcher: {}", e); return; }
        };
        loop {
            match watcher.next().await {
                Ok(ev) => { if sender.send(ev).await.is_err() { break; } }
                Err(e) => { error!("Journal watcher error: {}", e); break; }
            }
        }
    });

    ReceiverStream::new(receiver)
}

fn stream_snapshot(file_name: &'static str) -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        use crate::journal::{SnapshotWatcher, get_journal_directory};
        let dir = match get_journal_directory() {
            Ok(d) => d,
            Err(e) => { error!("Failed to get journal directory: {}", e); return; }
        };
        let path = dir.join(file_name);
        let mut watcher = SnapshotWatcher::new(path);
        loop {
            match watcher.next().await {
                Ok(ev) => { if sender.send(ev).await.is_err() { break; } }
                Err(e) => { error!("Snapshot watcher error: {}", e); break; }
            }
        }
    });

    ReceiverStream::new(receiver)
}

pub fn stream_status() -> impl Stream<Item=Message> { stream_snapshot("Status.json") }

pub fn stream_backpack() -> impl Stream<Item=Message> { stream_snapshot("Backpack.json") }

pub fn stream_cargo() -> impl Stream<Item=Message> { stream_snapshot("Cargo.json") }

pub fn stream_ship_locker() -> impl Stream<Item=Message> { stream_snapshot("ShipLocker.json") }

pub fn stream_market() -> impl Stream<Item=Message> { stream_snapshot("Market.json") }

pub fn stream_navroute() -> impl Stream<Item=Message> { stream_snapshot("NavRoute.json") }

pub fn stream_capi_history() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(64);

    tokio::spawn(async move {
        let client = crate::capi::CapiClient::default();
        match client.get_journal().await {
            Ok(events) => {
                for ev in events.into_iter() {
                    if sender.send(Message::JournalEvent(ev)).await.is_err() { return; }
                }
                let _ = sender.send(Message::JournalLoaded).await;
            }
            Err(e) => {
                error!("Failed to load CAPI journal: {}", e);
                let _ = sender.send(Message::AuthFailed(format!("{}", e).into())).await;
            }
        }
    });

    ReceiverStream::new(receiver)
}
