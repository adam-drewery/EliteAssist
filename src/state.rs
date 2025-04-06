mod journal_entry;
mod market;
mod material;
mod message;
mod ship_locker;

pub use journal_entry::*;
pub use market::*;
pub use material::*;
pub use message::*;
pub use ship_locker::*;

use crate::event::Event;
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
    pub messages: Vec<ChatMessage>,
    pub journal: Vec<JournalEntry>,
    pub crime: CrimeStats,
    pub market: Market,
}

#[derive(Default)]
pub struct CrimeStats {
    pub legal_state: String,
    pub active_fine: bool,
    pub wanted: bool,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub enum ActiveScreen {
    #[default]
    Commander,
    ShipLocker,
    Navigation,
    Market,
    Materials,
    Messages,
}

impl State {
    pub fn update_from(&mut self, event: Event) {
        match event {
            Event::Commander(commander) => {
                self.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
            }

            Event::Materials(materials) => {
                if materials.is_empty() { return; }
                self.materials = materials.into();
            }

            Event::Location(location) => {
                self.current_system = location.star_system.clone();
                self.current_body = location.body.clone();
                //self.journal.push(location.into());
            }

            Event::ShipLocker(ship_locker) => {
                if ship_locker.is_empty() { return; }
                self.ship_locker = ship_locker.into();
            }

            Event::Status(status) => {
                if let Some(balance) = status.balance {
                    self.credits = balance.separate_with_commas() + " CR";
                }

                if let Some(legal_state) = status.legal_state {
                    self.crime.legal_state = legal_state;
                }

                if status.body_name.is_some() {
                    self.current_body = status.body_name.unwrap()
                }
            }

            Event::Disembark(disembark) => {
                self.current_body = disembark.body.clone();
                self.journal.push(disembark.into());
            }

            Event::Embark(embark) => {
                self.current_body = embark.body.clone();
                self.journal.push(embark.into());
            }

            Event::NavigateTo(screen) => {
                self.active_screen = screen;
            }

            Event::Docked(docked) => {
                if let Some(active_fine) = docked.active_fine {
                    self.crime.active_fine = active_fine;
                }
                if let Some(wanted) = docked.wanted {
                    self.crime.wanted = wanted;
                }
            }

            Event::ReceiveText(text) => {
                self.messages.push(text.into());
            }

            Event::Market(market) => {
                if market.items.is_none() {
                    return;
                }
                self.market = market.into();
            }
            _ => {}
        }
    }
}
