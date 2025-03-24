mod event;

use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Seek, SeekFrom},
    thread,
    time::Duration,
};

fn main() -> std::io::Result<()> {
    let journal_directory = "~/.steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/";
    let journal_path = std::fs::read_dir(journal_directory)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "log")
                .unwrap_or(false)
        })
        .max_by_key(|entry| entry.metadata().unwrap().modified().unwrap())
        .map(|entry| entry.path())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "No log files found"))?;

    // Open the file in read-only mode, allowing appends by the game.
    let file = OpenOptions::new().read(true).open(&journal_path)?;
    let mut reader = BufReader::new(file);

    reader.seek(SeekFrom::Start(0))?;

    println!("Monitoring Elite Dangerous journal at: {}", journal_path.to_str().unwrap());
    println!("Press Ctrl+C to stop.\n");

    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer)?;

        if bytes_read > 0 {
            print!("New journal entry: {}", buffer);
        }

        // Sleep briefly before checking again. Adjust as needed.
        thread::sleep(Duration::from_millis(500));
    }
}
