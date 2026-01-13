use crate::message::Message;
use crate::state::*;
use iced::Task;
use crate::{ardent, edsm};
use crate::message::Query::*;

#[derive(Clone, Debug)]
pub enum Query {
    StationsQueried(edsm::stations::Stations),
    NearbySystemsQueried(Vec<ardent::NearbySystem>),
    BodiesQueried(edsm::bodies::Bodies),
    TrafficQueried(edsm::traffic::Traffic),
    DeathsQueried(edsm::deaths::Deaths),

    // EDSM status updates
    EdsmServerStatus(edsm::ServerStatus),
}

impl Query {
    pub fn update(self, state: &mut State) -> Task<Message> {
        match self {
            
            StationsQueried(response) => state.location.stations = response.into(),
            
            TrafficQueried(traffic) => state.location.traffic = Some(traffic.into()),
            
            DeathsQueried(deaths) => state.location.deaths = Some(deaths.into()),
            
            EdsmServerStatus(status) => state.edsm_server_status = Some(status.into()),
            
            NearbySystemsQueried(systems) => {
                state.location.nearby_systems = systems.into_iter().map(|s| s.into()).collect();
            }

            BodiesQueried(response) => {
                if let Some(queried_bodies) = response.bodies && let Some (system_id) = response.id64 {
                    for queried_body in queried_bodies.into_iter() {
                        let system_scan = state.system_scans.entry(system_id).or_default();
                        let body = system_scan.bodies.entry(queried_body.body_id as u8).or_default();
                        body.update_from_query(queried_body);
                    }
                }
                
            }
        }

        Task::none()
    }
}
