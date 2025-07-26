use crate::event::JournalEvent;
use log::{debug, error, info};
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
        Self {
            path,
            last_modified: SystemTime::UNIX_EPOCH,
        }
    }
}

pub struct JournalWatcher {
    reader: Option<BufReader<File>>,
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

        // Get journal files, handling the case where they don't exist yet
        let journal_files = get_journal_paths(dir_path).unwrap_or_else(|e| {
            error!("Error getting journal paths: {}", e);
            Vec::new()
        });

        // Initialize reader and current path
        let (reader, current_journal_path) = if !journal_files.is_empty() {
            let path = journal_files[0].clone();
            match OpenOptions::new().read(true).open(&path) {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    if let Err(e) = reader.seek(SeekFrom::Start(0)) {
                        error!("Failed to seek in journal file: {}", e);
                    }
                    (Some(reader), path)
                }
                Err(e) => {
                    error!("Failed to open journal file {}: {}", path.display(), e);
                    (None, path)
                }
            }
        } else {
            info!("No journal files found. Waiting for files to be created...");
            (None, dir_path.to_path_buf())
        };

        let snapshot_files = vec![
            FileDetails::new(dir_path.join("Status.json")),
            FileDetails::new(dir_path.join("Backpack.json")),
            FileDetails::new(dir_path.join("Cargo.json")),
            FileDetails::new(dir_path.join("ShipLocker.json")),
            FileDetails::new(dir_path.join("Market.json")),
            FileDetails::new(dir_path.join("NavRoute.json")),
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
                            if metadata.is_file()
                                && entry
                                    .path()
                                    .extension()
                                    .map_or(false, |ext| ext == "json" || ext == "log")
                            {
                                let _ = tx.send(()).await;
                            }
                        }
                    }
                }
            }
        });
    }

    pub async fn next(&mut self) -> JournalEvent {
        loop {
            // Check if we have a reader
            if let Some(reader) = &mut self.reader {
                // Try to read next line from current file
                let mut buffer = String::new();
                match reader.read_line(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let line = buffer.as_str();
                        
                        debug!("Journal file updated: {}", &line);
                        let deserialize_result = serde_json::from_str(line);

                        if let Ok(event) = deserialize_result {
                            return event;
                        } else if let Err(e) = deserialize_result {
                            let error_msg = e.to_string();
                            if error_msg.starts_with("unknown variant") {
                                if let Some(first_part) = error_msg.split(',').next() {
                                    error!(
                                        "Failed to parse journal entry: {}\n{}",
                                        first_part, &line
                                    );
                                }
                            } else {
                                error!("Failed to parse journal entry: {}\n{}", &e, &line);
                            }
                        }
                    }
                    Ok(_) => {
                        // Reached end of file, try next file if available
                        if self.current_file_index < self.journal_files.len() - 1 {
                            // Move to next journal file
                            self.current_file_index += 1;
                            self.current_journal_path =
                                self.journal_files[self.current_file_index].clone();
                            match OpenOptions::new()
                                .read(true)
                                .open(&self.current_journal_path)
                            {
                                Ok(new_file) => {
                                    self.reader = Some(BufReader::new(new_file));
                                    if let Some(file_name) = self.current_journal_path.file_name() {
                                        if let Some(name) = file_name.to_str() {
                                            info!("Scanning journal file: {}", name);
                                        }
                                    }
                                    continue;
                                }
                                Err(e) => {
                                    error!(
                                        "Failed to open journal file {}: {}",
                                        self.current_journal_path.display(),
                                        e
                                    );
                                    self.reader = None;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error reading from journal file: {}", e);
                        self.reader = None;
                    }
                }
            }

            // If we don't have a reader or reached the end of all files, wait for changes
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
                    match get_journal_paths(dir_path) {
                        Ok(journal_files) => {
                            if !journal_files.is_empty() {
                                // If we have new files or we didn't have a reader before
                                if journal_files.len() > self.journal_files.len() || self.reader.is_none() {
                                    info!("New journal file(s) detected!");

                                    // Try to open the newest file if we don't have a reader
                                    if self.reader.is_none() && !journal_files.is_empty() {
                                        let newest_file = &journal_files[0];
                                        match OpenOptions::new().read(true).open(newest_file) {
                                            Ok(file) => {
                                                let mut reader = BufReader::new(file);
                                                if let Err(e) = reader.seek(SeekFrom::Start(0)) {
                                                    error!("Failed to seek in journal file: {}", e);
                                                }
                                                self.reader = Some(reader);
                                                self.current_journal_path = newest_file.clone();
                                                self.current_file_index = 0;
                                            },
                                            Err(e) => {
                                                error!("Failed to open journal file {}: {}",
                                                       newest_file.display(), e);
                                            }
                                        }
                                    }

                                    self.journal_files = journal_files;
                                }
                            }
                        },
                        Err(e) => {
                            error!("Failed to get journal paths: {}", e);
                        }
                    }
                }
            }
        }
    }
}

/// Return the path to the newest `.log` file in `JOURNAL_DIRECTORY`.
/// If the directory doesn't exist, returns an empty vector.
fn get_journal_paths(dir: &Path) -> io::Result<Vec<PathBuf>> {
    // Check if directory exists
    if !dir.exists() {
        info!("Journal directory does not exist yet: {}", dir.display());
        return Ok(Vec::new());
    }

    // Try to read the directory
    let read_dir = match std::fs::read_dir(dir) {
        Ok(rd) => rd,
        Err(e) => {
            error!("Failed to read journal directory: {}", e);
            return Ok(Vec::new());
        }
    };

    let mut files: Vec<_> = read_dir
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

    // Sort files by modified time, handling errors gracefully
    files.sort_by_key(|path| {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    });

    Ok(files)
}

fn check_snapshot_file(file_details: &mut FileDetails) -> Option<JournalEvent> {
    // Check if the file exists
    if !file_details.path.exists() {
        // File doesn't exist yet, just return None
        return None;
    }

    // Get file metadata
    let metadata = match std::fs::metadata(&file_details.path) {
        Ok(meta) => meta,
        Err(e) => {
            error!(
                "Failed to get metadata for snapshot file {}: {}",
                file_details.path.display(),
                e
            );
            return None;
        }
    };

    // Get modified time
    let modified = match metadata.modified() {
        Ok(time) => time,
        Err(e) => {
            error!(
                "Failed to get modified time for snapshot file {}: {}",
                file_details.path.display(),
                e
            );
            return None;
        }
    };

    // Check if file has been modified since last check
    if modified > file_details.last_modified {
        // Try to open the file
        let file = match File::open(&file_details.path) {
            Ok(f) => f,
            Err(e) => {
                error!(
                    "Failed to open snapshot file {}: {}",
                    file_details.path.display(),
                    e
                );
                return None;
            }
        };

        // Read file contents
        let mut line = String::new();
        let mut file_reader = BufReader::new(file);
        if file_reader.read_to_string(&mut line).is_ok() && !line.is_empty() {
            file_details.last_modified = modified;

            info!("Snapshot file updated: {}", &line);
            let deserialize_result = serde_json::from_str(&line);
            if let Ok(event) = deserialize_result {
                return Some(event);
            } else if let Err(e) = deserialize_result {
                let error_msg = e.to_string();
                if error_msg.starts_with("unknown variant") {
                    if let Some(first_part) = error_msg.split(',').next() {
                        error!("Failed to parse snapshot entry: {}\n{}", first_part, &line);
                    }
                } else {
                    error!("Failed to parse snapshot entry: {}\n{}", &e, &line);
                }
            }
        }
    }
    None
}
