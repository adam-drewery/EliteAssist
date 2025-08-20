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
use serde::Deserialize;
use std::collections::HashMap;
use thousands::Separable;

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

            Message::StationsQueried(response) => {
                self.location.stations = response.into();
            }

            Message::NearbySystemsQueried(systems) => {
                self.location.nearby_systems = systems.into_iter().map(|s| s.into()).collect();
            }

            Message::BodiesQueried(bodies) => {
                self.location.edsm_bodies = bodies.into();
            }

            Message::FactionsQueried(factions) => {
                self.location.edsm_factions = Some(factions.into());
            }

            Message::TrafficQueried(traffic) => {
                self.location.edsm_traffic = Some(traffic.into());
            }

            Message::DeathsQueried(deaths) => {
                self.location.edsm_deaths = Some(deaths.into());
            }

            Message::JournalLoaded => {
                self.journal_loaded = true;

                if self.journal_loaded {
                    return query::system(&self.current_system);
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
                            return query::system(&self.current_system);
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
