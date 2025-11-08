use iced::futures::Stream;
use log::error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use crate::message::Message;

pub fn stream_history() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(64);

    tokio::spawn(async move {
        use crate::journal::{HistoryLoader, get_journal_directory};
        use tokio::time::{sleep, Duration};

        // Poll periodically until a valid directory with at least one .log file is available
        loop {
            let dir = match get_journal_directory() {
                Ok(d) => d,
                Err(e) => { error!("Failed to get journal directory: {}", e); sleep(Duration::from_millis(750)).await; continue; }
            };

            // Check for at least one .log file
            let has_logs = if dir.exists() {
                std::fs::read_dir(&dir)
                    .ok()
                    .map(|iter| {
                        for entry in iter.flatten() {
                            let p = entry.path();
                            if p.extension().and_then(|e| e.to_str()).map(|e| e.eq_ignore_ascii_case("log")).unwrap_or(false) {
                                return true;
                            }
                        }
                        false
                    })
                    .unwrap_or(false)
            } else { false };

            if !has_logs {
                // Keep the waiting screen visible; try again shortly
                sleep(Duration::from_millis(750)).await;
                continue;
            }

            // We have logs: load and emit messages, then exit the task
            let loader = HistoryLoader::with_dir(dir);
            match loader.load_messages() {
                Ok(messages) => {
                    for msg in messages.into_iter() {
                        if sender.send(msg).await.is_err() { break; }
                    }
                    break;
                }
                Err(e) => {
                    error!("Failed to load historical messages: {}", e);
                    // Wait a bit before retrying to avoid tight loop on persistent error
                    sleep(Duration::from_millis(750)).await;
                }
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