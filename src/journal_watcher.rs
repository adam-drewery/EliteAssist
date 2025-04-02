use crate::event::Event;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use tokio::fs;
use tokio::select;
use tokio::sync::mpsc;
use log::debug;

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
    journal_files: Vec<PathBuf>,
    current_file_index: usize,
}

impl JournalWatcher {
    pub fn new() -> Self {
        let dir_path = Path::new(JOURNAL_DIRECTORY);
        let journal_files = get_journal_paths(dir_path).unwrap();
        let current_journal_path = journal_files[0].clone();
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
            journal_files,
            current_file_index: 0,
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
            // Try to read next line from current file
            let mut buffer = String::new();
            let bytes_read = self.reader.read_line(&mut buffer).unwrap();

            if bytes_read > 0 {
                let line = buffer.as_str();
                let deserialize_result = serde_json::from_str(line);

                if let Ok(event) = deserialize_result {
                    return event;
                }
                else if let Err(e) = deserialize_result {
                    let error_msg = e.to_string();
                    if error_msg.starts_with("unknown variant") {
                        if let Some(first_part) = error_msg.split(',').next() {
                            eprintln!("Failed to parse journal entry: {}", &line);
                            eprintln!("{}\n", first_part);
                        }
                    } else {
                        eprintln!("Failed to parse journal entry: {}", &line);
                        eprintln!("{}\n", &e);
                    }
                }
            }

            // Reached end of current file
            if self.current_file_index < self.journal_files.len() - 1 {
                // Move to next journal file
                self.current_file_index += 1;
                self.current_journal_path = self.journal_files[self.current_file_index].clone();
                let new_file = OpenOptions::new()
                    .read(true)
                    .open(&self.current_journal_path)
                    .unwrap();
                self.reader = BufReader::new(new_file);
                //println!("\nMoving to next journal file: {}\n", self.current_journal_path.display());
                continue;
            }

            // On latest file, wait for changes
            select! {
                _ = self.watcher_rx.recv() => {
                    // Check snapshot files
                    for file_details in &mut self.snapshot_files {
                        if let Some(event) = check_snapshot_file(file_details) {
                            return event;
                        }
                    }
    
                    // Check for new journal files
                    let dir_path = Path::new(JOURNAL_DIRECTORY);
                    let journal_files = get_journal_paths(dir_path).unwrap();
                    
                    if journal_files.len() > self.journal_files.len() {
                        println!("\nNew journal file detected!\n");
                        self.journal_files = journal_files;
                    }
                }
            }
        }
    }
}

/// Return the path to the newest `.log` file in `JOURNAL_DIRECTORY`.
fn get_journal_paths(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files: Vec<_> = std::fs::read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let is_log = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("log"))
                .unwrap_or(false);

            if is_log { Some(path) } else { None }
        })
        .collect();

    files.sort_by_key(|path| {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    });

    Ok(files)
}

fn check_snapshot_file(file_details: &mut FileDetails) -> Option<Event> {
    // Check status.json first
    let metadata = std::fs::metadata(&file_details.path).unwrap();
    let modified = metadata.modified().unwrap();

    if modified > file_details.last_modified {
        let mut line = String::new();
        let mut file = File::open(&file_details.path).unwrap();
        if file.read_to_string(&mut line).is_ok() {
            file_details.last_modified = modified;

            let deserizlize_result = serde_json::from_str(&line);
            if let Ok(event) = deserizlize_result {
                debug!("Handling {}\n", &line);
                return event;
            } else if let Err(e) = deserizlize_result {
                let error_msg = e.to_string();
                if error_msg.starts_with("unknown variant") {
                    if let Some(first_part) = error_msg.split(',').next() {
                        eprintln!("Failed to parse journal entry: {}", &line);
                        eprintln!("{}\n", first_part);
                    }
                } else {
                    eprintln!("Failed to parse journal entry: {}", &line);
                    eprintln!("{}\n", &e);
                }
            }
        }
    }
    None
}