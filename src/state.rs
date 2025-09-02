mod activity;
mod engineering;
mod market;
mod material;
mod chat_message;
mod mission;
mod navigation;
mod personal;
mod ship;
mod suit;
mod layout;
mod powerplay;
mod server_status;
mod fss;

pub use activity::*;
pub use engineering::*;
pub use layout::*;
pub use market::*;
pub use material::*;
pub use chat_message::*;
pub use mission::*;
pub use navigation::*;
pub use personal::*;
pub use powerplay::*;
pub use ship::*;
pub use suit::*;
pub use fss::*;

use crate::state::server_status::StatusDetails;
use serde::Deserialize;
use std::collections::HashMap;
use log::info;
use crate::config::Settings;

pub struct State {
    pub commander_name: Box<str>,
    pub credits: Box<str>,
    pub location: CurrentLocation,
    pub ship_locker: ShipLocker,
    pub ship_loadout: ShipLoadout,
    pub suit_loadout: SuitLoadout,
    pub active_screen: Screen,
    pub materials: Materials,
    pub messages: Vec<ChatMessage>,
    pub logs: Vec<GameEventLog>,
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
    pub edsm_server_status: Option<StatusDetails>,
    pub journal_loaded: bool,
    pub first_message_timestamp: i64,
    pub latest_message_timestamp: i64,
    pub latest_message_timestamp_formatted: Box<str>,

    pub layout: Layout,
    pub fss: FssState,

    // Data source selection
    pub data_source: DataSource,
    pub capi_enabled: bool,
    pub auth_in_progress: bool,
    pub auth_error: Option<Box<str>>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub enum Screen {
    #[default]
    Custom,
    Settings,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub enum DataSource {
    #[default]
    Unselected,
    Local,
    Capi,
}

impl Default for State {
    fn default() -> Self {
        let settings = Settings::load();
        let has_token = settings.as_ref().and_then(|s| s.capi_access_token.clone()).is_some();

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
            data_source: DataSource::Unselected,
            capi_enabled: has_token,
            auth_in_progress: false,
            auth_error: None,
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
                info!("DEBUG: trimming nav route to: {} AND {}", address_inclusive_to_trim, pos);
                self.nav_route.drain(0..=pos);
            }
        }
    }
}