use iced::Task;
use log::{info, warn};
use crate::ardent::ArdentClient;
use crate::gui::Message;
use crate::edsm::EdsmClient;
use std::sync::LazyLock;

pub fn system(star_system: &str, radius: f32) -> Task<Message> {
    static EDSM: LazyLock<EdsmClient> = LazyLock::new(|| EdsmClient::default());
    static ARDENT: LazyLock<ArdentClient> = LazyLock::new(|| ArdentClient::default());

    info!("Querying system: {}", star_system);

    macro_rules! fetch_edsm {
            ($method:ident, $Msg:ident, $label:literal) => {{
                let name = star_system.to_string();
                Task::perform(async move {
                    match EDSM.$method(name.as_ref()).await {
                        Ok(v) => Message::$Msg(v),
                        Err(error) => { warn!("Failed to fetch {}: {}", $label, error); Message::Empty }
                    }
                }, |m| m)
            }};
            ($method:ident, $arg:expr, $Msg:ident, $label:literal) => {{
                let name = star_system.to_string();
                Task::perform(async move {
                    match EDSM.$method(name.as_ref(), $arg).await {
                        Ok(v) => Message::$Msg(v),
                        Err(error) => { warn!("Failed to fetch {}: {}", $label, error); Message::Empty }
                    }
                }, |m| m)
            }};
        }
    
    Task::batch(vec![
        fetch_edsm!(get_bodies, BodiesQueried, "bodies"),
        fetch_edsm!(get_stations, StationsQueried, "stations"),
        fetch_edsm!(get_traffic, TrafficQueried, "traffic"),
        fetch_edsm!(get_deaths, DeathsQueried, "deaths"),
        {
            let star_system = star_system.to_string();
            Task::perform(async move {
                let nearby_systems = ARDENT.get_nearby_systems(&star_system, Some(radius))
                    .await
                    .map(|systems| Message::NearbySystemsQueried(systems))
                    .unwrap_or_else(|error| {
                        warn!("Failed to fetch nearby systems: {}", error);
                        Message::Empty
                    });
                nearby_systems
            }, |m| m)
        }
    ])
}