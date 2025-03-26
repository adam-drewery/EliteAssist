mod files;
mod events;

use crate::events::EliteEvent;
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
            let line = buffer.as_str();
            println!("{}", line);
            
            let event: EliteEvent = serde_json::from_str(line)?;
            
            match event {
                EliteEvent::FileHeader(header) => {
                    println!("Language: {}", header.language);
                    println!("Game Version: {} ({})", header.game_version, header.build.trim_end());
                }
                EliteEvent::Commander(commander) => {
                    println!("Commander name: {}", commander.name)
                }
                EliteEvent::Materials(_) => {}
                EliteEvent::Rank(_) => {}
                EliteEvent::Progress(_) => {}
                EliteEvent::Reputation(_) => {}
                EliteEvent::EngineerProgress(_) => {}
                EliteEvent::SquadronStartup(_) => {}
                EliteEvent::LoadGame(_) => {}
                EliteEvent::Statistics(_) => {}
                EliteEvent::ReceiveText(_) => {}
                EliteEvent::Location(_) => {}
                EliteEvent::Powerplay(_) => {}
                EliteEvent::Music(_) => {}
                EliteEvent::SuitLoadout(_) => {}
                EliteEvent::Backpack(_) => {}
                EliteEvent::ShipLocker(_) => {}
                EliteEvent::Missions(_) => {}
                EliteEvent::Shutdown(_) => {}
                EliteEvent::Loadout(_) => {}
                EliteEvent::BuyAmmo(_) => {}
                EliteEvent::RestockVehicle(_) => {}
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
