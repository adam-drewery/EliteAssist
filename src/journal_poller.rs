use crate::events::EliteEvent;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Read};
use std::path::{Path, PathBuf};
use std::{io};
use std::time::{Duration, SystemTime};

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

pub struct JournalPoller {
    reader: BufReader<File>,
    current_journal_path: PathBuf,
    snapshot_files: Vec<FileDetails>,
}

impl JournalPoller {
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

        Self {
            reader,
            current_journal_path,
            snapshot_files,
        }
    }

    pub async fn next(&mut self) -> EliteEvent {
        loop {
            
            for file_details in &mut self.snapshot_files {
                if let Some(event) = check_snapshot_file(file_details) {
                    return event;
                }
            }

            let mut buffer = String::new();
            let bytes_read = self.reader.read_line(&mut buffer).unwrap();

            if bytes_read > 0 {
                let line = buffer.as_str();
                
                if let Ok(event) = serde_json::from_str(&line) {
                    println!("Handling {}\n", line);
                    return event;
                } else {
                    eprintln!("Failed to parse journal file: {}", &line);
                }
            } else {
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
            }

            tokio::time::sleep(Duration::from_millis(500)).await;
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

fn check_snapshot_file(file_details: &mut FileDetails) -> Option<EliteEvent> {
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