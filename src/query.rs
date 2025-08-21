use iced::Task;
use log::{info, warn};
use crate::ardent::ArdentClient;
use crate::gui::Message;
use crate::edsm::EdsmClient;
use std::sync::LazyLock;

pub fn system(star_system: String, radius: f32) -> Task<Message> {
    static EDSM: LazyLock<EdsmClient> = LazyLock::new(|| EdsmClient::default());
    static ARDENT: LazyLock<ArdentClient> = LazyLock::new(|| ArdentClient::default());

    info!("Querying system: {}", star_system);

    macro_rules! fetch {
            ($method:ident, $Msg:ident, $label:literal) => {{
                let name = star_system.clone();
                Task::perform(async move {
                    match EDSM.$method(name.as_str()).await {
                        Ok(v) => Message::$Msg(v),
                        Err(error) => { warn!("Failed to fetch {}: {}", $label, error); Message::Empty }
                    }
                }, |m| m)
            }};
            ($method:ident, $arg:expr, $Msg:ident, $label:literal) => {{
                let name = star_system.clone();
                Task::perform(async move {
                    match EDSM.$method(name.as_str(), $arg).await {
                        Ok(v) => Message::$Msg(v),
                        Err(error) => { warn!("Failed to fetch {}: {}", $label, error); Message::Empty }
                    }
                }, |m| m)
            }};
        }

    Task::batch(vec![
        fetch!(get_bodies, BodiesQueried, "bodies"),
        fetch!(get_stations, StationsQueried, "stations"),
        fetch!(get_traffic, TrafficQueried, "traffic"),
        fetch!(get_deaths, DeathsQueried, "deaths"),
        Task::perform(async move {
            let nearby_systems = ARDENT.get_nearby_systems(star_system.as_str(), Some(radius))
                .await
                .map(|systems| Message::NearbySystemsQueried(systems))
                .unwrap_or_else(|error| {
                    warn!("Failed to fetch nearby systems: {}", error);
                    Message::Empty
                });
            nearby_systems
        }, |m| m)
    ])
}