use iced::futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use crate::gui::Message;
use crate::journal;

pub fn stream() -> impl Stream<Item = Message> {
    use tokio::fs;
    use std::path::PathBuf;

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
            let mut events: Vec<journal::Event> = serde_json::from_str(&content).unwrap();
            use rand::seq::SliceRandom;
            events.shuffle(&mut rand::rng());
            for event in events {
                sender.send(Message::JournalEvent(event)).await.unwrap();
            }
        }
    });

    ReceiverStream::new(receiver)
}