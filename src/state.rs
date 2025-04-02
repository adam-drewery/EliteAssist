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
    pub materials: Materials,
    pub legal_state: String,
    pub active_fine : bool,
    pub wanted : bool,
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

            Event::Commander(commander) => {
                self.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
            }
            Event::Materials(materials) => {
                let empty = materials.encoded.is_empty() 
                    && materials.raw.is_empty() 
                    && materials.manufactured.is_empty();
                
                if empty { return; }
                self.materials = materials;
            }
            Event::Location(location) => {
                self.current_system = location.star_system;
            },
            Event::ShipLocker(ship_locker) => {

                if ship_locker.consumables.is_none()
                    && ship_locker.components.is_none()
                    && ship_locker.items.is_none()
                    && ship_locker.data.is_none() {
                    return;
                }
                
                self.ship_locker = ship_locker;
            }
            Event::Status(status) => {
                if let Some(balance) = status.balance {
                    self.credits = balance.separate_with_commas() + " CR";
                }
                
                if let Some(legal_state) = status.legal_state {
                    self.legal_state = legal_state;
                }

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
            
            Event::NavigateTo(screen) => {
                self.active_screen = screen;
            }

            Event::Docked(docked) => {
                if let Some(active_fine) = docked.active_fine {
                    self.active_fine = active_fine;
                }
                if let Some(wanted) = docked.wanted {
                    self.wanted = wanted;
                }
            },
            _ => {}
        }
    }
}