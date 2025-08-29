use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use iced::futures::Stream;
use log::error;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use crate::gui::Message;

#[cfg(not(feature = "mock_events"))]
pub fn stream() -> impl Stream<Item=Message> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::task::spawn_blocking(move || {
        // Register CTRL+Tab global hotkey in a blocking context
        let manager = match GlobalHotKeyManager::new() {
            Ok(m) => m,
            Err(e) => { error!("Failed to start hotkey manager: {}", e); return; }
        };
        let hotkey = HotKey::new(Some(Modifiers::CONTROL), Code::Tab);
        if let Err(e) = manager.register(hotkey) {
            error!("Failed to register CTRL+Tab hotkey: {}", e);
        }

        let rx = GlobalHotKeyEvent::receiver();
        use std::time::Duration;
        use crossbeam_channel::RecvTimeoutError;

        loop {
            // Periodically check for shutdown by timing out and inspecting sender state
            match rx.recv_timeout(Duration::from_millis(200)) {
                Ok(event) => {
                    if event.state == HotKeyState::Pressed {
                        if sender.blocking_send(Message::NextTab).is_err() { break; }
                    }
                }
                Err(err) => {
                    match err {
                        RecvTimeoutError::Timeout => {if sender.is_closed() { break; } }
                        RecvTimeoutError::Disconnected => { break; }
                    }
                }
            }
        }

        // Best-effort cleanup: unregister hotkey before exiting thread
        let _ = manager.unregister(hotkey);
    });

    ReceiverStream::new(receiver)
}