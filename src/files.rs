use std::io;
use std::path::{Path, PathBuf};

/// Return the path to the newest `.log` file in `journal_directory`.
pub fn get_latest_journal_path(dir: &Path) -> io::Result<PathBuf> {
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