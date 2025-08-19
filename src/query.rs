use iced::Task;
use log::{info, warn};
use crate::gui::Message;
use crate::edsm::EdsmClient;

pub fn system(star_system: &String) -> Task<Message> {

    info!("Querying system: {}", star_system);

    macro_rules! fetch {
            ($method:ident, $Msg:ident, $label:literal) => {{
                let name = star_system.clone();
                Task::perform(async move {
                    let client = EdsmClient::default();
                    match client.$method(name.as_str()).await {
                        Ok(v) => Message::$Msg(v),
                        Err(error) => { warn!("Failed to fetch {}: {}", $label, error); Message::Empty }
                    }
                }, |m| m)
            }};
            ($method:ident, $arg:expr, $Msg:ident, $label:literal) => {{
                let name = star_system.clone();
                Task::perform(async move {
                    let client = EdsmClient::default();
                    match client.$method(name.as_str(), $arg).await {
                        Ok(v) => Message::$Msg(v),
                        Err(error) => { warn!("Failed to fetch {}: {}", $label, error); Message::Empty }
                    }
                }, |m| m)
            }};
        }

    Task::batch(vec![
        fetch!(get_bodies, BodiesQueried, "bodies"),
        fetch!(get_stations, StationsQueried, "stations"),
        fetch!(get_factions, FactionsQueried, "factions"),
        fetch!(get_traffic, TrafficQueried, "traffic"),
        fetch!(get_deaths, DeathsQueried, "deaths"),
    ])
}