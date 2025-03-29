use crate::events::EliteEvent;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::{io};

const JOURNAL_DIRECTORY: &str = "/home/adam/.steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/";

pub struct JournalPoller {
    reader: BufReader<File>,
    current_journal_path: PathBuf,
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

        Self {
            reader,
            current_journal_path,
        }
    }
    
    pub async fn next(&mut self) -> EliteEvent {
        loop { 
            let mut buffer = String::new();
            let bytes_read = self.reader.read_line(&mut buffer).unwrap();

            if bytes_read > 0 {
                 let line = buffer.as_str();
                return serde_json::from_str(line).unwrap();
            } else {
                let dir_path = Path::new(JOURNAL_DIRECTORY); // todo: store this somewhere
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
