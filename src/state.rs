use crate::event::{Event, Materials, ShipLocker};
use serde::Deserialize;
use thousands::Separable;

#[derive(Default)]
pub struct State {
    pub commander_name: String,
    pub credits: String,
    pub current_system: String,
    pub current_body: String,
    pub ship_locker: ShipLocker,
    pub active_screen: ActiveScreen,
    pub materials: Materials
}

#[derive(Deserialize, Default, Clone, Debug)]
pub enum ActiveScreen {

    Commander,

    #[default]
    ShipLocker,

    Navigation,

    Market,
    Materials,
}

impl State {
    pub fn update_from(&mut self, event: Event) {
        match event {
            Event::FileHeader(_) => {}
            Event::Commander(commander) => {
                self.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
            }
            Event::Materials(materials) => {
                self.materials = materials;
            }
            Event::Rank(_) => {}
            Event::Progress(_) => {}
            Event::Reputation(_) => {}
            Event::EngineerProgress(_) => {}
            Event::SquadronStartup(_) => {}
            Event::LoadGame(_) => {}
            Event::Statistics(_) => {}
            Event::ReceiveText(_) => {}
            Event::Location(location) => {
                self.current_system = location.star_system;
            }
            Event::Powerplay(_) => {}
            Event::Music(_) => {}
            Event::SuitLoadout(_) => {}
            Event::Backpack(_) => {}
            Event::ShipLocker(ship_locker) => {
                self.ship_locker = ship_locker;
            }
            Event::Missions(_) => {}
            Event::Shutdown(_) => {}
            Event::Loadout(_) => {}
            Event::BuyAmmo(_) => {}
            Event::RestockVehicle(_) => {}
            Event::BuyMicroResources(_) => {}
            Event::Status(status) => {
                self.credits = status.balance.separate_with_commas() + " CR";

                if status.body_name.is_some() {
                    self.current_body = status.body_name.unwrap()
                }
            }
            Event::Disembark(disembark) => {
                self.current_body = disembark.body;
            }
            Event::Embark(embark) => {
                self.current_body = embark.body;
            }
            Event::NpcCrewPaidWage(_) => {}
            Event::Cargo(_) => {}
            Event::Market(_) => {}
            
            Event::NavigateTo(screen) => {
                self.active_screen = screen;
            }

            Event::None => {},
        }
    }
}