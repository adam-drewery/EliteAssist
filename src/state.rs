pub mod history;
pub mod engineering;
pub mod market;
pub mod material;
pub mod chat;
pub mod mission;
pub mod navigation;
pub mod personal;
pub mod ship;
pub mod suit;
pub mod layout;
pub mod powerplay;
pub mod server;
pub mod fss;

use history::*;
use chat::*;
use engineering::*;
use fss::*;
use layout::*;
use market::*;
use material::*;
use mission::*;
use navigation::*;
use personal::*;
use powerplay::*;

use crate::state::server::Status;
use serde::Deserialize;
use std::collections::HashMap;

pub struct State {
    pub commander_name: Box<str>,
    pub credits: Box<str>,
    pub location: CurrentLocation,
    pub ship_locker: ship::Locker,
    pub ship_loadout: ship::Loadout,
    pub suit_loadout: suit::Loadout,
    pub active_screen: Screen,
    pub materials: Materials,
    pub messages: Vec<Message>,
    pub logs: Vec<Event>,
    pub crime: CrimeStats,
    pub market: Market,
    pub rank: Rank,
    pub reputation: Reputation,
    pub engineers: Vec<Engineer>,
    pub nav_route: Vec<NavRouteStep>,
    pub missions: Vec<Mission>,
    pub combat_bonds: HashMap<Box<str>, u32>,
    pub bounties: HashMap<Box<str>, u32>,
    pub discoveries: HashMap<Box<str>, u32>,
    pub progress: Rank,
    pub powerplay: Powerplay,
    pub edsm_server_status: Option<Status>,
    pub journal_loaded: bool,
    pub first_message_timestamp: i64,
    pub latest_message_timestamp: i64,
    pub latest_message_timestamp_formatted: Box<str>,

    pub layout: Layout,
    pub fss: Fss,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub enum Screen {
    #[default]
    Custom,
    Settings,
}

impl Default for State {
    fn default() -> Self {

        // Start with basic defaults for all fields
        let state = Self {
            commander_name: String::new().into(),
            credits: String::new().into(),
            location: Default::default(),
            ship_locker: Default::default(),
            ship_loadout: Default::default(),
            suit_loadout: Default::default(),
            active_screen: Default::default(),
            materials: Default::default(),
            messages: Vec::new(),
            logs: Vec::new(),
            crime: Default::default(),
            market: Default::default(),
            rank: Default::default(),
            reputation: Default::default(),
            engineers: Default::default(),
            nav_route: Vec::new(),
            missions: Vec::new(),
            combat_bonds: HashMap::new(),
            bounties: HashMap::new(),
            discoveries: HashMap::new(),
            progress: Default::default(),
            powerplay: Default::default(),
            edsm_server_status: None,
            journal_loaded: false,
            first_message_timestamp: 0,
            latest_message_timestamp: 0,
            latest_message_timestamp_formatted: String::new().into(),
            layout: Layout::from_settings(),
            fss: Default::default(),
        };

        state
    }
}

impl State {
    pub fn trim_nav_route(&mut self, address_inclusive_to_trim: u64) {
        if !self.nav_route.is_empty() {
            if let Some(pos) = self.nav_route
                .iter()
                .position(|step| step.system_address == address_inclusive_to_trim)
            {
                self.nav_route.drain(0..=pos);
            }
        }
    }
}