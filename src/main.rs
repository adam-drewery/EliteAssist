mod files;
mod events;

use crate::events::EliteEvent;
use serde::Deserialize;
use std::{
    fs::OpenOptions,
    io,
    io::{BufRead, BufReader, Seek, SeekFrom},
    path::Path,
    thread,
    time::Duration,
};

fn main() -> io::Result<()> {
    let journal_directory = "/home/adam/.steam/steam/steamapps/compatdata/359320/pfx/drive_c/users/steamuser/Saved Games/Frontier Developments/Elite Dangerous/";
    let dir_path = Path::new(journal_directory);
    let mut current_journal_path = files::get_latest_journal_path(dir_path)?;
    let file = OpenOptions::new().read(true).open(&current_journal_path)?;
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::Start(0))?;

    println!("Monitoring Elite Dangerous journal at: {}", current_journal_path.display());
    println!("Press Ctrl+C to stop.\n");

    loop {
        let mut buffer = String::new();
        let bytes_read = reader.read_line(&mut buffer)?;

        if bytes_read > 0 {
            let event: EliteEvent = serde_json::from_str(buffer.as_str())?;
            
            match event {
                EliteEvent::FileHeader(header) => {
                    println!("{}", header);
                }
                EliteEvent::Commander(commander) => {
                    println!("Commander name: {}", commander.name)
                }
                EliteEvent::Materials { .. } => {}
                EliteEvent::Rank { .. } => {}
                EliteEvent::Progress { .. } => {}
                EliteEvent::Reputation { .. } => {}
                EliteEvent::EngineerProgress { .. } => {}
                EliteEvent::SquadronStartup { .. } => {}
                EliteEvent::LoadGame { .. } => {}
                EliteEvent::Statistics { .. } => {}
                EliteEvent::ReceiveText { .. } => {}
                EliteEvent::Location { .. } => {}
                EliteEvent::Powerplay { .. } => {}
                EliteEvent::Music { .. } => {}
                EliteEvent::SuitLoadout { .. } => {}
                EliteEvent::Backpack { .. } => {}
                EliteEvent::ShipLocker { .. } => {}
                EliteEvent::Missions { .. } => {}
                EliteEvent::Shutdown { .. } => {}
            }
            
        } else {
            if let Ok(latest_path) = files::get_latest_journal_path(dir_path) {
                if latest_path != current_journal_path {
                    println!("\nNewer log file detected! Switching to: {}\n", latest_path.display());
                    current_journal_path = latest_path;
                    let new_file = OpenOptions::new().read(true).open(&current_journal_path)?;
                    reader = BufReader::new(new_file);
                    reader.seek(SeekFrom::Start(0))?;
                }
            }

            thread::sleep(Duration::from_millis(500));
        }
    }
}
