use crate::event::JournalEvent;
use crate::gui::Message;
use crate::state::State;
use log::{debug, error, info};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::select;
use tokio::sync::mpsc;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JournalError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("File watcher error: {0}")]
    Watcher(#[from] notify::Error),

    #[error("Channel error")]
    Channel,

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Failed to get home directory")]
    HomeDirectoryNotFound,
}


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
    watcher_tx: mpsc::Sender<()>,
    watcher_rx: mpsc::Receiver<()>,
    journal_files: Vec<PathBuf>,
    current_file_index: usize,
}

pub fn get_journal_directory() -> PathBuf {
    const JOURNAL_DIRECTORY: &str = ".steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/";
    let home = std::env::var("HOME").expect("Failed to get HOME directory");
    Path::new(&home).join(JOURNAL_DIRECTORY)
}

impl JournalWatcher {
    pub fn new() -> Self {
        let dir_path = get_journal_directory();

        // Get journal files, handling the case where they don't exist yet
        let journal_files = get_journal_paths(dir_path.as_path()).unwrap_or_else(|e| {
            error!("Error getting journal paths: {}", e);
            Vec::new()
        });

        // Initialize reader and current path to tail the newest file
        let (reader, current_journal_path, current_file_index) = if !journal_files.is_empty() {
            let idx = journal_files.len() - 1;
            let path = journal_files[idx].clone();
            match OpenOptions::new().read(true).open(&path) {
                Ok(file) => {
                    let mut reader = BufReader::new(file);
                    if let Err(e) = reader.seek(SeekFrom::End(0)) {
                        error!("Failed to seek to end of journal file: {}", e);
                    }
                    (Some(reader), path, idx)
                }
                Err(e) => {
                    error!("Failed to open journal file {}: {}", path.display(), e);
                    (None, path, idx)
                }
            }
        } else {
            info!("No journal files found. Waiting for files to be created...");
            (None, dir_path.to_path_buf(), 0)
        };


        let (watcher_tx, watcher_rx) = mpsc::channel(32);

        let watcher = Self {
            reader,
            current_journal_path,
            watcher_tx,
            watcher_rx,
            journal_files,
            current_file_index,
        };

        watcher.spawn_watcher();
        watcher
    }

    fn spawn_watcher(&self) {
        let tx = self.watcher_tx.clone();
        let target_dir = get_journal_directory();
        spawn_dir_watcher(tx, target_dir);
    }

    pub async fn next(&mut self) -> Message {
        loop {
            // Check if we have a reader
            if let Some(reader) = &mut self.reader {
                // Try to read next line from current file
                let mut buffer = String::new();
                match reader.read_line(&mut buffer) {
                    Ok(bytes_read) if bytes_read > 0 => {
                        let line = buffer.as_str();

                        debug!("Journal file updated: {}", &line);
                        let deserialize_result = serde_json::from_str::<JournalEvent>(line);

                        if let Ok(event) = deserialize_result {
                            return Message::JournalEvent(event);
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
                        // Reached end of file on current journal; wait for filesystem notification
                        // The watcher will trigger when the file is appended or a new file is created.
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
                    // Check for new journal files
                    let journal_dir = get_journal_directory();
                    let dir_path = journal_dir.as_path();
                    match get_journal_paths(dir_path) {
                        Ok(journal_files) => {
                            if !journal_files.is_empty() {
                                let increased = journal_files.len() > self.journal_files.len();
                                let had_none = self.reader.is_none();
                                if increased {
                                    info!("New journal file detected, switching to newest file");
                                    self.journal_files = journal_files;
                                    self.current_file_index = self.journal_files.len() - 1;
                                    self.current_journal_path = self.journal_files[self.current_file_index].clone();
                                    match OpenOptions::new().read(true).open(&self.current_journal_path) {
                                        Ok(file) => {
                                            let mut reader = BufReader::new(file);
                                            if let Err(e) = reader.seek(SeekFrom::Start(0)) {
                                                error!("Failed to seek in new journal file: {}", e);
                                            }
                                            self.reader = Some(reader);
                                        }
                                        Err(e) => {
                                            error!("Failed to open journal file {}: {}", self.current_journal_path.display(), e);
                                            self.reader = None;
                                        }
                                    }
                                } else if had_none {
                                    // No new files, but we didn't have a reader: reopen newest and seek to end
                                    self.journal_files = journal_files;
                                    self.current_file_index = self.journal_files.len() - 1;
                                    self.current_journal_path = self.journal_files[self.current_file_index].clone();
                                    match OpenOptions::new().read(true).open(&self.current_journal_path) {
                                        Ok(file) => {
                                            let mut reader = BufReader::new(file);
                                            if let Err(e) = reader.seek(SeekFrom::End(0)) {
                                                error!("Failed to seek end of journal file: {}", e);
                                            }
                                            self.reader = Some(reader);
                                        }
                                        Err(e) => {
                                            error!("Failed to open journal file {}: {}", self.current_journal_path.display(), e);
                                            self.reader = None;
                                        }
                                    }
                                } else {
                                    // Same set of files: the current one may have been appended.
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

/// Return the list of `.log` files in `JOURNAL_DIRECTORY`, sorted by modified time (oldest first).
/// If the directory doesn't exist, returns an error.
fn get_journal_paths(dir: &Path) -> Result<Vec<PathBuf>, JournalError> {
    // Check if directory exists; let the caller decide what to do if it doesn't
    if !dir.exists() {
        return Err(JournalError::DirectoryNotFound(dir.display().to_string()));
    }

    // Read the directory; propagate IO errors to the caller
    let read_dir = std::fs::read_dir(dir)?;

    let mut files: Vec<_> = read_dir
        .filter_map(|entry| {
            let entry = entry.ok()?; // Skip entries we can't read; defer hard errors to read_dir stage
            let path = entry.path();
            let is_log = path
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.eq_ignore_ascii_case("log"))
                .unwrap_or(false);

            if is_log { Some(path) } else { None }
        })
        .collect();

    // Sort files by modified time; if metadata lookup fails for a file, push it to the start
    files.sort_by_key(|path| {
        std::fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    });

    Ok(files)
}

fn check_snapshot_file(file_details: &mut FileDetails) -> Result<Option<JournalEvent>, JournalError> {
    // Check if the file exists
    if !file_details.path.exists() {
        return Ok(None);
    }

    // Get file metadata and modified time
    let metadata = std::fs::metadata(&file_details.path)?;
    let modified = metadata.modified()?;

    // Check if file has been modified since last check
    if modified > file_details.last_modified {
        let mut line = String::new();
        let mut file_reader = BufReader::new(File::open(&file_details.path)?);
        file_reader.read_to_string(&mut line)?;
        if line.is_empty() {
            return Ok(None);
        }

        file_details.last_modified = modified;
        info!(
            "Snapshot file updated: {:?}",
            &file_details.path.file_name().unwrap_or_default()
        );

        let event: JournalEvent = serde_json::from_str(&line)?;
        return Ok(Some(event));
    }

    Ok(None)
}

fn spawn_dir_watcher(tx: mpsc::Sender<()>, target_dir: PathBuf) {
    std::thread::spawn(move || {
        // If the target directory doesn't exist yet, watch its parent recursively so we catch its creation
        let (watch_path, mode) = if target_dir.exists() {
            (target_dir.clone(), RecursiveMode::NonRecursive)
        } else {
            let parent = target_dir.parent().unwrap_or_else(|| Path::new("/")).to_path_buf();
            (parent, RecursiveMode::Recursive)
        };
        let watch_path_for_closure = watch_path.clone();

        let mut watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                match res {
                    Ok(event) => {
                        let mut relevant = false;
                        for p in &event.paths {
                            if p == &watch_path_for_closure {
                                relevant = true;
                                break;
                            }
                            if let Some(ext) = p.extension().and_then(|e| e.to_str()) {
                                if ext.eq_ignore_ascii_case("json") || ext.eq_ignore_ascii_case("log") {
                                    relevant = true;
                                    break;
                                }
                            }
                        }
                        if relevant { let _ = tx.try_send(()); }
                    }
                    Err(e) => { error!("Notify watcher error: {}", e); let _ = tx.try_send(()); }
                }
            },
            Config::default(),
        ).expect("Failed to create file watcher");

        if let Err(e) = watcher.watch(&watch_path, mode) {
            error!("Failed to watch {}: {}", watch_path.display(), e);
        } else {
            info!("Watching for changes in: {}", watch_path.display());
        }

        // Keep the watcher and thread alive
        loop { std::thread::park_timeout(std::time::Duration::from_secs(3600)); }
    });
}

pub struct SnapshotWatcher {
    file: FileDetails,
    watcher_tx: mpsc::Sender<()>,
    watcher_rx: mpsc::Receiver<()>,
}

impl SnapshotWatcher {
    pub fn new(path: PathBuf) -> Self {
        let (watcher_tx, watcher_rx) = mpsc::channel(32);
        let watch_dir = path.parent().unwrap_or_else(|| Path::new(".")).to_path_buf();
        spawn_dir_watcher(watcher_tx.clone(), watch_dir);
        // Seed last_modified to current file's modified time to avoid emitting initial snapshot contents
        let mut file_details = FileDetails::new(path);
        if let Ok(meta) = std::fs::metadata(&file_details.path) {
            if let Ok(modified) = meta.modified() {
                file_details.last_modified = modified;
            }
        }
        Self {
            file: file_details,
            watcher_tx,
            watcher_rx,
        }
    }

    pub async fn next(&mut self) -> Message {
        loop {
            let _ = self.watcher_rx.recv().await;
            match check_snapshot_file(&mut self.file) {
                Ok(Some(event)) => return Message::JournalEvent(event),
                Ok(None) => {},
                Err(e) => error!("Snapshot check error: {}", e),
            }
        }
    }
}

// Loads historical journal and snapshot data
pub struct HistoryLoader {
    dir: PathBuf,
}

impl HistoryLoader {
    pub fn new() -> Self {
        Self { dir: get_journal_directory() }
    }

    fn read_all_journal_events(&self) -> Result<Vec<JournalEvent>, JournalError> {
        let mut events = Vec::new();
        let files = get_journal_paths(&self.dir)?;
        for path in files {
            let file = OpenOptions::new().read(true).open(&path)?;
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            loop {
                line.clear();
                let n = reader.read_line(&mut line)?;
                if n == 0 { break; }
                if line.trim().is_empty() { continue; }
                let ev: JournalEvent = serde_json::from_str(&line)?;
                events.push(ev);
            }
        }
        Ok(events)
    }

    fn read_snapshot_events(&self) -> Result<Vec<JournalEvent>, JournalError> {
        let names = [
            "Status.json",
            "Backpack.json",
            "Cargo.json",
            "ShipLocker.json",
            "Market.json",
            "NavRoute.json",
        ];
        let mut events = Vec::new();
        for name in names {
            let path = self.dir.join(name);
            if !path.exists() { continue; }
            let content = std::fs::read_to_string(&path)?;
            if content.trim().is_empty() { continue; }
            let ev: JournalEvent = serde_json::from_str(&content)?;
            events.push(ev);
        }
        Ok(events)
    }

    pub fn load_messages(&self) -> Vec<Message> {
        let journal_events = self.read_all_journal_events().unwrap_or_else(|e| {
            error!("Failed to load journal events: {}", e);
            Vec::new()
        });
        let snapshot_events = self.read_snapshot_events().unwrap_or_else(|e| {
            error!("Failed to load snapshot events: {}", e);
            Vec::new()
        });

        let mut msgs: Vec<Message> = journal_events
            .into_iter()
            .map(Message::JournalEvent)
            .collect();
        // Apply snapshots last to reflect current state
        msgs.extend(snapshot_events.into_iter().map(Message::JournalEvent));
        msgs.push(Message::JournalLoaded);
        msgs
    }

    pub fn load_state(&self) -> State {
        let mut state = State::default();
        // Build from events only; do not trigger JournalLoaded side-effects here
        if let Ok(events) = self.read_all_journal_events() {
            for ev in events.into_iter() {
                let _ = state.update_from(Message::JournalEvent(ev));
            }
        } else {
            error!("Failed to load journal events for state");
        }
        if let Ok(events) = self.read_snapshot_events() {
            for ev in events.into_iter() {
                let _ = state.update_from(Message::JournalEvent(ev));
            }
        } else {
            error!("Failed to load snapshot events for state");
        }
        state
    }
}
