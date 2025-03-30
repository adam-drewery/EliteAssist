use crate::events::EliteEvent;
use crate::journal_poller::JournalPoller;
use crate::state::State;
use iced::futures::Stream;
use iced::Subscription;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

fn stream_events() -> impl Stream<Item = EliteEvent> {
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut poller = JournalPoller::new();
        loop {
            let input = poller.next().await;
            sender.send(input).await.unwrap();
        }
    });

    ReceiverStream::new(receiver)
}

pub fn subscription(_state: &State) -> Subscription<EliteEvent> {
    Subscription::run(stream_events)
}
