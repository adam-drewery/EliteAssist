use crate::event::Event;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::fs;
use tokio::select;
use tokio::sync::mpsc;

const JOURNAL_DIRECTORY: &str = "/home/adam/.steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/";

struct FileDetails {
    path: PathBuf,
    last_modified: SystemTime,
}

impl FileDetails {
    fn new(path: PathBuf) -> Self {
        Self { path, last_modified: SystemTime::UNIX_EPOCH }
    }
}

pub struct JournalWatcher {
    reader: BufReader<File>,
    current_journal_path: PathBuf,
    snapshot_files: Vec<FileDetails>,
    watcher_tx: mpsc::Sender<()>,
    watcher_rx: mpsc::Receiver<()>,
}

impl JournalWatcher {
    pub fn new() -> Self {
        let dir_path = Path::new(JOURNAL_DIRECTORY);
        let current_journal_path = get_latest_journal_path(dir_path).unwrap();
        let file = OpenOptions::new()
            .read(true)
            .open(&current_journal_path)
            .unwrap();

        let mut reader = BufReader::new(file);
        reader.seek(SeekFrom::Start(0)).unwrap();

        let snapshot_files = vec![
            FileDetails::new(dir_path.join("Status.json")),
            FileDetails::new(dir_path.join("Backpack.json")),
            FileDetails::new(dir_path.join("Cargo.json")),
            FileDetails::new(dir_path.join("Market.json")),
        ];

        let (watcher_tx, watcher_rx) = mpsc::channel(32);

        let poller = Self {
            reader,
            current_journal_path,
            snapshot_files,
            watcher_tx,
            watcher_rx,
        };

        poller.spawn_watcher();
        poller
    }

    fn spawn_watcher(&self) {
        let tx = self.watcher_tx.clone();
        let dir = PathBuf::from(JOURNAL_DIRECTORY);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(1));

            loop {
                interval.tick().await;

                if let Ok(mut entries) = fs::read_dir(&dir).await {
                    while let Ok(Some(entry)) = entries.next_entry().await {
                        if let Ok(metadata) = entry.metadata().await {
                            if metadata.is_file() && entry.path().extension().map_or(false, |ext| ext == "json" || ext == "log") {
                                let _ = tx.send(()).await;
                            }
                        }
                    }
                }
            }
        });
    }

    pub async fn next(&mut self) -> Event {
        loop {
            select! {
                _ = self.watcher_rx.recv() => {
                    // Check snapshot files
                    for file_details in &mut self.snapshot_files {
                        if let Some(event) = check_snapshot_file(file_details) {
                            return event;
                        }
                    }

                    // Check latest journal
                    let dir_path = Path::new(JOURNAL_DIRECTORY);
                    let latest_path = get_latest_journal_path(dir_path).unwrap();
                    
                    if latest_path != self.current_journal_path {
                        println!(
                            "\nNewer log file detected! Switching to: {}\n",
                            latest_path.display()
                        );
                        
                        self.current_journal_path = latest_path;
                        let new_file = OpenOptions::new()
                            .read(true)
                            .open(&self.current_journal_path)
                            .unwrap();
                        
                        self.reader = BufReader::new(new_file);
                        self.reader.seek(SeekFrom::Start(0)).unwrap();
                    }

                    let mut buffer = String::new();
                    let bytes_read = self.reader.read_line(&mut buffer).unwrap();

                    if bytes_read > 0 {
                        let line = buffer.as_str();
                        if let Ok(event) = serde_json::from_str(&line) {
                            println!("Handling {}", line);
                            return event;
                        } else {
                            eprintln!("Failed to parse journal file: {}", &line);
                        }
                    }
                }
            }
        }
    }
}

/// Return the path to the newest `.log` file in `JOURNAL_DIRECTORY`.
fn get_latest_journal_path(dir: &Path) -> io::Result<PathBuf> {
    let newest = std::fs::read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            // Check extension == ".log"  
            let is_log = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("log"))
                .unwrap_or(false);

            if is_log { Some(path) } else { None }
        })
        // Compare by last modification time
        .max_by_key(|path| {
            std::fs::metadata(path)
                .and_then(|m| m.modified())
                .unwrap_or(SystemTime::UNIX_EPOCH)
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No .log files found"))?;

    Ok(newest)
}

fn check_snapshot_file(file_details: &mut FileDetails) -> Option<Event> {
    // Check status.json first
    let metadata = std::fs::metadata(&file_details.path).unwrap();
    let modified = metadata.modified().unwrap();

    if modified > file_details.last_modified {
        let mut content = String::new();
        let mut file = File::open(&file_details.path).unwrap();
        if file.read_to_string(&mut content).is_ok() {
            file_details.last_modified = modified;
            if let Ok(event) = serde_json::from_str(&content) {
                println!("Handling {}\n", content);
                return event;
            } else {
                eprintln!("Failed to parse snapshot file: {}", &content);
            }
        }
    }
    None
}