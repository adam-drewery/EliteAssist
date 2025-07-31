use crate::event::JournalEvent;
use crate::state::State;
use iced::futures::Stream;
use iced::Subscription;
use std::path::PathBuf;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub fn subscription(_state: &State) -> Subscription<JournalEvent> {
    Subscription::run(stream_events)
}

#[cfg(not(feature = "mock_events"))]
fn stream_events() -> impl Stream<Item = JournalEvent> {
    use crate::journal::JournalWatcher;

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

#[cfg(feature = "mock_events")]
fn stream_events() -> impl Stream<Item = JournalEvent> {
    use tokio::fs;

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
            events.shuffle(&mut rand::thread_rng());
            for event in events {
                sender.send(event).await.unwrap();
            }
        }
    });

    ReceiverStream::new(receiver)
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;
    use std::fs;

    fn test_deserialize_file(path: &PathBuf) -> Result<(), String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path.display(), e))?;

        serde_json::from_str::<Vec<JournalEvent>>(&content).map_err(|e| {
            let error = format!("Failed to deserialize {}: {}", path.display(), e);
            let re = Regex::new(r"(.*?), expected one of.*?(at line \d+ column \d+)").unwrap();
            if let Some(captures) = re.captures(&error) {
                format!("{} {}", &captures[1], &captures[2])
            } else {
                error
            }
        })?;
        Ok(())
    }

    #[test]
    fn test_example_files_deserialization() {
        let example_dir = PathBuf::from("src/example_data");
        let mut files = Vec::new();
        let mut dirs = vec![example_dir];

        while let Some(dir) = dirs.pop() {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() {
                    dirs.push(path);
                } else if path.extension().map_or(false, |ext| ext == "json") {
                    files.push(path);
                }
            }
        }

        let mut failed = Vec::new();
        let mut missing_variants = Vec::new();
        let variant_re = Regex::new(r"unknown variant `(\w+)`").unwrap();

        for file in files {
            if let Err(error) = test_deserialize_file(&file) {
                if let Some(captures) = variant_re.captures(&error) {
                    missing_variants.push(captures[1].to_string());
                }
                failed.push(error);
            }
        }

        assert!(
            failed.is_empty(),
            "Failed to deserialize the following files:\n{}\n\nMissing enum variants:\n{}",
            failed.join("\n"),
            missing_variants.join("\n")
        );
    }
}
