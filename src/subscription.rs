use crate::event::JournalEvent;
use crate::journal::JournalWatcher;
use crate::state::State;
use iced::futures::Stream;
use iced::Subscription;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

fn stream_events() -> impl Stream<Item =JournalEvent> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut watcher = JournalWatcher::new();
        loop {
            let input = watcher.next().await;
            sender.send(input).await.unwrap();
        }
    });

    ReceiverStream::new(receiver)
}

pub fn subscription(_state: &State) -> Subscription<JournalEvent> {
    Subscription::run(stream_events)
}
