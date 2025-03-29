use crate::events::EliteEvent;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Read};
use std::path::{Path, PathBuf};
use std::{io};
use std::time::SystemTime;

const JOURNAL_DIRECTORY: &str = "/home/adam/.steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/";

pub struct JournalPoller {
    reader: BufReader<File>,
    current_journal_path: PathBuf,
    status_path: PathBuf,
    last_status_modified: SystemTime,
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

        let status_path = dir_path.join("Status.json");
        let last_status_modified = SystemTime::UNIX_EPOCH;

        Self {
            reader,
            current_journal_path,
            status_path,
            last_status_modified
        }
    }

    pub async fn next(&mut self) -> EliteEvent {
        loop {
            // Check status.json first
            if let Ok(metadata) = std::fs::metadata(&self.status_path) {
                if let Ok(modified) = metadata.modified() {
                    if modified > self.last_status_modified {
                        let mut content = String::new();
                        if let Ok(mut file) = File::open(&self.status_path) {
                            if file.read_to_string(&mut content).is_ok() {
                                self.last_status_modified = modified;
                                if let Ok(event) = serde_json::from_str(&content) {
                                    return event;
                                }
                            }
                        }
                    }
                }
            }

            let mut buffer = String::new();
            let bytes_read = self.reader.read_line(&mut buffer).unwrap();

            if bytes_read > 0 {
                let line = buffer.as_str();
                return serde_json::from_str(line).unwrap();
            } else {
                let dir_path = Path::new(JOURNAL_DIRECTORY);
                if let Ok(latest_path) = get_latest_journal_path(dir_path) {
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
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        })
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "No .log files found"))?;

    Ok(newest)
}