use crate::events::{EliteEvent, ShipLocker};
use serde::Deserialize;
use thousands::Separable;

#[derive(Default)]
pub struct State {
    pub commander_name: String,
    pub credits: String,
    pub current_system: String,
    pub current_body: String,
    pub ship_locker: ShipLocker,
    pub active_screen: ActiveScreen
}

#[derive(Deserialize, Default, Clone, Debug)]
pub enum ActiveScreen {

    Commander,

    #[default]
    ShipLocker,

    Navigation,

    Market,
}

impl State {
    pub fn update_from(&mut self, event: EliteEvent) {
        match event {
            EliteEvent::FileHeader(_) => {}
            EliteEvent::Commander(commander) => {
                self.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
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
            EliteEvent::Location(location) => {
                self.current_system = location.star_system;
            }
            EliteEvent::Powerplay(_) => {}
            EliteEvent::Music(_) => {}
            EliteEvent::SuitLoadout(_) => {}
            EliteEvent::Backpack(_) => {}
            EliteEvent::ShipLocker(ship_locker) => {
                self.ship_locker = ship_locker;
            }
            EliteEvent::Missions(_) => {}
            EliteEvent::Shutdown(_) => {}
            EliteEvent::Loadout(_) => {}
            EliteEvent::BuyAmmo(_) => {}
            EliteEvent::RestockVehicle(_) => {}
            EliteEvent::BuyMicroResources(_) => {}
            EliteEvent::Status(status) => {
                self.credits = status.balance.separate_with_commas() + " CR";

                if status.body_name.is_some() {
                    self.current_body = status.body_name.unwrap()
                }
            }
            EliteEvent::Disembark(disembark) => {
                self.current_body = disembark.body;
            }
            EliteEvent::Embark(embark) => {
                self.current_body = embark.body;
            }
            EliteEvent::NpcCrewPaidWage(_) => {}
            EliteEvent::Cargo(_) => {}
            EliteEvent::Market(_) => {}
            
            EliteEvent::NavigateTo(screen) => {
                self.active_screen = screen;
            }

            EliteEvent::None => {},
        }
    }
}