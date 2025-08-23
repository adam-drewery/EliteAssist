use crate::state::State;
use iced::futures::Stream;
use iced::Subscription;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use crate::gui::Message;

#[cfg(not(feature = "mock_events"))]
use log::error;

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
        use ed_journals::journal::auto_detect_journal_path;
        use ed_journals::journal::JournalEvent;
        use ed_journals::journal::JournalEventKind;
        use ed_journals::logs::asynchronous::LogDirReader;

        // Detect journal directory
        let Some(dir) = auto_detect_journal_path() else {
            error!("Failed to detect journal directory for history load");
            return;
        };

        // Attempt to read snapshot files once to initialize state
        // Status.json
        if let Ok(status) = ed_journals::status::asynchronous::read_status_file(dir.join("Status.json")).await {
            let _ = sender
                .send(Message::JournalEvent(JournalEvent { is_live: false, kind: JournalEventKind::StatusEvent(status) }))
                .await;
        }
        // Market.json
        if let Ok(market) = ed_journals::market::asynchronous::read_market_file(dir.join("Market.json")).await {
            let _ = sender
                .send(Message::JournalEvent(JournalEvent { is_live: false, kind: JournalEventKind::MarketEvent(market) }))
                .await;
        }
        // NavRoute.json
        if let Ok(route) = ed_journals::nav_route::asynchronous::read_nav_route_file(dir.join("NavRoute.json")).await {
            let _ = sender
                .send(Message::JournalEvent(JournalEvent { is_live: false, kind: JournalEventKind::NavRoute(route) }))
                .await;
        }
        // ShipLocker.json
        if let Ok(ship_locker) = ed_journals::ship_locker::asynchronous::read_ship_locker_file(dir.join("ShipLocker.json")).await {
            let _ = sender
                .send(Message::JournalEvent(JournalEvent { is_live: false, kind: JournalEventKind::ShipLocker(ship_locker) }))
                .await;
        }
        // Backpack.json
        if let Ok(backpack) = ed_journals::backpack::asynchronous::read_backpack_file(dir.join("Backpack.json")).await {
            let _ = sender
                .send(Message::JournalEvent(JournalEvent { is_live: false, kind: JournalEventKind::Backpack(backpack) }))
                .await;
        }
        // Cargo.json
        if let Ok(cargo) = ed_journals::cargo::asynchronous::read_cargo_file(dir.join("Cargo.json")).await {
            let _ = sender
                .send(Message::JournalEvent(JournalEvent { is_live: false, kind: JournalEventKind::Cargo(cargo) }))
                .await;
        }

        // Now iterate historical log events
        let mut reader = LogDirReader::open(&dir);
        while let Some(result) = reader.next().await {
            match result {
                Ok(ev) => {
                    if sender.send(Message::LogEvent(ev)).await.is_err() { break; }
                }
                Err(e) => {
                    error!("Failed reading historical log event: {}", e);
                    // Continue reading remaining entries
                }
            }
        }

        // Signal that journal history has been fully loaded
        let _ = sender.send(Message::JournalLoaded).await;
    });

    ReceiverStream::new(receiver)
}

#[cfg(not(feature = "mock_events"))]
fn stream_journal() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        use ed_journals::journal::asynchronous::LiveJournalDirReader;
        use ed_journals::journal::auto_detect_journal_path;
        use ed_journals::journal::JournalEventKind;

        let Some(dir) = auto_detect_journal_path() else {
            error!("Failed to detect journal directory for live journal");
            return;
        };

        let mut reader = match LiveJournalDirReader::open(&dir) {
            Ok(r) => r,
            Err(e) => { error!("Failed to start LiveJournalDirReader: {}", e); return; }
        };

        loop {
            match reader.next().await {
                Some(Ok(event)) => {
                    match event.kind {
                        JournalEventKind::LogEvent(log) => {
                            if sender.send(Message::LogEvent(log)).await.is_err() { break; }
                        }
                        _ => {
                            // Ignore here; dedicated snapshot streams handle these
                        }
                    }
                }
                Some(Err(e)) => { error!("LiveJournalDirReader error: {}", e); }
                None => { break; }
            }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(not(feature = "mock_events"))]
fn stream_snapshot(file_name: &'static str) -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        use ed_journals::journal::auto_detect_journal_path;
        use ed_journals::journal::{JournalEvent, JournalEventKind};

        let Some(dir) = auto_detect_journal_path() else {
            error!("Failed to detect journal directory for snapshot: {file_name}");
            return;
        };
        let path = dir.join(file_name);

        match file_name {
            "Status.json" => {
                use ed_journals::status::asynchronous::StatusFileWatcher;
                let mut watcher = match StatusFileWatcher::open(&path) {
                    Ok(w) => w,
                    Err(e) => { error!("Failed to open StatusFileWatcher: {}", e); return; }
                };
                loop {
                    match watcher.next().await {
                        Some(Ok(value)) => {
                            let event = JournalEvent { is_live: true, kind: JournalEventKind::StatusEvent(value) };
                            if sender.send(Message::JournalEvent(event)).await.is_err() { break; }
                        }
                        Some(Err(e)) => { error!("StatusFileWatcher error: {}", e); }
                        None => break,
                    }
                }
            }
            "Market.json" => {
                use ed_journals::market::asynchronous::MarketFileWatcher;
                let mut watcher = match MarketFileWatcher::open(&path) {
                    Ok(w) => w,
                    Err(e) => { error!("Failed to open MarketFileWatcher: {}", e); return; }
                };
                loop {
                    match watcher.next().await {
                        Some(Ok(value)) => {
                            let event = JournalEvent { is_live: true, kind: JournalEventKind::MarketEvent(value) };
                            if sender.send(Message::JournalEvent(event)).await.is_err() { break; }
                        }
                        Some(Err(e)) => { error!("MarketFileWatcher error: {}", e); }
                        None => break,
                    }
                }
            }
            "NavRoute.json" => {
                use ed_journals::nav_route::asynchronous::NavRouteFileWatcher;
                let mut watcher = match NavRouteFileWatcher::open(&path) {
                    Ok(w) => w,
                    Err(e) => { error!("Failed to open NavRouteFileWatcher: {}", e); return; }
                };
                loop {
                    match watcher.next().await {
                        Some(Ok(value)) => {
                            let event = JournalEvent { is_live: true, kind: JournalEventKind::NavRoute(value) };
                            if sender.send(Message::JournalEvent(event)).await.is_err() { break; }
                        }
                        Some(Err(e)) => { error!("NavRouteFileWatcher error: {}", e); }
                        None => break,
                    }
                }
            }
            "Backpack.json" => {
                use ed_journals::backpack::asynchronous::BackpackFileWatcher;
                let mut watcher = match BackpackFileWatcher::open(&path) {
                    Ok(w) => w,
                    Err(e) => { error!("Failed to open BackpackFileWatcher: {}", e); return; }
                };
                loop {
                    match watcher.next().await {
                        Some(Ok(value)) => {
                            let event = JournalEvent { is_live: true, kind: JournalEventKind::Backpack(value) };
                            if sender.send(Message::JournalEvent(event)).await.is_err() { break; }
                        }
                        Some(Err(e)) => { error!("BackpackFileWatcher error: {}", e); }
                        None => break,
                    }
                }
            }
            "Cargo.json" => {
                use ed_journals::cargo::asynchronous::CargoFileWatcher;
                let mut watcher = match CargoFileWatcher::open(&path) {
                    Ok(w) => w,
                    Err(e) => { error!("Failed to open CargoFileWatcher: {}", e); return; }
                };
                loop {
                    match watcher.next().await {
                        Some(Ok(value)) => {
                            let event = JournalEvent { is_live: true, kind: JournalEventKind::Cargo(value) };
                            if sender.send(Message::JournalEvent(event)).await.is_err() { break; }
                        }
                        Some(Err(e)) => { error!("CargoFileWatcher error: {}", e); }
                        None => break,
                    }
                }
            }
            "ShipLocker.json" => {
                use ed_journals::ship_locker::asynchronous::ShipLockerFileWatcher;
                let mut watcher = match ShipLockerFileWatcher::open(&path) {
                    Ok(w) => w,
                    Err(e) => { error!("Failed to open ShipLockerFileWatcher: {}", e); return; }
                };
                loop {
                    match watcher.next().await {
                        Some(Ok(value)) => {
                            let event = JournalEvent { is_live: true, kind: JournalEventKind::ShipLocker(value) };
                            if sender.send(Message::JournalEvent(event)).await.is_err() { break; }
                        }
                        Some(Err(e)) => { error!("ShipLockerFileWatcher error: {}", e); }
                        None => break,
                    }
                }
            }
            other => {
                error!("Unsupported snapshot file: {}", other);
            }
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
