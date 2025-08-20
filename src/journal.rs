//!
use crate::gui::Message;
use log::{error, info};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use tokio::sync::mpsc;
use thiserror::Error;
pub(crate) use crate::journal::event::Event;

pub mod event;
pub mod format;

///
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

    #[error("Failed to get home directory")]
    HomeDirectoryNotFound,
}


/// A structure that represents details about a file, including its path and last modification time.
///
/// # Fields
///
/// * `path` - A `PathBuf` that holds the file's path. This allows for platform-independent
///            representation and manipulation of file system paths.
/// * `last_modified` - A `SystemTime` value representing the last time the file was modified.
///
/// # Example
///
/// ```
/// use std::path::PathBuf;
/// use std::time::{SystemTime, UNIX_EPOCH};
///
/// let file_details = FileDetails {
///     path: PathBuf::from("/example/path/file.txt"),
///     last_modified: SystemTime::now(),
/// };
///
/// println!("File path: {:?}", file_details.path);
/// println!("Last modified: {:?}", file_details.last_modified);
/// ```
struct FileDetails {
    path: PathBuf,
    last_modified: SystemTime,
}

impl FileDetails {
    /// Creates a new instance of the struct with the given file path.
    ///
    /// # Arguments
    ///
    /// * `path` - A `PathBuf` representing the file path to associate with the instance.
    ///
    /// # Returns
    ///
    /// Returns a new instance of the struct, initialized with the provided file path.
    /// The `last_modified` field is set to the UNIX epoch (`SystemTime::UNIX_EPOCH`) by default.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::path::PathBuf;
    /// use std::time::SystemTime;
    ///
    /// let file_path = PathBuf::from("./example.txt");
    /// let instance = YourStruct::new(file_path);
    /// assert_eq!(instance.last_modified, SystemTime::UNIX_EPOCH);
    /// ```
    fn new(path: PathBuf) -> Self {
        Self {
            path,
            last_modified: SystemTime::UNIX_EPOCH,
        }
    }
}

/// The `JournalWatcher` struct is designed to monitor and handle reading from a series of journal files.
/// It provides functionality to manage a list of journal files, maintain the state of which file
/// is currently being read, and enables communication mechanisms for signaling changes or updates.
///
/// # Fields
///
/// * `reader` - An optional buffered reader (`BufReader<File>`) for reading the current journal file.
///   It is initialized when a file is being processed and set to `None` when no file is being read.
///
/// * `current_journal_path` - The path to the current journal file being processed. This helps track
///   the specific journal file currently in use.
///
/// * `watcher_tx` - A sender channel (`mpsc::Sender<()>`), part
pub struct JournalWatcher {
    reader: Option<BufReader<File>>,
    current_journal_path: PathBuf,
    watcher_tx: mpsc::Sender<()>,
    watcher_rx: mpsc::Receiver<()>,
    journal_files: Vec<PathBuf>,
    current_file_index: usize,
}

/// Returns the path to the Elite Dangerous journal directory as a `PathBuf`.
///
/// On Windows, this resolves to:
///   %USERPROFILE%\\Saved Games\\Frontier Developments\\Elite Dangerous\\
/// On Linux (Steam/Proton), this resolves to:
///   $HOME/.steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/
///
/// The function uses the user's home/profile directory per OS and appends the expected relative location.
pub fn get_journal_directory() -> Result<PathBuf, JournalError> {
    // Determine the Elite Dangerous journal directory consistently by using the
    // user's home/profile directory for each OS and appending the expected
    // relative location.
    //
    // - Linux (Steam/Proton): $HOME + proton prefix + Windows-like Saved Games path
    // - Windows: %USERPROFILE% + Saved Games\\Frontier Developments\\Elite Dangerous
    #[cfg(target_os = "windows")]
    {
        const JOURNAL_DIRECTORY: &str = "Saved Games\\Frontier Developments\\Elite Dangerous\\";
        let user_profile = std::env::var("USERPROFILE").map_err(|_| JournalError::HomeDirectoryNotFound)?;
        Ok(Path::new(&user_profile).join(JOURNAL_DIRECTORY))
    }

    #[cfg(not(target_os = "windows"))]
    {
        const JOURNAL_DIRECTORY: &str = ".steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/";
        let home = std::env::var("HOME").map_err(|_| JournalError::HomeDirectoryNotFound)?;
        Ok(Path::new(&home).join(JOURNAL_DIRECTORY))
    }
}

impl JournalWatcher {
    ///
    pub fn new() -> Result<Self, JournalError> {
        let dir_path = get_journal_directory()?;

        // Get journal files (empty is OK)
        let journal_files = get_journal_paths(dir_path.as_path())?;

        // Initialize reader and current path to tail the newest file
        let (reader, current_journal_path, current_file_index) = if !journal_files.is_empty() {
            let idx = journal_files.len() - 1;
            let path = journal_files[idx].clone();
            let file = OpenOptions::new().read(true).open(&path)?;
            let mut reader = BufReader::new(file);
            reader.seek(SeekFrom::End(0))?;
            (Some(reader), path, idx)
        } else {
            info!("No journal files were found. Waiting for files to be created...");
            (None, dir_path.clone(), 0)
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

        watcher.spawn_watcher()?;
        Ok(watcher)
    }

    /// Spawns a directory watcher to monitor a target directory for changes.
    ///
    /// This function initializes and launches a directory watcher by cloning the
    /// existing watcher transmitter (`watcher_tx`) and determining the target
    /// directory to be monitored using the `get_journal_directory` function.
    /// The `spawn_dir_watcher` function is then called with the cloned transmitter
    /// and the directory path to begin monitoring.
    ///
    /// # Details
    /// - `self.watcher_tx`: A transmitter channel used to send notifications about
    ///   directory changes. The function clones this transmitter to ensure it is
    ///   shared safely across threads.
    /// - `get_journal_directory()`: A function that retrieves the path of the target
    ///   directory that needs to be monitored.
    /// - `spawn_dir_watcher(tx, target_dir)`: A utility function invoked to start
    ///   the directory watcher
    fn spawn_watcher(&self) -> Result<(), JournalError> {
        let tx = self.watcher_tx.clone();
        let target_dir = get_journal_directory()?;
        spawn_dir_watcher(tx, target_dir);
        Ok(())
    }

    /// Asynchronously reads and processes journal messages.
    ///
    /// This function operates within an event loop, continuously monitoring for updates to journal files.
    /// - If a journal file is being actively read and contains a new line, the line is parsed as a JSON-encoded `JournalEvent`.
    /// - If the JSON deserialization succeeds, a `Message::JournalEvent` is returned.
    /// - If deserialization fails, appropriate error messages are logged. If the deserialization error indicates an unknown variant, a more detailed message is logged.
    ///
    /// The function also handles the following cases:
    /// - End of the currently monitored journal file:
    ///     - Wait for filesystem notifications to detect changes, such as updates to the existing file or the addition of new files.
    /// - Errors during reading:
    ///     - Logs the error and resets the reader state.
    ///
    /// When a filesystem notification is received:
    /// - The directory containing
    pub async fn next(&mut self) -> Result<Message, JournalError> {
        loop {
            // Check if we have a reader
            if let Some(reader) = &mut self.reader {
                // Try to read the next line from the current file
                let mut buffer = String::new();
                let bytes_read = reader.read_line(&mut buffer)?;
                if bytes_read > 0 {
                    let line = buffer.as_str();
                    info!("Journal file updated: {}", &line);
                    let event: Event = serde_json::from_str(line)?;
                    return Ok(Message::JournalEvent(event));
                }
            }

            // Wait for filesystem notification
            self.watcher_rx.recv().await.ok_or(JournalError::Channel)?;

            // Check for new or updated journal files
            let journal_dir = get_journal_directory()?;
            let dir_path = journal_dir.as_path();
            let journal_files = get_journal_paths(dir_path)?;
            if !journal_files.is_empty() {
                let increased = journal_files.len() > self.journal_files.len();
                let had_none = self.reader.is_none();
                if increased {
                    info!("New journal file detected, switching to the newest file");
                    self.journal_files = journal_files;
                    self.current_file_index = self.journal_files.len() - 1;
                    self.current_journal_path = self.journal_files[self.current_file_index].clone();
                    let file = OpenOptions::new().read(true).open(&self.current_journal_path)?;
                    let mut reader = BufReader::new(file);
                    reader.seek(SeekFrom::Start(0))?;
                    self.reader = Some(reader);
                } else if had_none {
                    // No new files, but we didn't have a reader: reopen newest and seek to end
                    self.journal_files = journal_files;
                    self.current_file_index = self.journal_files.len() - 1;
                    self.current_journal_path = self.journal_files[self.current_file_index].clone();
                    let file = OpenOptions::new().read(true).open(&self.current_journal_path)?;
                    let mut reader = BufReader::new(file);
                    reader.seek(SeekFrom::End(0))?;
                    self.reader = Some(reader);
                } else {
                    // Same set of files: the current one may have been appended.
                    self.journal_files = journal_files;
                }
            }
        }
    }
}

///
fn get_journal_paths(dir: &Path) -> Result<Vec<PathBuf>, JournalError> {
    // Check if a directory exists; let the caller decide what to do if it doesn't
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

/// Checks the provided snapshot file for updates and parses its content into a `JournalEvent` if applicable.
///
/// # Arguments
///
/// * `file_details` - A mutable reference to a `FileDetails` struct that contains metadata about the file,
///   including its path and the timestamp of its last modification.
///
/// # Returns
///
/// * `Result<Option<JournalEvent>, JournalError>`:
///   - `Ok(Some(JournalEvent))`: If the file was updated, and a valid `JournalEvent` was created after parsing its content.
///   - `Ok(None)`: If the file does not exist, is empty, or has not been modified since the last check.
///   - `Err(JournalError)`: If an error occurred during file operations, parsing, or metadata retrieval.
///
/// # Behavior
///
/// 1. The function checks whether the file is specified in `
fn check_snapshot_file(file_details: &mut FileDetails) -> Result<Option<Event>, JournalError> {
    // Check if the file exists
    if !file_details.path.exists() {
        return Ok(None);
    }

    // Get file metadata and modified time
    let metadata = std::fs::metadata(&file_details.path)?;
    let modified = metadata.modified()?;

    // Check if a file has been modified since the last check
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

        let event: Event = serde_json::from_str(&line)?;
        return Ok(Some(event));
    }

    Ok(None)
}

///
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
        ).expect("Failed to create a file watcher");

        if let Err(e) = watcher.watch(&watch_path, mode) {
            error!("Failed to watch {}: {}", watch_path.display(), e);
        } else {
            info!("Watching for changes in: {}", watch_path.file_name().unwrap_or_default().to_str().unwrap_or_default());
        }

        // Keep the watcher and thread alive
        loop { std::thread::park_timeout(std::time::Duration::from_secs(3600)); }
    });
}

///
pub struct SnapshotWatcher {
    file: FileDetails,
    watcher_tx: mpsc::Sender<()>,
    watcher_rx: mpsc::Receiver<()>,
}

impl SnapshotWatcher {
    /// Creates a new instance of the containing struct, initializing the components necessary
    pub fn new(path: PathBuf) -> Self {
        let (watcher_tx, watcher_rx) = mpsc::channel(32);
        // Watch the specific file; if it doesn't exist yet, spawn_dir_watcher will watch the parent recursively
        spawn_dir_watcher(watcher_tx.clone(), path.clone());
        // Seed last_modified to the current file's modified time to avoid emitting initial snapshot contents
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

    /// Asynchronously retrieves the next `Message` from the stream by observing changes in a snapshot file.
    ///
    /// This method continuously listens for updates from the given `watcher_rx` channel and evaluates the
    /// snapshot file for events using the `check_snapshot_file` function. When a journal event is successfully
    /// found, it constructs and returns a `Message::JournalEvent` containing the event data.
    ///
    /// # Behavior
    /// - Listens indefinitely for events from the `watcher_rx` channel.
    /// - Invokes `check_snapshot_file` to analyze the state of the snapshot file.
    /// - If an event is found, the method returns `Message::JournalEvent` wrapping the event.
    /// - If no event is found (`Ok(None)`), the loop continues.
    /// - Logs an error if `check_snapshot_file` encounters an error.
    ///
    /// # Returns
    ///
    pub async fn next(&mut self) -> Result<Message, JournalError> {
        loop {
            self.watcher_rx.recv().await.ok_or(JournalError::Channel)?;
            if let Some(event) = check_snapshot_file(&mut self.file)? {
                return Ok(Message::JournalEvent(event));
            }
        }
    }
}

/// `HistoryLoader` is a struct designed to manage and facilitate
/// the loading of historical data from a specified directory.
///
/// # Fields
///
/// * `dir` - A `PathBuf` that specifies the directory from which
/// historical data will be loaded. This field points to the location
/// containing relevant files or resources associated with the data.
///
/// # Example
///
/// ```rust
/// use std::path::PathBuf;
/// use your_crate::HistoryLoader;
///
/// let loader = HistoryLoader {
///     dir: PathBuf::from("/path/to/history"),
/// };
/// ```
///
/// The `HistoryLoader` struct can be extended with additional methods
/// or functionality to retrieve and process files from the specified directory.
pub struct HistoryLoader {
    dir: PathBuf,
}

impl HistoryLoader {
    ///
    pub fn new() -> Result<Self, JournalError> {
        Ok(Self { dir: get_journal_directory()? })
    }

    /// Reads all journal events from files located in the directory specified by `self.dir`.
    ///
    /// This function iterates through all the files in the journal directory, reads each file line-by-line,
    /// and attempts to deserialize each non-empty line into a `JournalEvent` object using `serde_json`.
    /// Successfully deserialized events are collected into a vector, which is returned upon completion.
    ///
    /// # Returns
    ///
    /// * `Ok(Vec<JournalEvent>)` - A vector containing all successfully read and deserialized journal events.
    /// * `Err(JournalError)` - An error occurred while retrieving journal paths, opening files,
    ///   reading lines, or deserializing a line into
    fn read_all_journal_events(&self) -> Result<Vec<Event>, JournalError> {
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
                let ev: Event = serde_json::from_str(&line)?;
                events.push(ev);
            }
        }
        Ok(events)
    }

    ///
    /// Reads snapshot event files from a specified directory and parses
    fn read_snapshot_events(&self) -> Result<Vec<Event>, JournalError> {
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
            let ev: Event = serde_json::from_str(&content)?;
            events.push(ev);
        }
        Ok(events)
    }

    /// Loads all messages by aggregating journal events and snapshot events, ensuring that
    /// snapshots are applied last to reflect the most up-to-date state.
    ///
    /// This function combines events retrieved from the journal and snapshots, processes
    /// them into `Message` objects, and appends a final `Message::JournalLoaded` to
    /// indicate the completion of the operation. Any errors encountered during the retrieval
    /// of journal or snapshot events are logged, and an empty event list is substituted.
    ///
    /// # Returns
    ///
    /// A `Vec<Message>` containing:
    /// - Transformed journal events as `Message::JournalEvent`s.
    /// - Transformed snapshot events as `Message::JournalEvent`s.
    /// - A final `Message::JournalLoaded` to signify successful loading.
    ///
    /// # Error Handling
    ///
    /// If an error occurs when fetching journal or snapshot events, it logs the error and
    ///
    pub fn load_messages(&self) -> Result<Vec<Message>, JournalError> {
        let journal_events = self.read_all_journal_events()?;
        let snapshot_events = self.read_snapshot_events()?;

        let mut msgs: Vec<Message> = journal_events
            .into_iter()
            .map(Message::JournalEvent)
            .collect();
        // Apply snapshots last to reflect the current state
        msgs.extend(snapshot_events.into_iter().map(Message::JournalEvent));
        msgs.push(Message::JournalLoaded);
        Ok(msgs)
    }
}
