use iced::futures::Stream;
use iced::Subscription;
use journal_poller::JournalPoller;

use crate::events::{Commander, EliteEvent};

mod journal_poller;
mod events;
mod gui;

#[tokio::main]
async fn main() {

    iced::application("EliteAssist", gui::MainView::update, gui::MainView::view)
        .subscription(subscription)
        .run()
        .unwrap();
}

#[derive(Default)]
struct State {
    commander: Commander,
}

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

fn some_worker() -> impl Stream<Item = EliteEvent> {
    // Create a Tokio channel with a buffer size of 100
    let (sender, receiver) = mpsc::channel(16);

    tokio::spawn(async move {
        let mut poller = JournalPoller::new();
        loop {
                let input = poller.next().await;
                sender.send(input).await.unwrap();
            }
        }
    );

    // Convert the receiver into a stream
    ReceiverStream::new(receiver)
}

fn subscription(_state: &State) -> Subscription<EliteEvent> {
    Subscription::run(some_worker)
}