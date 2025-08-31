use crate::message::Message;
use crate::state::*;
use iced::Task;
use crate::{ardent, edsm};

#[derive(Clone, Debug)]
pub enum Query {
    StationsQueried(edsm::Stations),
    NearbySystemsQueried(Vec<ardent::NearbySystem>),
    BodiesQueried(edsm::Bodies),
    TrafficQueried(edsm::Traffic),
    DeathsQueried(edsm::Deaths),

    // EDSM status updates
    EdsmServerStatus(edsm::ServerStatus),
}

impl Query {
    pub fn update(self, state: &mut State) -> Task<Message> {
        match self {
            Query::StationsQueried(response) => {
                state.location.stations = response.into();
            }
            Query::NearbySystemsQueried(systems) => {
                state.location.nearby_systems = systems.into_iter().map(|s| s.into()).collect();
            }
            Query::BodiesQueried(bodies) => {
                state.location.known_bodies = bodies.into();
            }
            Query::TrafficQueried(traffic) => {
                state.location.traffic = Some(traffic.into());
            }
            Query::DeathsQueried(deaths) => {
                state.location.deaths = Some(deaths.into());
            }
            Query::EdsmServerStatus(status) => {
                state.edsm_server_status = Some(status.into());
            }
        }

        Task::none()
    }
}
