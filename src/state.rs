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

use crate::edsm::EdsmClient;
use crate::event::JournalEvent;
use crate::gui::Message;
use serde::Deserialize;
use std::collections::HashMap;
use iced::Task;
use log::warn;
use thousands::Separable;
use crate::event::format::prettify_date;

#[derive(Default)]
pub struct State {
    pub commander_name: String,
    pub credits: String,
    pub current_system: String,
    pub current_body: String,
    pub location: CurrentLocation,
    pub ship_locker: ShipLocker,
    pub ship_loadout: ShipLoadout,
    pub suit_loadout: SuitLoadout,
    pub active_screen: ActiveScreen,
    pub materials: Materials,
    pub messages: Vec<ChatMessage>,
    pub journal: Vec<GameEventLog>,
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
pub enum ActiveScreen {
    #[default]
    Commander,
    ShipLocker,
    Market,
    Materials,
    Messages,
}

impl State {
    pub fn update_from(&mut self, message: Message) -> Task<Message> {

        match message {

            Message::Empty => {}

            Message::NavigateTo(screen) => self.active_screen = screen,

            Message::SystemQueried(system) => {
                if self.location.star_system == system.name.unwrap_or_default() {
                    // todo
                }
            }

            Message::JournalLoaded => {
                self.journal_loaded = true;

                return self.query_system(self.current_system.clone());
            }

            Message::JournalEvent(event) => {

                match event {

                    // BACKPACK
                    JournalEvent::Backpack(_) => {}
                    JournalEvent::BackpackChange(_) => {}
                    JournalEvent::DropItems(_) => {}
                    JournalEvent::CollectItems(_) => {}
                    JournalEvent::UseConsumable(_) => {}

                    // CARGO
                    JournalEvent::Cargo(_) => {}
                    JournalEvent::CargoTransfer(_) => {}
                    JournalEvent::CargoDepot(_) => {}
                    JournalEvent::CollectCargo(_) => {}
                    JournalEvent::EjectCargo(_) => {}

                    // CARRIER
                    JournalEvent::CarrierLocation(_) => {}
                    JournalEvent::CarrierJump(_) => {}
                    JournalEvent::CarrierBuy(_) => {}
                    JournalEvent::CarrierStats(_) => {}
                    JournalEvent::CarrierJumpRequest(_) => {}
                    JournalEvent::CarrierDecommission(_) => {}
                    JournalEvent::CarrierCancelDecommission(_) => {}
                    JournalEvent::CarrierBankTransfer(_) => {}
                    JournalEvent::CarrierDepositFuel(_) => {}
                    JournalEvent::CarrierCrewServices(_) => {}
                    JournalEvent::CarrierFinance(_) => {}
                    JournalEvent::CarrierShipPack(_) => {}
                    JournalEvent::CarrierModulePack(_) => {}
                    JournalEvent::CarrierTradeOrder(_) => {}
                    JournalEvent::CarrierDockingPermission(_) => {}
                    JournalEvent::CarrierNameChange(_) => {}
                    JournalEvent::CarrierJumpCancelled(_) => {}
                    JournalEvent::FCMaterials(_) => {}

                    // COLONISATION
                    JournalEvent::ColonisationBeaconDeployed(_) => {}
                    JournalEvent::ColonisationConstructionDepot(_) => {}
                    JournalEvent::ColonisationContribution(_) => {}
                    JournalEvent::ColonisationSystemClaim(_) => {}
                    JournalEvent::ColonisationSystemClaimRelease(_) => {}

                    // COMBAT
                    JournalEvent::CapShipBond(_) => {}
                    JournalEvent::UnderAttack(_) => {}
                    JournalEvent::PVPKill(_) => {}

                    JournalEvent::FactionKillBond(e) => {
                        self.combat_bonds
                            .entry(e.awarding_faction.clone())
                            .and_modify(|v| *v = v.saturating_add(e.reward as i64))
                            .or_insert(e.reward as i64);
                    }

                    JournalEvent::Bounty(e) => {
                        for bounty in e.rewards.unwrap_or_default() {
                            self.bounties
                                .entry(bounty.faction.clone())
                                .and_modify(|v| *v = v.saturating_add(bounty.reward as i64))
                                .or_insert(bounty.reward as i64);
                        }
                    }

                    // COMMUNITY GOAL
                    JournalEvent::CommunityGoalJoin(_) => {}
                    JournalEvent::CommunityGoalDiscard(_) => {}
                    JournalEvent::CommunityGoalReward(_) => {}
                    JournalEvent::CommunityGoal(_) => {}
                    JournalEvent::ScientificResearch(_) => {}

                    // CREW
                    JournalEvent::QuitACrew(_) => {}
                    JournalEvent::JoinACrew(_) => {}
                    JournalEvent::CrewFire(_) => {}
                    JournalEvent::CrewHire(_) => {}
                    JournalEvent::KickCrewMember(_) => {}

                    JournalEvent::CrewAssign(e) => self.journal.push(e.into()),

                    JournalEvent::CrewMemberRoleChange(e) => self.journal.push(e.into()),

                    JournalEvent::CrewLaunchFighter(e) => self.journal.push(e.into()),

                    JournalEvent::ChangeCrewRole(e) => self.journal.push(e.into()),

                    JournalEvent::EndCrewSession(e) => self.journal.push(e.into()),

                    JournalEvent::NpcCrewRank(e) => self.journal.push(e.into()),

                    JournalEvent::CrewMemberJoins(e) => self.journal.push(e.into("joined")),

                    JournalEvent::CrewMemberQuits(e) => self.journal.push(e.into("quit")),

                    JournalEvent::NpcCrewPaidWage(e) => {
                        if e.amount != 0 {
                            self.journal.push(e.into())
                        }
                    }

                    // CRIME
                    JournalEvent::ClearImpound(_) => {}
                    JournalEvent::CommitCrime(_) => {}
                    JournalEvent::CrimeVictim(_) => {}
                    JournalEvent::PayBounties(_) => {}
                    JournalEvent::PayFines(_) => {}
                    JournalEvent::HoloscreenHacked(_) => {}

                    // DATA MARKET
                    JournalEvent::SellExplorationData(_) => {}
                    JournalEvent::BuyExplorationData(_) => {}
                    JournalEvent::BuyTradeData(_) => {}
                    JournalEvent::SellOrganicData(_) => {}
                    JournalEvent::MultiSellExplorationData(_) => {}

                    JournalEvent::RedeemVoucher(e) => {
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
                    JournalEvent::EngineerLegacyConvert(_) => {}
                    JournalEvent::EngineerContribution(_) => {}
                    JournalEvent::EngineerCraft(_) => {}

                    JournalEvent::EngineerProgress(e) => self.engineers = e.into(),

                    // ENVIRONMENT
                    JournalEvent::JetConeDamage(_) => {}
                    JournalEvent::CockpitBreached(_) => {}
                    JournalEvent::HeatWarning(_) => {}
                    JournalEvent::HeatDamage(_) => {}
                    JournalEvent::ShipTargeted(_) => {}
                    JournalEvent::HullDamage(_) => {}
                    JournalEvent::SelfDestruct(_) => {}
                    JournalEvent::SystemsShutdown(_) => {}
                    JournalEvent::ShieldState(_) => {}
                    JournalEvent::LaunchDrone(_) => {}
                    JournalEvent::DatalinkVoucher(_) => {}

                    // FIGHTER
                    JournalEvent::VehicleSwitch(e) => self.journal.push(e.into()),

                    JournalEvent::LaunchFighter(e) => self.journal.push(e.into()),

                    JournalEvent::FighterRebuilt(e) => self.journal.push(e.into()),

                    JournalEvent::DockFighter(e) => self.journal.push(e.into()),

                    JournalEvent::FighterDestroyed(e) => {
                        self.journal.push(e.into("Destroyed", "Fighter"))
                    }

                    // FSD
                    JournalEvent::Interdiction(_) => {}
                    JournalEvent::Interdicted(_) => {}
                    JournalEvent::EscapeInterdiction(_) => {}
                    JournalEvent::SupercruiseEntry(_) => {}
                    JournalEvent::SupercruiseExit(_) => {}
                    JournalEvent::SupercruiseDestinationDrop(_) => {}

                    JournalEvent::FSDTarget(_) => {}

                    JournalEvent::StartJump(e) => self.journal.push(e.into()),

                    JournalEvent::FSDJump(e) => {
                        self.current_system = e.star_system.to_string();
                        self.current_body = "".to_string();
                        self.location = e.into();

                        return self.query_system(self.current_system.clone());
                    }

                    // FUEL
                    JournalEvent::FuelScoop(_) => {}
                    JournalEvent::ReservoirReplenished(_) => {}

                    // MARKET
                    JournalEvent::MarketBuy(_) => {}
                    JournalEvent::MarketSell(_) => {}
                    JournalEvent::TechnologyBroker(_) => {}

                    JournalEvent::Market(e) => {
                        if !e.items.is_none() {
                            self.market = e.into();
                        }
                    }

                    // MATERIALS
                    JournalEvent::MaterialDiscarded(_) => {}
                    JournalEvent::MaterialCollected(_) => {}
                    JournalEvent::MaterialDiscovered(_) => {}
                    JournalEvent::MaterialTrade(_) => {}
                    JournalEvent::Synthesis(_) => {}

                    JournalEvent::Materials(e) => {
                        if !e.is_empty() {
                            self.materials = e.into();
                        }
                    }

                    // MICRO RESOURCES
                    JournalEvent::RequestPowerMicroResources(_) => {}
                    JournalEvent::TransferMicroResources(_) => {}
                    JournalEvent::DeliverPowerMicroResources(_) => {}
                    JournalEvent::SellMicroResources(_) => {}
                    JournalEvent::TradeMicroResources(_) => {}
                    JournalEvent::BuyMicroResources(_) => {}

                    // MINING
                    JournalEvent::ProspectedAsteroid(_) => {}
                    JournalEvent::AsteroidCracked(_) => {}
                    JournalEvent::MiningRefined(_) => {}

                    // MISSIONS
                    JournalEvent::Missions(_) => { /* this doesn't give us all the info we need */ }
                    JournalEvent::MissionRedirected(_) => {}

                    JournalEvent::MissionAccepted(e) => {
                        self.missions.push(e.into());
                    }

                    JournalEvent::MissionFailed(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    JournalEvent::MissionAbandoned(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    JournalEvent::MissionCompleted(e) => {
                        self.missions.retain(|m| m.mission_id != e.mission_id);
                    }

                    // NAVIGATION
                    JournalEvent::ApproachBody(_) => {}
                    JournalEvent::LeaveBody(_) => {}
                    JournalEvent::ApproachSettlement(_) => {}
                    JournalEvent::DockingRequested(_) => {}
                    JournalEvent::DockingGranted(_) => {}
                    JournalEvent::DockingTimeout(_) => {}
                    JournalEvent::DockingDenied(_) => {}
                    JournalEvent::DockingCancelled(_) => {}
                    JournalEvent::USSDrop(_) => {}
                    JournalEvent::Touchdown(_) => {}
                    JournalEvent::Liftoff(_) => {}
                    JournalEvent::Undocked(_) => {}
                    JournalEvent::JetConeBoost(_) => {}

                    JournalEvent::NavRoute(e) => {
                        let route = e.into();

                        // The journal file gives us blank NavRoute events when we plot one. Kinda weird.
                        if !route.is_empty() {
                            self.nav_route = route;
                        }
                    }

                    JournalEvent::NavRouteClear(_) => {
                        self.nav_route.clear();
                    }

                    JournalEvent::Disembark(e) => {
                        self.current_body = e.body.clone();
                        self.journal.push(e.into());
                    }

                    JournalEvent::Embark(e) => {
                        self.current_body = e.body.clone();
                        self.journal.push(e.into());
                    }

                    JournalEvent::Docked(e) => {
                        if let Some(active_fine) = e.active_fine {
                            self.crime.active_fine = active_fine;
                        }
                        if let Some(wanted) = e.wanted {
                            self.crime.wanted = wanted;
                        }
                    }

                    JournalEvent::Location(e) => {
                        self.current_system = e.star_system.clone();

                        if e.body_type != "Star" {
                            self.current_body = e.body.clone();
                        }

                        self.location = e.into();
                    }

                    // OUTFITTING
                    JournalEvent::Outfitting(_) => {}
                    JournalEvent::ModuleInfo(_) => {}
                    JournalEvent::ModuleBuyAndStore(_) => {}
                    JournalEvent::ModuleSell(_) => {}
                    JournalEvent::ModuleStore(_) => {}
                    JournalEvent::ModuleRetrieve(_) => {}
                    JournalEvent::MassModuleStore(_) => {}
                    JournalEvent::ModuleSwap(_) => {}
                    JournalEvent::ModuleBuy(_) => {}
                    JournalEvent::ModuleSellRemote(_) => {}
                    JournalEvent::FetchRemoteModule(_) => {}
                    JournalEvent::StoredModules(_) => {}

                    JournalEvent::Loadout(e) => self.ship_loadout = e.into(),

                    // PASSENGERS
                    JournalEvent::Passengers(_) => {}
                    JournalEvent::SearchAndRescue(_) => {}

                    // PERSONAL
                    JournalEvent::Statistics(_) => {}
                    JournalEvent::Promotion(_) => {}

                    JournalEvent::Commander(commander) => {
                        self.commander_name = "CMDR ".to_owned() + &commander.name;
                    }

                    JournalEvent::Status(e) => {
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

                    JournalEvent::Rank(e) => self.rank = e.into(),

                    JournalEvent::Progress(e) => self.progress = e.into(),

                    JournalEvent::Reputation(e) => self.reputation = e.into(),

                    // POWERPLAY
                    JournalEvent::Powerplay(_) => {}
                    JournalEvent::PowerplayJoin(_) => {}
                    JournalEvent::PowerplayMerits(_) => {}
                    JournalEvent::PowerplayRank(_) => {}
                    JournalEvent::PowerplayFastTrack(_) => {}
                    JournalEvent::PowerplayCollect(_) => {}
                    JournalEvent::PowerplayVoucher(_) => {}
                    JournalEvent::PowerplayVote(_) => {}
                    JournalEvent::PowerplayDefect(_) => {}
                    JournalEvent::PowerplayDeliver(_) => {}
                    JournalEvent::PowerplaySalary(_) => {}
                    JournalEvent::PowerplayLeave(_) => {}

                    // SCAN
                    JournalEvent::Scan(_) => {}
                    JournalEvent::ScanBaryCentre(_) => {}
                    JournalEvent::ScanOrganic(_) => {}
                    JournalEvent::Scanned(_) => {}
                    JournalEvent::CodexEntry(_) => {}
                    JournalEvent::DatalinkScan(_) => {}
                    JournalEvent::NavBeaconScan(_) => {}
                    JournalEvent::DiscoveryScan(_) => {}
                    JournalEvent::DataScanned(_) => {}
                    JournalEvent::FSSBodySignals(_) => {}
                    JournalEvent::FSSDiscoveryScan(_) => {}
                    JournalEvent::FSSAllBodiesFound(_) => {}
                    JournalEvent::FSSSignalDiscovered(_) => {}
                    JournalEvent::SAASignalsFound(_) => {}
                    JournalEvent::SAAScanComplete(_) => {}

                    // SESSION
                    JournalEvent::Continued(_) => {}
                    JournalEvent::NewCommander(_) => {}
                    JournalEvent::Friends(_) => {}
                    JournalEvent::ClearSavedGame(_) => {}
                    JournalEvent::Screenshot(_) => {}
                    JournalEvent::Fileheader(_) => {}
                    JournalEvent::SendText(_) => {}
                    JournalEvent::Died(_) => {}
                    JournalEvent::Resurrect(_) => {}
                    JournalEvent::Music(_) => {}

                    JournalEvent::LoadGame(_) => {
                        self.nav_route.clear();
                    }

                    JournalEvent::ReceiveText(e) => {

                        if self.first_message_timestamp == 0 {
                            self.first_message_timestamp = e.timestamp.timestamp();
                        }
                        else {
                            self.latest_message_timestamp = e.timestamp.timestamp();
                            self.latest_message_timestamp_formatted = prettify_date(&e.timestamp)
                        }

                        if e.channel != "npc" && e.channel != "starsystem" {
                            self.messages.push(e.into());
                        }
                    }

                    JournalEvent::Shutdown(_) => {
                        self.nav_route.clear();
                    }

                    // SHIP LOCKER
                    JournalEvent::ShipLockerMaterials(_) => {}

                    JournalEvent::ShipLocker(e) => {
                        if !e.is_empty() {
                            self.ship_locker = e.into();
                        }
                    }

                    // SHIP MAINTENANCE
                    JournalEvent::RefuelAll(_) => {}
                    JournalEvent::RefuelPartial(_) => {}
                    JournalEvent::RepairAll(_) => {}
                    JournalEvent::Repair(_) => {}
                    JournalEvent::Resupply(_) => {}
                    JournalEvent::BuyDrones(_) => {}
                    JournalEvent::RepairDrone(_) => {}
                    JournalEvent::SellDrones(_) => {}
                    JournalEvent::RebootRepair(_) => {}
                    JournalEvent::AfmuRepairs(_) => {}

                    JournalEvent::RestockVehicle(e) => self.journal.push(e.into()),

                    // SHIPYARD
                    JournalEvent::Shipyard(_) => {}
                    JournalEvent::ShipyardNew(_) => {}
                    JournalEvent::ShipyardRedeem(_) => {}
                    JournalEvent::ShipyardBuy(_) => {}
                    JournalEvent::ShipRedeemed(_) => {}
                    JournalEvent::ShipyardSwap(_) => {}
                    JournalEvent::ShipyardSell(_) => {}
                    JournalEvent::ShipyardTransfer(_) => {}
                    JournalEvent::SellShipOnRebuy(_) => {}
                    JournalEvent::StoredShips(_) => {}
                    JournalEvent::SetUserShipName(_) => {}

                    // SQUADRON
                    JournalEvent::SquadronStartup(_) => {}
                    JournalEvent::SquadronCreated(_) => {}
                    JournalEvent::SquadronDemotion(_) => {}
                    JournalEvent::SquadronPromotion(_) => {}
                    JournalEvent::DisbandedSquadron(_) => {}
                    JournalEvent::InvitedToSquadron(_) => {}
                    JournalEvent::AppliedToSquadron(_) => {}
                    JournalEvent::JoinedSquadron(_) => {}
                    JournalEvent::KickedFromSquadron(_) => {}
                    JournalEvent::LeftSquadron(_) => {}
                    JournalEvent::SharedBookmarkToSquadron(_) => {}

                    // SRV
                    JournalEvent::DockSRV(_) => {}
                    JournalEvent::LaunchSRV(_) => {}
                    JournalEvent::SRVDestroyed(_) => {}

                    // SUIT LOADOUT
                    JournalEvent::BuySuit(_) => {}
                    JournalEvent::SellSuit(_) => {}
                    JournalEvent::UpgradeSuit(_) => {}
                    JournalEvent::CreateSuitLoadout(_) => {}
                    JournalEvent::RenameSuitLoadout(_) => {}
                    JournalEvent::DeleteSuitLoadout(_) => {}
                    JournalEvent::SwitchSuitLoadout(_) => {}

                    JournalEvent::SuitLoadout(e) => self.suit_loadout = e.into(),

                    // TAXI
                    JournalEvent::BookTaxi(_) => {}
                    JournalEvent::CancelTaxi(_) => {}
                    JournalEvent::BookDropship(_) => {}
                    JournalEvent::CancelDropship(_) => {}
                    JournalEvent::DropshipDeploy(_) => {}

                    // WEAPON
                    JournalEvent::BuyWeapon(_) => {}
                    JournalEvent::SellWeapon(_) => {}
                    JournalEvent::UpgradeWeapon(_) => {}
                    JournalEvent::LoadoutRemoveModule(_) => {}
                    JournalEvent::LoadoutEquipModule(_) => {}

                    JournalEvent::BuyAmmo(e) => self.journal.push(e.into("ammo")),

                    // WING
                    JournalEvent::WingAdd(_) => {}
                    JournalEvent::WingInvite(_) => {}
                    JournalEvent::WingJoin(_) => {}
                    JournalEvent::WingLeave(_) => {}
                }
            }
        }

        Task::none()
    }

    fn query_system(&mut self, star_system: String) -> Task<Message> {

        if self.journal_loaded {
            Task::perform(async move {
                let fetched = EdsmClient::default()
                    .get_system(Some(&star_system), None)
                    .await;

                match fetched {
                    Ok(system) => Message::SystemQueried(system),
                    Err(error) => {
                        warn!("Failed to fetch system: {}", error);
                        Message::Empty
                    }
                }
            }, |m| m)
        }
        else {
            Task::none()
        }
    }
}
