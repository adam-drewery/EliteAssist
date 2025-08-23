mod activity;
mod market;
mod material;
mod message;
mod mission;
mod navigation;
mod personal;
mod ship;
mod suit;
pub mod pane;

pub use activity::*;
pub use market::*;
pub use material::*;
pub use message::*;
pub use mission::*;
pub use navigation::*;
pub use personal::*;
pub use ship::*;
pub use suit::*;

use crate::gui::Message;
use crate::query;
use iced::Task;
use iced::widget::pane_grid;
use iced::window;
use serde::Deserialize;
use std::collections::HashMap;
use ed_journals::galaxy::BodyType;
use ed_journals::journal::JournalEventKind;
use ed_journals::logs::bounty_event::BountyEvent;
use ed_journals::logs::LogEventContent;
use ed_journals::logs::promotion_event::PromotionEvent;
use ed_journals::logs::redeem_voucher_event::RedeemVoucherEventType;
use ed_journals::status::LegalStatus;
use log::error;
use thousands::Separable;
use crate::journal::format;

pub struct State {
    pub overview_panes: Option<pane_grid::State<pane::Type>>,
    pub show_settings_menu: bool,
    pub fullscreen: bool,
    pub enabled_panes: Option<Vec<pane::Type>>,
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
    pub nav_route: Vec<NavRouteStep>,
    pub missions: Vec<Mission>,
    pub combat_bonds: HashMap<String, i64>,
    pub bounties: HashMap<String, i64>,
    pub discoveries: HashMap<String, i64>,
    pub progress: Progress,

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
        let mut state = Self {
            overview_panes: None,
            show_settings_menu: false,
            fullscreen: false,
            enabled_panes: None,
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
                state.overview_panes = Some(crate::settings::build_panes_from_layout(layout));

                // If visible list not provided, derive from layout leaves
                state.enabled_panes = Some(settings.visible.unwrap_or_else(|| crate::settings::layout_leaf_panes(layout)));
            } else if let Some(visible) = settings.visible {
                state.enabled_panes = Some(visible);
            }
        }

        state
    }
}

impl State {

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
                pane::dragged(self, event);
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

            Message::TogglePane(pane, enabled) => {
                pane.toggle(self, enabled);
            }

            Message::ToggleFullscreen => {
                // Request the latest window Id and handle in a follow-up message
                return window::get_latest().map(Message::ToggleFullscreenWithId);
            }

            Message::ToggleFullscreenWithId(id_opt) => {
                if let Some(id) = id_opt {
                    let mode = if self.fullscreen { window::Mode::Windowed } else { window::Mode::Fullscreen };
                    self.fullscreen = !self.fullscreen;
                    return window::change_mode(id, mode).map(|_: ()| Message::Empty);
                }
            }

            Message::JournalLoaded => {
                self.journal_loaded = true;
                if self.overview_panes.is_none() {
                    pane::load(self)
                }

                return query::system(
                    self.current_system.clone(),
                    self.ship_loadout.max_jump_range);
            }

            Message::JournalEvent(event) => {

                match event.kind {

                    JournalEventKind::LogEvent(_) => {}

                    JournalEventKind::StatusEvent(e) => {

                        if let Some(status) = e.contents {
                            self.credits = status.balance.separate_with_commas() + " CR";

                            self.crime.legal_state = match status.legal_state{
                                LegalStatus::Allied => "Allied".to_string(),
                                LegalStatus::Clean => "Clean".to_string(),
                                LegalStatus::None => "None".to_string(),
                                LegalStatus::Unknown => "Unknown".to_string(),
                                LegalStatus::Lawless => "Lawless".to_string(),
                                LegalStatus::Enemy => "Enemy".to_string(),
                                LegalStatus::WantedEnemy => "Wanted Enemy".to_string(),
                                LegalStatus::Hunter => "Hunter".to_string(),
                                LegalStatus::IllegalCargo => "Illegal Cargo".to_string(),
                                LegalStatus::Speeding => "Speeding".to_string(),
                                LegalStatus::Wanted => "Wanted".to_string(),
                                LegalStatus::Hostile => "Hostile".to_string(),
                                LegalStatus::PassengerWanted => "Passenger Wanted".to_string(),
                                LegalStatus::Warrant => "Warrant".to_string(),
                                LegalStatus::Thargoid => "Thargoid".to_string(),
                            };

                            if let Some(planet) = status.planet_status {
                                self.current_body = planet.body_name;
                            }
                        }
                    }

                    JournalEventKind::OutfittingEvent(_) => {}
                    JournalEventKind::ShipyardEvent(_) => {}
                    JournalEventKind::MarketEvent(e) => {
                        self.market = e.into();
                    }
                    JournalEventKind::NavRoute(e) => {
                        self.nav_route = NavRouteStep::vec_from(e);

                    }
                    JournalEventKind::ModulesInfo(_) => {}
                    JournalEventKind::Backpack(_) => {}
                    JournalEventKind::Cargo(_) => {}
                    JournalEventKind::ShipLocker(e) => {
                        self.ship_locker = e.into();
                    }
                }
            }
            
            Message::LogEvent(event) => {
                match event.content {
                    // BACKPACK
                    LogEventContent::Backpack(_) => {}
                    LogEventContent::BackpackChange(_) => {}
                    LogEventContent::DropItems(_) => {}
                    LogEventContent::CollectItems(_) => {}
                    LogEventContent::UseConsumable(_) => {}

                    // CARGO
                    LogEventContent::Cargo(_) => {}
                    LogEventContent::CargoTransfer(_) => {}
                    LogEventContent::CargoDepot(_) => {}
                    LogEventContent::CollectCargo(_) => {}
                    LogEventContent::EjectCargo(_) => {}

                    // CARRIER
                    LogEventContent::CarrierLocation(_) => {}
                    LogEventContent::CarrierJump(_) => {}
                    LogEventContent::CarrierBuy(_) => {}
                    LogEventContent::CarrierStats(_) => {}
                    LogEventContent::CarrierJumpRequest(_) => {}
                    LogEventContent::CarrierDecommission(_) => {}
                    LogEventContent::CarrierCancelDecommission(_) => {}
                    LogEventContent::CarrierBankTransfer(_) => {}
                    LogEventContent::CarrierDepositFuel(_) => {}
                    LogEventContent::CarrierCrewServices(_) => {}
                    LogEventContent::CarrierFinance(_) => {}
                    LogEventContent::CarrierShipPack(_) => {}
                    LogEventContent::CarrierModulePack(_) => {}
                    LogEventContent::CarrierTradeOrder(_) => {}
                    LogEventContent::CarrierDockingPermission(_) => {}
                    LogEventContent::CarrierNameChange(_) => {}
                    LogEventContent::CarrierJumpCancelled(_) => {}
                    LogEventContent::FCMaterials(_) => {}

                    // COLONISATION
                    ////LogEventContent::ColonisationBeaconDeployed(_) => {}
                    LogEventContent::ColonisationConstructionDepot(_) => {}
                    LogEventContent::ColonisationContribution(_) => {}
                    ////LogEventContent::ColonisationSystemClaim(_) => {}
                    ////LogEventContent::ColonisationSystemClaimRelease(_) => {}

                    // COMBAT
                    LogEventContent::CapShipBond(_) => {}
                    LogEventContent::UnderAttack(_) => {}
                    LogEventContent::PVPKill(_) => {}

                    LogEventContent::FactionKillBond(e) => {
                        self.combat_bonds
                            .entry(e.awarding_faction.clone())
                            .and_modify(|v| *v = v.saturating_add(e.reward as i64))
                            .or_insert(e.reward as i64);
                    }

                    LogEventContent::Bounty(e) => {
                        
                        match e {
                            BountyEvent::Normal(e) => {

                                for bounty in e.rewards {
                                    self.bounties
                                        .entry(bounty.faction.clone())
                                        .and_modify(|v| *v = v.saturating_add(bounty.reward as i64))
                                        .or_insert(bounty.reward as i64);
                                }
                            }
                            BountyEvent::Skimmer(_) => {}
                        }
                    }

                    // COMMUNITY GOAL
                    LogEventContent::CommunityGoalJoin(_) => {}
                    LogEventContent::CommunityGoalDiscard(_) => {}
                    LogEventContent::CommunityGoalReward(_) => {}
                    LogEventContent::CommunityGoal(_) => {}
                    LogEventContent::ScientificResearch(_) => {}

                    // CREW
                    LogEventContent::QuitACrew(_) => {}
                    LogEventContent::JoinACrew(_) => {}
                    LogEventContent::CrewFire(_) => {}
                    LogEventContent::CrewHire(_) => {}
                    LogEventContent::KickCrewMember(_) => {}

                    LogEventContent::CrewAssign(e) => self.logs.push(GameEventLog::from_crew_assign(e, event.timestamp)),

                    LogEventContent::CrewMemberRoleChange(e) => self.logs.push(GameEventLog::from_crew_member_role_change(e, event.timestamp)),

                    LogEventContent::CrewLaunchFighter(e) => self.logs.push(GameEventLog::from_crew_launch_fighter(e, event.timestamp)),

                    LogEventContent::ChangeCrewRole(e) => self.logs.push(GameEventLog::from_change_crew_role(e, event.timestamp)),

                    LogEventContent::EndCrewSession(e) => self.logs.push(GameEventLog::from_end_crew_session(e, event.timestamp)),

                    LogEventContent::NPCCrewRank(_) => {}

                    LogEventContent::CrewMemberJoins(e) => {}

                    LogEventContent::CrewMemberQuits(e) => {}

                    LogEventContent::NPCCrewPaidWage(e) => {
                        if e.amount != 0 {
                            self.logs.push(GameEventLog::from_npc_crew_wage_paid(e, event.timestamp));
                        }
                    }

                    // CRIME
                    LogEventContent::ClearImpound(_) => {}
                    LogEventContent::CommitCrime(_) => {}
                    LogEventContent::CrimeVictim(_) => {}
                    LogEventContent::PayBounties(_) => {}
                    LogEventContent::PayFines(_) => {}
                    ////LogEventContent::HoloscreenHacked(_) => {}

                    // DATA MARKET
                    LogEventContent::SellExplorationData(_) => {}
                    LogEventContent::BuyExplorationData(_) => {}
                    LogEventContent::BuyTradeData(_) => {}
                    LogEventContent::SellOrganicData(_) => {}
                    LogEventContent::MultiSellExplorationData(_) => {}

                    LogEventContent::RedeemVoucher(e) => {


                        let target = match e.kind {
                            RedeemVoucherEventType::CombatBond => &mut self.combat_bonds,
                            RedeemVoucherEventType::Bounty => &mut self.bounties,
                            RedeemVoucherEventType::Trade => todo!(),
                            RedeemVoucherEventType::Settlement => todo!(),
                            RedeemVoucherEventType::Scannable => todo!(),
                            RedeemVoucherEventType::Codex => &mut self.discoveries,
                        };

                        if let Some(faction) = e.faction {
                            let result = target
                                .entry(faction.clone())
                                .and_modify(|b| *b = b.saturating_sub(e.amount as i64))
                                .or_default();

                            if *result <= 0 {
                                target.remove(&faction);
                            }
                        }

                        for voucher in e.factions {

                            let result = target
                                .entry(voucher.faction.clone())
                                .and_modify(|b| *b = b.saturating_sub(e.amount as i64))
                                .or_default();

                            if *result <= 0 {
                                target.remove(&voucher.faction);
                            }

                        }
                    }

                    // ENGINEERING
                    ////LogEventContent::EngineerLegacyConvert(_) => {}
                    LogEventContent::EngineerContribution(_) => {}
                    LogEventContent::EngineerCraft(_) => {}

                    LogEventContent::EngineerProgress(_) => {}

                    // ENVIRONMENT
                    LogEventContent::JetConeDamage(_) => {}
                    LogEventContent::CockpitBreached => {}
                    LogEventContent::HeatWarning => {}
                    LogEventContent::HeatDamage => {}
                    LogEventContent::ShipTargeted(_) => {}
                    LogEventContent::HullDamage(_) => {}
                    LogEventContent::SelfDestruct => {}
                    LogEventContent::SystemsShutdown => {}
                    LogEventContent::ShieldState(_) => {}
                    LogEventContent::LaunchDrone(_) => {}
                    LogEventContent::DatalinkVoucher(_) => {}
                    // FIGHTER
                    LogEventContent::VehicleSwitch(e) => self.logs.push(GameEventLog::from_vehicle_switch(e, event.timestamp)),

                    LogEventContent::LaunchFighter(e) => self.logs.push(GameEventLog::from_launch_fighter(e, event.timestamp)),

                    LogEventContent::FighterRebuilt(e) => self.logs.push(GameEventLog::from_fighter_rebuilt(e, event.timestamp)),

                    LogEventContent::DockFighter(e) => self.logs.push(GameEventLog::from_dock_fighter(e, event.timestamp)),

                    LogEventContent::FighterDestroyed(_) => {}

                    // FSD
                    LogEventContent::Interdiction(_) => {}
                    LogEventContent::Interdicted(_) => {}
                    LogEventContent::EscapeInterdiction(_) => {}
                    LogEventContent::SupercruiseEntry(_) => {}
                    LogEventContent::SupercruiseExit(_) => {}
                    LogEventContent::SupercruiseDestinationDrop(_) => {}

                    LogEventContent::FSDTarget(_) => {}

                    LogEventContent::StartJump(e) => self.logs.push(GameEventLog::from_start_jump(e, event.timestamp)),

                    LogEventContent::FSDJump(e) => {

                        // trim the new system from the start of our nav route if it matches.
                        if !self.nav_route.is_empty() {
                            if let Some(first) = self.nav_route.first() {
                                if first.star_system == e.system_info.star_system {
                                    self.nav_route.remove(0);
                                }
                            }
                        }

                        self.current_system = e.system_info.star_system.to_string();
                        self.current_body = "".to_string();
                        self.location = e.into();

                        if self.journal_loaded {
                            return query::system(
                                self.current_system.clone(),
                                self.ship_loadout.max_jump_range);
                        }
                    }

                    // FUEL
                    LogEventContent::FuelScoop(_) => {}
                    LogEventContent::ReservoirReplenished(_) => {}

                    // MARKET
                    LogEventContent::MarketBuy(_) => {}
                    LogEventContent::MarketSell(_) => {}
                    LogEventContent::TechnologyBroker(_) => {}
                    LogEventContent::Market(_) => {}

                    // MATERIALS
                    LogEventContent::MaterialDiscarded(_) => {}
                    LogEventContent::MaterialCollected(_) => {}
                    LogEventContent::MaterialDiscovered(_) => {}
                    LogEventContent::MaterialTrade(_) => {}
                    LogEventContent::Synthesis(_) => {}

                    LogEventContent::Materials(e) => {

                        let is_empty = e.encoded.is_empty()
                            && e.manufactured.is_empty()
                            && e.raw.is_empty();

                        if !is_empty {
                            self.materials = e.into();
                        }
                    }

                    // MICRO RESOURCES
                    LogEventContent::RequestPowerMicroResources(_) => {}
                    LogEventContent::TransferMicroResources(_) => {}
                    ////LogEventContent::DeliverPowerMicroResources(_) => {}
                    LogEventContent::SellMicroResources(_) => {}
                    LogEventContent::TradeMicroResources(_) => {}
                    LogEventContent::BuyMicroResources(_) => {}

                    // MINING
                    LogEventContent::ProspectedAsteroid(_) => {}
                    LogEventContent::AsteroidCracked(_) => {}
                    LogEventContent::MiningRefined(_) => {}

                    // MISSIONS
                    LogEventContent::Missions(_) => { /* this doesn't give us all the info we need */ }
                    LogEventContent::MissionRedirected(_) => {}

                    LogEventContent::MissionAccepted(e) => {
                        self.missions.push(Mission::from(e));
                    }

                    LogEventContent::MissionFailed(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    LogEventContent::MissionAbandoned(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    LogEventContent::MissionCompleted(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    // NAVIGATION
                    LogEventContent::ApproachBody(_) => {}
                    LogEventContent::LeaveBody(_) => {}
                    LogEventContent::ApproachSettlement(_) => {}
                    LogEventContent::DockingRequested(_) => {}
                    LogEventContent::DockingGranted(_) => {}
                    LogEventContent::DockingTimeout(_) => {}
                    LogEventContent::DockingDenied(_) => {}
                    LogEventContent::DockingCancelled(_) => {}
                    LogEventContent::USSDrop(_) => {}
                    LogEventContent::Touchdown(_) => {}
                    LogEventContent::Liftoff(_) => {}
                    LogEventContent::Undocked(_) => {}
                    LogEventContent::JetConeBoost(_) => {}
                    LogEventContent::NavRoute => {}

                    LogEventContent::NavRouteClear => {
                        self.nav_route.clear();
                    }

                    LogEventContent::Disembark(e) => {
                        self.current_body = e.body.clone();
                        self.logs.push(GameEventLog::from_disembark(e, event.timestamp));
                    }

                    LogEventContent::Embark(e) => {
                        self.current_body = e.body.clone();
                        self.logs.push(GameEventLog::from_embark(e, event.timestamp));
                    }

                    // LogEventContent::Docked(e) => {
                    //
                    //
                    //     if let Some(active_fine) = e.active_fine {
                    //         self.crime.active_fine = active_fine;
                    //     }
                    //     if let Some(wanted) = e.wanted {
                    //         self.crime.wanted = wanted;
                    //     }
                    // }

                    LogEventContent::Location(e) => {
                        self.current_system = e.location_info.star_system.clone();

                        if e.location_info.body_type != BodyType::Star {
                            self.current_body = e.location_info.body.clone();
                        }

                        self.location = e.into();
                    }

                    // OUTFITTING
                    LogEventContent::Outfitting(_) => {}
                    LogEventContent::ModuleInfo => {}
                    LogEventContent::ModuleBuyAndStore(_) => {}
                    LogEventContent::ModuleSell(_) => {}
                    LogEventContent::ModuleStore(_) => {}
                    LogEventContent::ModuleRetrieve(_) => {}
                    LogEventContent::MassModuleStore(_) => {}
                    LogEventContent::ModuleSwap(_) => {}
                    LogEventContent::ModuleBuy(_) => {}
                    LogEventContent::ModuleSellRemote(_) => {}
                    LogEventContent::FetchRemoteModule(_) => {}
                    LogEventContent::StoredModules(_) => {}

                    LogEventContent::Loadout(e) => self.ship_loadout = e.into(),

                    // PASSENGERS
                    LogEventContent::Passengers(_) => {}
                    LogEventContent::SearchAndRescue(_) => {}

                    // PERSONAL
                    LogEventContent::Statistics(_) => {}
                    LogEventContent::Promotion(promotion) => {

                        // CQC isn't handled here because we can't rank up in that outside of CQC mode.
                        match promotion {
                            PromotionEvent::Combat(e) => {
                                self.rank.combat = e.to_string();
                                self.progress.combat = 0;
                            }
                            PromotionEvent::Trade(e) => {
                                self.rank.trade = e.to_string();
                                self.progress.trade = 0;
                            }
                            PromotionEvent::Exploration(e) => {
                                self.rank.explore = e.to_string();
                                self.progress.explore = 0;
                            }
                            PromotionEvent::Mercenary(e) => {
                                self.rank.mercenary = e.to_string();
                                self.progress.soldier = 0;
                            }
                            PromotionEvent::Exobiologist(e) => {
                                self.rank.exobiology = e.to_string();
                                self.progress.exobiologist = 0;
                            }
                            PromotionEvent::Federation(e) => {
                                self.rank.federation = e.to_string();
                                self.progress.federation = 0;
                            }
                            PromotionEvent::Empire(e) => {
                                self.rank.empire = e.to_string();
                                self.progress.empire = 0;
                            }
                        }
                    }

                    LogEventContent::Commander(commander) => {
                        self.commander_name = "CMDR ".to_owned() + &commander.name;
                    }

                    LogEventContent::Rank(e) => self.rank = e.into(),

                    LogEventContent::Progress(e) => self.progress = e.into(),

                    LogEventContent::Reputation(e) => self.reputation = e.into(),

                    // POWERPLAY
                    LogEventContent::Powerplay(_) => {}
                    LogEventContent::PowerplayJoin(_) => {}
                    LogEventContent::PowerplayMerits(_) => {}
                    LogEventContent::PowerplayRank(_) => {}
                    LogEventContent::PowerplayFastTrack(_) => {}
                    LogEventContent::PowerplayCollect(_) => {}
                    ////LogEventContent::PowerplayVoucher(_) => {}
                    LogEventContent::PowerplayVote(_) => {}
                    LogEventContent::PowerplayDefect(_) => {}
                    LogEventContent::PowerplayDeliver(_) => {}
                    LogEventContent::PowerplaySalary(_) => {}
                    LogEventContent::PowerplayLeave(_) => {}

                    // SCAN
                    LogEventContent::Scan(_) => {}
                    LogEventContent::ScanBaryCentre(_) => {}
                    LogEventContent::ScanOrganic(_) => {}
                    LogEventContent::Scanned(_) => {}
                    LogEventContent::CodexEntry(_) => {}
                    LogEventContent::DatalinkScan(_) => {}
                    LogEventContent::NavBeaconScan(_) => {}
                    LogEventContent::DiscoveryScan(_) => {}
                    LogEventContent::DataScanned(_) => {}
                    LogEventContent::FSSBodySignals(_) => {}
                    LogEventContent::FSSDiscoveryScan(_) => {}
                    LogEventContent::FSSAllBodiesFound(_) => {}
                    LogEventContent::FSSSignalDiscovered(_) => {}
                    LogEventContent::SAASignalsFound(_) => {}
                    LogEventContent::SAAScanComplete(_) => {}

                    // SESSION
                    LogEventContent::Continued(_) => {}
                    LogEventContent::NewCommander(_) => {}
                    LogEventContent::Friends(_) => {}
                    LogEventContent::ClearSavedGame(_) => {}
                    LogEventContent::Screenshot(_) => {}
                    LogEventContent::FileHeader(_) => {}
                    LogEventContent::SendText(_) => {}
                    LogEventContent::Died(_) => {}
                    LogEventContent::Resurrect(_) => {}
                    LogEventContent::Music(_) => {}

                    LogEventContent::LoadGame(_) => {
                        self.nav_route.clear();
                    }

                    LogEventContent::ReceiveText(e) => {

                        if self.first_message_timestamp == 0 {
                            self.first_message_timestamp = event.timestamp.timestamp();
                        }
                        else {
                            self.latest_message_timestamp = event.timestamp.timestamp();
                            self.latest_message_timestamp_formatted = format::prettify_date(&event.timestamp)
                        }

                        if e.channel != "npc" && e.channel != "starsystem" {
                            self.messages.push(ChatMessage::from(e, event.timestamp));
                        }
                    }

                    LogEventContent::Shutdown => {
                        self.nav_route.clear();
                    }

                    // SHIP LOCKER
                    ////LogEventContent::ShipLockerMaterials(_) => {}

                    LogEventContent::ShipLocker(e) => {
                        if let Some(e) = e.contents {
                            self.ship_locker = e.into();
                        }
                    }

                    // SHIP MAINTENANCE
                    LogEventContent::RefuelAll(_) => {}
                    LogEventContent::RefuelPartial(_) => {}
                    LogEventContent::RepairAll(_) => {}
                    LogEventContent::Repair(_) => {}
                    LogEventContent::Resupply => {}
                    LogEventContent::BuyDrones(_) => {}
                    LogEventContent::RepairDrone(_) => {}
                    LogEventContent::SellDrones(_) => {}
                    LogEventContent::RebootRepair(_) => {}
                    LogEventContent::AFMURepairs(_) => {}

                    LogEventContent::RestockVehicle(e) => self.logs.push(GameEventLog::from_restock_vehicle(e, event.timestamp)),

                    // SHIPYARD
                    LogEventContent::Shipyard(_) => {}
                    LogEventContent::ShipyardNew(_) => {}
                    LogEventContent::ShipyardRedeem(_) => {}
                    LogEventContent::ShipyardBuy(_) => {}
                    LogEventContent::ShipRedeemed(_) => {}
                    LogEventContent::ShipyardSwap(_) => {}
                    LogEventContent::ShipyardSell(_) => {}
                    LogEventContent::ShipyardTransfer(_) => {}
                    LogEventContent::SellShipOnRebuy(_) => {}
                    LogEventContent::StoredShips(_) => {}
                    LogEventContent::SetUserShipName(_) => {}
                    ////LogEventContent::ShipyardBankDeposit(_) => {}

                    // SQUADRON
                    LogEventContent::SquadronStartup(_) => {}
                    LogEventContent::SquadronCreated(_) => {}
                    LogEventContent::SquadronDemotion(_) => {}
                    LogEventContent::SquadronPromotion(_) => {}
                    LogEventContent::DisbandedSquadron(_) => {}
                    LogEventContent::InvitedToSquadron(_) => {}
                    LogEventContent::AppliedToSquadron(_) => {}
                    LogEventContent::JoinedSquadron(_) => {}
                    LogEventContent::KickedFromSquadron(_) => {}
                    LogEventContent::LeftSquadron(_) => {}
                    LogEventContent::SharedBookmarkToSquadron(_) => {}

                    // SRV
                    LogEventContent::DockSRV(_) => {}
                    LogEventContent::LaunchSRV(_) => {}
                    LogEventContent::SRVDestroyed(_) => {}

                    // SUIT LOADOUT
                    LogEventContent::BuySuit(_) => {}
                    LogEventContent::SellSuit(_) => {}
                    LogEventContent::UpgradeSuit(_) => {}
                    LogEventContent::CreateSuitLoadout(_) => {}
                    LogEventContent::RenameSuitLoadout(_) => {}
                    LogEventContent::DeleteSuitLoadout(_) => {}
                    LogEventContent::SwitchSuitLoadout(_) => {}

                    LogEventContent::SuitLoadout(e) => self.suit_loadout = e.into(),

                    // TAXI
                    LogEventContent::BookTaxi(_) => {}
                    LogEventContent::CancelTaxi(_) => {}
                    LogEventContent::BookDropship(_) => {}
                    LogEventContent::CancelDropship(_) => {}
                    LogEventContent::DropshipDeploy(_) => {}

                    // WEAPON
                    LogEventContent::BuyWeapon(_) => {}
                    LogEventContent::SellWeapon(_) => {}
                    LogEventContent::UpgradeWeapon(_) => {}
                    LogEventContent::LoadoutRemoveModule(_) => {}
                    LogEventContent::LoadoutEquipModule(_) => {}

                    LogEventContent::BuyAmmo(_) => {}

                    // WING
                    LogEventContent::WingAdd(_) => {}
                    LogEventContent::WingInvite(_) => {}
                    LogEventContent::WingJoin(_) => {}
                    LogEventContent::WingLeave => {}
                    LogEventContent::Docked(_) => {}
                    LogEventContent::WonATrophyForSquadron(_) => {}
                    _ => error!("Unknown log event: {:?}", event)
                }
            }
        }

        Task::none()
    }
}
