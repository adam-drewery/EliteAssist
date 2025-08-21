mod activity;
mod engineering;
mod market;
mod material;
mod message;
mod mission;
mod navigation;
mod personal;
mod ship;
mod suit;

pub use activity::*;
pub use engineering::*;
pub use market::*;
pub use material::*;
pub use message::*;
pub use mission::*;
pub use navigation::*;
pub use personal::*;
pub use ship::*;
pub use suit::*;

use crate::gui::Message;
use crate::journal;
use crate::journal::format;
use crate::query;
use iced::Task;
use iced::widget::pane_grid;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thousands::Separable;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PanelType {
    Loadout,
    Messages,
    Route,
    Location,
    ShipDetails,
    ShipModules,
    Ranks,
    Missions,
    Claims,
}

impl PanelType {
    pub const fn all() -> [PanelType; 9] {
        [
            PanelType::Loadout,
            PanelType::Messages,
            PanelType::Route,
            PanelType::Location,
            PanelType::ShipDetails,
            PanelType::ShipModules,
            PanelType::Ranks,
            PanelType::Missions,
            PanelType::Claims,
        ]
    }
    pub fn title(&self) -> &'static str {
        match self {
            PanelType::Loadout => "Loadout",
            PanelType::Messages => "Messages",
            PanelType::Route => "Route",
            PanelType::Location => "Location",
            PanelType::ShipDetails => "Ship",
            PanelType::ShipModules => "Ship Modules",
            PanelType::Ranks => "Ranks",
            PanelType::Missions => "Missions",
            PanelType::Claims => "Claims",
        }
    }

    pub fn default_enabled_vec() -> Vec<PanelType> {
        vec![
            PanelType::Loadout,
            PanelType::Messages,
            PanelType::Route,
            PanelType::Location,
            PanelType::ShipDetails,
            PanelType::ShipModules,
            PanelType::Ranks,
        ]
    }
}

pub struct State {
    pub overview_panes: Option<pane_grid::State<PanelType>>,
    pub show_settings_menu: bool,
    pub enabled_panels: Option<Vec<PanelType>>,
    pub commander_name: String,
    pub credits: String,
    pub current_system: String,
    pub current_body: String,
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
    pub engineers: EngineerProgress,
    pub nav_route: Vec<NavRouteStep>,
    pub missions: Vec<Mission>,
    pub combat_bonds: HashMap<String, i64>,
    pub bounties: HashMap<String, i64>,
    pub discoveries: HashMap<String, i64>,
    pub progress: Rank,

    pub journal_loaded: bool,
    pub first_message_timestamp: i64,
    pub latest_message_timestamp: i64,
    pub latest_message_timestamp_formatted: String
}

#[derive(Clone, Debug, Default, Deserialize)]
pub enum Screen {
    #[default]
    Commander,
    ShipLocker,
    Market,
    Materials,
    Messages,
}

impl Default for State {
    fn default() -> Self {
        // Start with basic defaults for all fields
        let mut s = Self {
            overview_panes: None,
            show_settings_menu: false,
            enabled_panels: None,
            commander_name: String::new(),
            credits: String::new(),
            current_system: String::new(),
            current_body: String::new(),
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
            journal_loaded: false,
            first_message_timestamp: 0,
            latest_message_timestamp: 0,
            latest_message_timestamp_formatted: String::new(),
        };

        // Attempt to load persisted settings and apply
        if let Some(settings) = crate::settings::Settings::load() {
            if let Some(layout) = &settings.layout {
                s.overview_panes = Some(crate::settings::build_panes_from_layout(layout));
                // If visible list not provided, derive from layout leaves
                s.enabled_panels = Some(settings.visible.unwrap_or_else(|| crate::settings::layout_leaf_panels(layout)));
            } else if let Some(visible) = settings.visible {
                s.enabled_panels = Some(visible);
            }
        }

        s
    }
}

impl State {
    fn default_overview_panes() -> pane_grid::State<PanelType> {
        let (mut panes, pane_1) = pane_grid::State::new(PanelType::Loadout);

        let Some((pane_2, split_1)) = panes.split(pane_grid::Axis::Vertical, pane_1, PanelType::Route) else { return panes; };
        let Some((pane_3, _split_2)) = panes.split(pane_grid::Axis::Vertical, pane_2, PanelType::ShipDetails) else { return panes; };

        let Some((_, split_3)) = panes.split(pane_grid::Axis::Horizontal, pane_1, PanelType::Messages) else { return panes; };
        let Some((_, split_4)) = panes.split(pane_grid::Axis::Horizontal, pane_1, PanelType::Ranks) else { return panes; };
        let Some((_, split_5)) = panes.split(pane_grid::Axis::Horizontal, pane_2, PanelType::Location) else { return panes; };
        let Some((_, split_6)) = panes.split(pane_grid::Axis::Horizontal, pane_3, PanelType::ShipModules) else { return panes; };

        // Set vertical splits so each column takes up 1/3 of the space
        panes.resize(split_1, 1.0f32 / 3.0f32);

        // Set horizontal splits 
        panes.resize(split_3, 0.66f32);
        panes.resize(split_4, 0.3f32);
        panes.resize(split_5, 0.6f32);
        panes.resize(split_6, 0.3f32);

        panes
    }

    fn build_panes_from(list: &Vec<PanelType>) -> pane_grid::State<PanelType> {
        // Fallback to default layout if list is empty
        if list.is_empty() {
            return Self::default_overview_panes();
        }
        let mut iter = list.iter();
        let first = iter.next().cloned().unwrap_or(PanelType::Loadout);
        let (mut panes, mut last_pane) = pane_grid::State::new(first);
        for panel in iter.cloned() {
            if let Some((new_pane, _split)) = panes.split(pane_grid::Axis::Vertical, last_pane, panel) {
                last_pane = new_pane;
            }
        }
        panes
    }

    // Helper: find the Pane that contains the given PanelType
    fn find_pane_with(panes: &pane_grid::State<PanelType>, target: &PanelType) -> Option<pane_grid::Pane> {
        // The iced::pane_grid::State exposes a `panes` field that can be iterated
        // We iterate by reference to avoid moving the internal state
        for (pane, content) in &panes.panes {
            if content == target {
                return Some(*pane);
            }
        }
        None
    }

    pub fn is_panel_enabled(&self, panel: &PanelType) -> bool {
        match &self.enabled_panels {
            Some(v) => v.contains(panel),
            None => PanelType::default_enabled_vec().contains(panel),
        }
    }
    pub fn update_from(&mut self, message: Message) -> Task<Message> {

        match message {

            Message::Empty => {}

            Message::NavigateTo(screen) => self.active_screen = screen,

            Message::StationsQueried(response) => {
                self.location.stations = response.into();
            }

            Message::NearbySystemsQueried(systems) => {
                self.location.nearby_systems = systems.into_iter().map(|s| s.into()).collect();
            }

            Message::BodiesQueried(bodies) => {
                self.location.known_bodies = bodies.into();
            }

            Message::TrafficQueried(traffic) => {
                self.location.traffic = Some(traffic.into());
            }

            Message::DeathsQueried(deaths) => {
                self.location.deaths = Some(deaths.into());
            }

            Message::PaneDragged(event) => {
                if let Some(panes) = &mut self.overview_panes {
                    match event {
                        pane_grid::DragEvent::Picked { .. } => {}
                        pane_grid::DragEvent::Dropped { pane, target } => {
                            panes.drop(pane, target);
                            let _ = crate::settings::Settings::save_from_state(self);
                        }
                        pane_grid::DragEvent::Canceled { .. } => {
                            // no-op on cancel
                        }
                    }
                }
            }

            Message::PaneResized(event) => {
                if let Some(panes) = &mut self.overview_panes {
                    panes.resize(event.split, event.ratio);
                    let _ = crate::settings::Settings::save_from_state(self);
                }
            }

            Message::ShowSettingsMenu(show) => {
                self.show_settings_menu = show;
            }

            Message::TogglePanel(panel, enabled) => {
                // Start from current enabled set (or all panels by default)
                let mut list: Vec<PanelType> = self
                    .enabled_panels
                    .clone()
                    .unwrap_or_else(|| PanelType::default_enabled_vec());

                let was_enabled = list.contains(&panel);
                let before_len = list.len();

                if enabled {
                    if !was_enabled {
                        list.push(panel.clone());
                    }
                } else {
                    // Prevent disabling the last remaining panel
                    if was_enabled && list.len() > 1 {
                        list.retain(|p| p != &panel);
                    }
                }
                // Keep deterministic order according to PanelType::all()
                let order = PanelType::all();
                list.sort_by_key(|p| order.iter().position(|q| q == p).unwrap_or(usize::MAX));

                let did_enable = enabled && !was_enabled;
                let did_disable = !enabled && was_enabled && list.len() < before_len;

                self.enabled_panels = Some(list.clone());

                // Mutate current layout instead of rebuilding to preserve existing splits
                if let Some(panes) = &mut self.overview_panes {
                    if did_enable {
                        // Insert the newly enabled panel by splitting an existing anchor pane
                        if let Some((&anchor, _)) = panes.panes.iter().next() {
                            let _ = panes.split(pane_grid::Axis::Horizontal, anchor, panel.clone());
                        }
                    } else if did_disable {
                        // Close the pane containing this panel, preserving other layout
                        if let Some(p) = Self::find_pane_with(panes, &panel) {
                            let _ = panes.close(p);
                        }
                    }
                }
                // Persist settings after visibility/layout changes
                let _ = crate::settings::Settings::save_from_state(self);
            }

            Message::JournalLoaded => {
                self.journal_loaded = true;
                if self.overview_panes.is_none() {
                    // Start from the default layout to preserve intended split structure
                    let mut panes = Self::default_overview_panes();
                    // If some panels are disabled, close them while keeping the rest of the layout
                    if let Some(enabled) = &self.enabled_panels {
                        let enabled_set: std::collections::HashSet<_> = enabled.iter().cloned().collect();
                        // Collect panes to close first to avoid borrowing issues
                        let to_close: Vec<_> = panes
                            .panes
                            .iter()
                            .filter_map(|(pane, content)| if !enabled_set.contains(content) { Some(*pane) } else { None })
                            .collect();
                        for p in to_close {
                            let _ = panes.close(p);
                        }
                    }
                    self.overview_panes = Some(panes);
                    // Persist the initialized layout so a settings file exists even before any manual changes
                    let _ = crate::settings::Settings::save_from_state(self);
                }

                if self.journal_loaded {
                    return query::system(
                        self.current_system.clone(),
                        self.ship_loadout.max_jump_range);
                }
            }

            Message::JournalEvent(event) => {
                use journal::Event;
                match event {

                    // BACKPACK
                    Event::Backpack(_) => {}
                    Event::BackpackChange(_) => {}
                    Event::DropItems(_) => {}
                    Event::CollectItems(_) => {}
                    Event::UseConsumable(_) => {}

                    // CARGO
                    Event::Cargo(_) => {}
                    Event::CargoTransfer(_) => {}
                    Event::CargoDepot(_) => {}
                    Event::CollectCargo(_) => {}
                    Event::EjectCargo(_) => {}

                    // CARRIER
                    Event::CarrierLocation(_) => {}
                    Event::CarrierJump(_) => {}
                    Event::CarrierBuy(_) => {}
                    Event::CarrierStats(_) => {}
                    Event::CarrierJumpRequest(_) => {}
                    Event::CarrierDecommission(_) => {}
                    Event::CarrierCancelDecommission(_) => {}
                    Event::CarrierBankTransfer(_) => {}
                    Event::CarrierDepositFuel(_) => {}
                    Event::CarrierCrewServices(_) => {}
                    Event::CarrierFinance(_) => {}
                    Event::CarrierShipPack(_) => {}
                    Event::CarrierModulePack(_) => {}
                    Event::CarrierTradeOrder(_) => {}
                    Event::CarrierDockingPermission(_) => {}
                    Event::CarrierNameChange(_) => {}
                    Event::CarrierJumpCancelled(_) => {}
                    Event::FCMaterials(_) => {}

                    // COLONISATION
                    Event::ColonisationBeaconDeployed(_) => {}
                    Event::ColonisationConstructionDepot(_) => {}
                    Event::ColonisationContribution(_) => {}
                    Event::ColonisationSystemClaim(_) => {}
                    Event::ColonisationSystemClaimRelease(_) => {}

                    // COMBAT
                    Event::CapShipBond(_) => {}
                    Event::UnderAttack(_) => {}
                    Event::PVPKill(_) => {}

                    Event::FactionKillBond(e) => {
                        self.combat_bonds
                            .entry(e.awarding_faction.clone())
                            .and_modify(|v| *v = v.saturating_add(e.reward as i64))
                            .or_insert(e.reward as i64);
                    }

                    Event::Bounty(e) => {
                        for bounty in e.rewards.unwrap_or_default() {
                            self.bounties
                                .entry(bounty.faction.clone())
                                .and_modify(|v| *v = v.saturating_add(bounty.reward as i64))
                                .or_insert(bounty.reward as i64);
                        }
                    }

                    // COMMUNITY GOAL
                    Event::CommunityGoalJoin(_) => {}
                    Event::CommunityGoalDiscard(_) => {}
                    Event::CommunityGoalReward(_) => {}
                    Event::CommunityGoal(_) => {}
                    Event::ScientificResearch(_) => {}

                    // CREW
                    Event::QuitACrew(_) => {}
                    Event::JoinACrew(_) => {}
                    Event::CrewFire(_) => {}
                    Event::CrewHire(_) => {}
                    Event::KickCrewMember(_) => {}

                    Event::CrewAssign(e) => self.logs.push(e.into()),

                    Event::CrewMemberRoleChange(e) => self.logs.push(e.into()),

                    Event::CrewLaunchFighter(e) => self.logs.push(e.into()),

                    Event::ChangeCrewRole(e) => self.logs.push(e.into()),

                    Event::EndCrewSession(e) => self.logs.push(e.into()),

                    Event::NpcCrewRank(e) => self.logs.push(e.into()),

                    Event::CrewMemberJoins(e) => self.logs.push(e.into("joined")),

                    Event::CrewMemberQuits(e) => self.logs.push(e.into("quit")),

                    Event::NpcCrewPaidWage(e) => {
                        if e.amount != 0 {
                            self.logs.push(e.into())
                        }
                    }

                    // CRIME
                    Event::ClearImpound(_) => {}
                    Event::CommitCrime(_) => {}
                    Event::CrimeVictim(_) => {}
                    Event::PayBounties(_) => {}
                    Event::PayFines(_) => {}
                    Event::HoloscreenHacked(_) => {}

                    // DATA MARKET
                    Event::SellExplorationData(_) => {}
                    Event::BuyExplorationData(_) => {}
                    Event::BuyTradeData(_) => {}
                    Event::SellOrganicData(_) => {}
                    Event::MultiSellExplorationData(_) => {}

                    Event::RedeemVoucher(e) => {
                        let target = match e.r#type.as_str() {
                            "CombatBond" => &mut self.combat_bonds,
                            "bounty" => &mut self.bounties,
                            "codex" => &mut self.discoveries,
                            _ => {
                                panic!("Unknown voucher type: {}", e.r#type);
                            }
                        };

                        if let Some(faction) = e.faction {
                            let result = target
                                .entry(faction.clone())
                                .and_modify(|b| *b = b.saturating_sub(e.amount as i64))
                                .or_default();

                            if *result <= 0 {
                                target.remove(&faction);
                            }
                        } else if let Some(vouchers) = e.factions {
                            for voucher in vouchers {
                                let result = target
                                    .entry(voucher.faction.clone())
                                    .and_modify(|b| *b = b.saturating_sub(e.amount as i64))
                                    .or_default();

                                if *result <= 0 {
                                    target.remove(&voucher.faction);
                                }
                            }
                        }
                    }

                    // ENGINEERING
                    Event::EngineerLegacyConvert(_) => {}
                    Event::EngineerContribution(_) => {}
                    Event::EngineerCraft(_) => {}

                    Event::EngineerProgress(e) => self.engineers = e.into(),

                    // ENVIRONMENT
                    Event::JetConeDamage(_) => {}
                    Event::CockpitBreached(_) => {}
                    Event::HeatWarning(_) => {}
                    Event::HeatDamage(_) => {}
                    Event::ShipTargeted(_) => {}
                    Event::HullDamage(_) => {}
                    Event::SelfDestruct(_) => {}
                    Event::SystemsShutdown(_) => {}
                    Event::ShieldState(_) => {}
                    Event::LaunchDrone(_) => {}
                    Event::DatalinkVoucher(_) => {}

                    // FIGHTER
                    Event::VehicleSwitch(e) => self.logs.push(e.into()),

                    Event::LaunchFighter(e) => self.logs.push(e.into()),

                    Event::FighterRebuilt(e) => self.logs.push(e.into()),

                    Event::DockFighter(e) => self.logs.push(e.into()),

                    Event::FighterDestroyed(e) => {
                        self.logs.push(e.into("Destroyed", "Fighter"))
                    }

                    // FSD
                    Event::Interdiction(_) => {}
                    Event::Interdicted(_) => {}
                    Event::EscapeInterdiction(_) => {}
                    Event::SupercruiseEntry(_) => {}
                    Event::SupercruiseExit(_) => {}
                    Event::SupercruiseDestinationDrop(_) => {}

                    Event::FSDTarget(_) => {}

                    Event::StartJump(e) => self.logs.push(e.into()),

                    Event::FSDJump(e) => {

                        // trim the new system from the start of our nav route if it matches.
                        if !self.nav_route.is_empty() {
                            if let Some(first) = self.nav_route.first() {
                                if first.star_system == e.star_system {
                                    self.nav_route.remove(0);
                                }
                            }
                        }

                        self.current_system = e.star_system.to_string();
                        self.current_body = "".to_string();
                        self.location = e.into();

                        if self.journal_loaded {
                            return query::system(
                                self.current_system.clone(),
                                self.ship_loadout.max_jump_range);
                        }
                    }

                    // FUEL
                    Event::FuelScoop(_) => {}
                    Event::ReservoirReplenished(_) => {}

                    // MARKET
                    Event::MarketBuy(_) => {}
                    Event::MarketSell(_) => {}
                    Event::TechnologyBroker(_) => {}

                    Event::Market(e) => {
                        if !e.items.is_none() {
                            self.market = e.into();
                        }
                    }

                    // MATERIALS
                    Event::MaterialDiscarded(_) => {}
                    Event::MaterialCollected(_) => {}
                    Event::MaterialDiscovered(_) => {}
                    Event::MaterialTrade(_) => {}
                    Event::Synthesis(_) => {}

                    Event::Materials(e) => {
                        if !e.is_empty() {
                            self.materials = e.into();
                        }
                    }

                    // MICRO RESOURCES
                    Event::RequestPowerMicroResources(_) => {}
                    Event::TransferMicroResources(_) => {}
                    Event::DeliverPowerMicroResources(_) => {}
                    Event::SellMicroResources(_) => {}
                    Event::TradeMicroResources(_) => {}
                    Event::BuyMicroResources(_) => {}

                    // MINING
                    Event::ProspectedAsteroid(_) => {}
                    Event::AsteroidCracked(_) => {}
                    Event::MiningRefined(_) => {}

                    // MISSIONS
                    Event::Missions(_) => { /* this doesn't give us all the info we need */ }
                    Event::MissionRedirected(_) => {}

                    Event::MissionAccepted(e) => {
                        self.missions.push(e.into());
                    }

                    Event::MissionFailed(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    Event::MissionAbandoned(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    Event::MissionCompleted(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    // NAVIGATION
                    Event::ApproachBody(_) => {}
                    Event::LeaveBody(_) => {}
                    Event::ApproachSettlement(_) => {}
                    Event::DockingRequested(_) => {}
                    Event::DockingGranted(_) => {}
                    Event::DockingTimeout(_) => {}
                    Event::DockingDenied(_) => {}
                    Event::DockingCancelled(_) => {}
                    Event::USSDrop(_) => {}
                    Event::Touchdown(_) => {}
                    Event::Liftoff(_) => {}
                    Event::Undocked(_) => {}
                    Event::JetConeBoost(_) => {}

                    Event::NavRoute(e) => {
                        let route = e.into();

                        // The journal file gives us blank NavRoute events when we plot one. Kinda weird.
                        if !route.is_empty() {
                            self.nav_route = route;
                        }
                    }

                    Event::NavRouteClear(_) => {
                        self.nav_route.clear();
                    }

                    Event::Disembark(e) => {
                        self.current_body = e.body.clone();
                        self.logs.push(e.into());
                    }

                    Event::Embark(e) => {
                        self.current_body = e.body.clone();
                        self.logs.push(e.into());
                    }

                    Event::Docked(e) => {
                        if let Some(active_fine) = e.active_fine {
                            self.crime.active_fine = active_fine;
                        }
                        if let Some(wanted) = e.wanted {
                            self.crime.wanted = wanted;
                        }
                    }

                    Event::Location(e) => {
                        self.current_system = e.star_system.clone();

                        if e.body_type != "Star" {
                            self.current_body = e.body.clone();
                        }

                        self.location = e.into();
                    }

                    // OUTFITTING
                    Event::Outfitting(_) => {}
                    Event::ModuleInfo(_) => {}
                    Event::ModuleBuyAndStore(_) => {}
                    Event::ModuleSell(_) => {}
                    Event::ModuleStore(_) => {}
                    Event::ModuleRetrieve(_) => {}
                    Event::MassModuleStore(_) => {}
                    Event::ModuleSwap(_) => {}
                    Event::ModuleBuy(_) => {}
                    Event::ModuleSellRemote(_) => {}
                    Event::FetchRemoteModule(_) => {}
                    Event::StoredModules(_) => {}

                    Event::Loadout(e) => self.ship_loadout = e.into(),

                    // PASSENGERS
                    Event::Passengers(_) => {}
                    Event::SearchAndRescue(_) => {}

                    // PERSONAL
                    Event::Statistics(_) => {}
                    Event::Promotion(promotion) => {

                        // CQC isn't handled here because we can't rank up in that outside of CQC mode.

                        if let Some(combat) = promotion.combat {
                            self.rank.combat = combat;
                            self.progress.combat = 0;
                        }
                        if let Some(trade) = promotion.trade {
                            self.rank.trade = trade;
                            self.progress.trade = 0;
                        }
                        if let Some(explore) = promotion.explore {
                            self.rank.explore = explore;
                            self.progress.explore = 0;
                        }
                        if let Some(soldier) = promotion.soldier {
                            self.rank.soldier = soldier;
                            self.progress.soldier = 0;
                        }
                        if let Some(exobiologist) = promotion.exobiologist {
                            self.rank.exobiologist = exobiologist;
                            self.progress.exobiologist = 0;
                        }
                    }

                    Event::Commander(commander) => {
                        self.commander_name = "CMDR ".to_owned() + &commander.name;
                    }

                    Event::Status(e) => {
                        if let Some(balance) = e.balance {
                            self.credits = balance.separate_with_commas() + " CR";
                        }
                        if let Some(legal_state) = e.legal_state {
                            self.crime.legal_state = legal_state;
                        }

                        if e.body_name.is_some() {
                            self.current_body = e.body_name.unwrap()
                        }
                    }

                    Event::Rank(e) => self.rank = e.into(),

                    Event::Progress(e) => self.progress = e.into(),

                    Event::Reputation(e) => self.reputation = e.into(),

                    // POWERPLAY
                    Event::Powerplay(_) => {}
                    Event::PowerplayJoin(_) => {}
                    Event::PowerplayMerits(_) => {}
                    Event::PowerplayRank(_) => {}
                    Event::PowerplayFastTrack(_) => {}
                    Event::PowerplayCollect(_) => {}
                    Event::PowerplayVoucher(_) => {}
                    Event::PowerplayVote(_) => {}
                    Event::PowerplayDefect(_) => {}
                    Event::PowerplayDeliver(_) => {}
                    Event::PowerplaySalary(_) => {}
                    Event::PowerplayLeave(_) => {}

                    // SCAN
                    Event::Scan(_) => {}
                    Event::ScanBaryCentre(_) => {}
                    Event::ScanOrganic(_) => {}
                    Event::Scanned(_) => {}
                    Event::CodexEntry(_) => {}
                    Event::DatalinkScan(_) => {}
                    Event::NavBeaconScan(_) => {}
                    Event::DiscoveryScan(_) => {}
                    Event::DataScanned(_) => {}
                    Event::FSSBodySignals(_) => {}
                    Event::FSSDiscoveryScan(_) => {}
                    Event::FSSAllBodiesFound(_) => {}
                    Event::FSSSignalDiscovered(_) => {}
                    Event::SAASignalsFound(_) => {}
                    Event::SAAScanComplete(_) => {}

                    // SESSION
                    Event::Continued(_) => {}
                    Event::NewCommander(_) => {}
                    Event::Friends(_) => {}
                    Event::ClearSavedGame(_) => {}
                    Event::Screenshot(_) => {}
                    Event::Fileheader(_) => {}
                    Event::SendText(_) => {}
                    Event::Died(_) => {}
                    Event::Resurrect(_) => {}
                    Event::Music(_) => {}

                    Event::LoadGame(_) => {
                        self.nav_route.clear();
                    }

                    Event::ReceiveText(e) => {

                        if self.first_message_timestamp == 0 {
                            self.first_message_timestamp = e.timestamp.timestamp();
                        }
                        else {
                            self.latest_message_timestamp = e.timestamp.timestamp();
                            self.latest_message_timestamp_formatted = format::prettify_date(&e.timestamp)
                        }

                        if e.channel != "npc" && e.channel != "starsystem" {
                            self.messages.push(e.into());
                        }
                    }

                    Event::Shutdown(_) => {
                        self.nav_route.clear();
                    }

                    // SHIP LOCKER
                    Event::ShipLockerMaterials(_) => {}

                    Event::ShipLocker(e) => {
                        if !e.is_empty() {
                            self.ship_locker = e.into();
                        }
                    }

                    // SHIP MAINTENANCE
                    Event::RefuelAll(_) => {}
                    Event::RefuelPartial(_) => {}
                    Event::RepairAll(_) => {}
                    Event::Repair(_) => {}
                    Event::Resupply(_) => {}
                    Event::BuyDrones(_) => {}
                    Event::RepairDrone(_) => {}
                    Event::SellDrones(_) => {}
                    Event::RebootRepair(_) => {}
                    Event::AfmuRepairs(_) => {}

                    Event::RestockVehicle(e) => self.logs.push(e.into()),

                    // SHIPYARD
                    Event::Shipyard(_) => {}
                    Event::ShipyardNew(_) => {}
                    Event::ShipyardRedeem(_) => {}
                    Event::ShipyardBuy(_) => {}
                    Event::ShipRedeemed(_) => {}
                    Event::ShipyardSwap(_) => {}
                    Event::ShipyardSell(_) => {}
                    Event::ShipyardTransfer(_) => {}
                    Event::SellShipOnRebuy(_) => {}
                    Event::StoredShips(_) => {}
                    Event::SetUserShipName(_) => {}

                    // SQUADRON
                    Event::SquadronStartup(_) => {}
                    Event::SquadronCreated(_) => {}
                    Event::SquadronDemotion(_) => {}
                    Event::SquadronPromotion(_) => {}
                    Event::DisbandedSquadron(_) => {}
                    Event::InvitedToSquadron(_) => {}
                    Event::AppliedToSquadron(_) => {}
                    Event::JoinedSquadron(_) => {}
                    Event::KickedFromSquadron(_) => {}
                    Event::LeftSquadron(_) => {}
                    Event::SharedBookmarkToSquadron(_) => {}

                    // SRV
                    Event::DockSRV(_) => {}
                    Event::LaunchSRV(_) => {}
                    Event::SRVDestroyed(_) => {}

                    // SUIT LOADOUT
                    Event::BuySuit(_) => {}
                    Event::SellSuit(_) => {}
                    Event::UpgradeSuit(_) => {}
                    Event::CreateSuitLoadout(_) => {}
                    Event::RenameSuitLoadout(_) => {}
                    Event::DeleteSuitLoadout(_) => {}
                    Event::SwitchSuitLoadout(_) => {}

                    Event::SuitLoadout(e) => self.suit_loadout = e.into(),

                    // TAXI
                    Event::BookTaxi(_) => {}
                    Event::CancelTaxi(_) => {}
                    Event::BookDropship(_) => {}
                    Event::CancelDropship(_) => {}
                    Event::DropshipDeploy(_) => {}

                    // WEAPON
                    Event::BuyWeapon(_) => {}
                    Event::SellWeapon(_) => {}
                    Event::UpgradeWeapon(_) => {}
                    Event::LoadoutRemoveModule(_) => {}
                    Event::LoadoutEquipModule(_) => {}

                    Event::BuyAmmo(e) => self.logs.push(e.into("ammo")),

                    // WING
                    Event::WingAdd(_) => {}
                    Event::WingInvite(_) => {}
                    Event::WingJoin(_) => {}
                    Event::WingLeave(_) => {}
                }
            }
        }

        Task::none()
    }
}
