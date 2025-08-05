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

use crate::event::JournalEvent;
use log::warn;
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
    pub journal: Vec<GameActivity>,
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
}

#[derive(Debug, Default, Deserialize, Clone)]
pub enum ActiveScreen {
    #[default]
    Commander,
    ShipLocker,
    Market,
    Materials,
    Messages,
}

impl State {
    pub fn update_from(&mut self, event: JournalEvent) {
        match event {
            JournalEvent::Commander(commander) => {
                self.commander_name = "CMDR ".to_owned() + &commander.name;
            }

            JournalEvent::Materials(e) => {
                if e.is_empty() { return; }
                self.materials = e.into();
            }

            JournalEvent::Location(e) => {
                self.current_system = e.star_system.clone();

                if e.body_type != "Star" {
                    self.current_body = e.body.clone();
                }

                self.location = e.into();
            }

            JournalEvent::ShipLocker(e) => {
                if e.is_empty() { return; }
                self.ship_locker = e.into();
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

            JournalEvent::Disembark(e) => {
                self.current_body = e.body.clone();
                self.journal.push(e.into());
            }

            JournalEvent::Embark(e) => {
                self.current_body = e.body.clone();
                self.journal.push(e.into());
            }

            JournalEvent::NavigateTo(e) => {
                self.active_screen = e;
            }

            JournalEvent::Docked(e) => {
                if let Some(active_fine) = e.active_fine {
                    self.crime.active_fine = active_fine;
                }
                if let Some(wanted) = e.wanted {
                    self.crime.wanted = wanted;
                }
            }

            JournalEvent::ReceiveText(e) => {
                if e.channel == "npc" || e.channel == "starsystem" { return; }
                self.messages.push(e.into());
            }

            JournalEvent::Market(e) => {
                if e.items.is_none() { return; }
                self.market = e.into();
            }

            JournalEvent::Rank(e) => self.rank = e.into(),

            JournalEvent::Progress(e) => self.rank = e.into(),

            JournalEvent::Reputation(e) => self.reputation = e.into(),

            JournalEvent::EngineerProgress(e) => self.engineers = e.into(),
            JournalEvent::SquadronStartup(_) => {}
            JournalEvent::Statistics(_) => {}
            JournalEvent::Powerplay(_) => {}
            JournalEvent::Music(_) => {}

            JournalEvent::SuitLoadout(e) => self.suit_loadout = e.into(),

            JournalEvent::Backpack(_) => {}
            JournalEvent::Missions(_) => { /* this doesn't give us all the info we need */ }
            JournalEvent::Shutdown(_) => {
                self.nav_route.clear();
            }

            JournalEvent::Loadout(e) => self.ship_loadout = e.into(),

            JournalEvent::BuyAmmo(e) => self.journal.push(e.into("ammo")),

            JournalEvent::RestockVehicle(e) => self.journal.push(e.into()),

            JournalEvent::BuyMicroResources(_) => {}

            JournalEvent::NpcCrewPaidWage(e) => {
                if e.amount == 0 {
                    return;
                }
                self.journal.push(e.into())
            }

            JournalEvent::Cargo(_) => {}
            JournalEvent::BookDropship(_) => {}

            JournalEvent::StartJump(e) => self.journal.push(e.into()),

            JournalEvent::LaunchDrone(_) => {}
            JournalEvent::SupercruiseEntry(_) => {}
            JournalEvent::SupercruiseExit(_) => {}
            JournalEvent::Resurrect(_) => {}
            JournalEvent::FSSSignalDiscovered(_) => {}

            JournalEvent::NavRoute(e) => {
                let route = e.into();

                // The journal file gives us blank NavRoute events when we plot one. Kinda weird.
                if !route.is_empty() {
                    self.nav_route = route;
                }
            }

            JournalEvent::Shipyard(_) => {}
            JournalEvent::ApproachSettlement(_) => {}
            JournalEvent::StoredShips(_) => {}
            JournalEvent::SwitchSuitLoadout(_) => {}

            JournalEvent::MissionAccepted(e) => {
                self.missions.push(e.into());
            }

            JournalEvent::FSDTarget(_) => {}
            JournalEvent::ShipyardSwap(_) => {}
            JournalEvent::ShipyardTransfer(_) => {}
            JournalEvent::RefuelAll(_) => {}
            JournalEvent::ClearImpound(_) => {}
            JournalEvent::ModuleInfo(_) => {}
            JournalEvent::Undocked(_) => {}
            JournalEvent::CommitCrime(_) => {}
            JournalEvent::UnderAttack(_) => {}
            JournalEvent::CollectItems(_) => {}
            JournalEvent::LeaveBody(_) => {}

            JournalEvent::FSDJump(e) => {
                self.current_system = e.star_system.to_string();
                self.current_body = "".to_string();
                self.location = e.into();
            }

            JournalEvent::NavRouteClear(_) => {
                self.nav_route.clear();
            }

            JournalEvent::Bounty(e) => {
                for reward in e.rewards.unwrap_or_default() {
                    self.bounties
                        .entry(reward.faction.clone())
                        .and_modify(|v| *v += i64::from(reward.reward))
                        .or_insert(i64::from(reward.reward));
                }
            }
            JournalEvent::ReservoirReplenished(_) => {}
            JournalEvent::UseConsumable(_) => {}
            JournalEvent::Outfitting(_) => {}
            JournalEvent::DockingDenied(_) => {}

            JournalEvent::MissionFailed(e) => {
                self.missions.retain(|m| m.mission_id != e.mission_id);
            }

            JournalEvent::MissionAbandoned(e) => {
                self.missions.retain(|m| m.mission_id != e.mission_id);
            }

            JournalEvent::MissionCompleted(e) => {
                self.missions.retain(|m| m.mission_id != e.mission_id);
            }

            JournalEvent::SupercruiseDestinationDrop(_) => {}
            JournalEvent::EngineerCraft(_) => {}
            JournalEvent::DropshipDeploy(_) => {}
            JournalEvent::FuelScoop(_) => {}
            JournalEvent::ApproachBody(_) => {}
            JournalEvent::Scan(_) => {}
            JournalEvent::StoredModules(_) => {}
            JournalEvent::SAASignalsFound(_) => {}
            JournalEvent::DockingRequested(_) => {}
            JournalEvent::DockingGranted(_) => {}
            JournalEvent::HeatWarning(_) => {}
            JournalEvent::ShieldState(_) => {}
            JournalEvent::MaterialTrade(_) => {}
            JournalEvent::FSSAllBodiesFound(_) => {}

            JournalEvent::FactionKillBond(e) => {
                self.combat_bonds
                    .entry(e.awarding_faction.clone())
                    .and_modify(|v| *v += i64::from(e.reward))
                    .or_insert(i64::from(e.reward));
            }

            JournalEvent::RedeemVoucher(e) => {
                let target = match e.r#type.as_str() {
                    "CombatBond" => &mut self.combat_bonds,
                    "bounty" => &mut self.bounties,
                    "codex" => &mut self.discoveries,
                    _ => {
                        warn!("Unknown voucher type: {}", e.r#type);
                        return;
                    }
                };

                if let Some(faction) = e.faction {
                    let result = target
                        .entry(faction.clone())
                        .and_modify(|b| *b -= i64::from(e.amount))
                        .or_default();

                    if *result <= 0 {
                        target.remove(&faction);
                    }
                } else if let Some(vouchers) = e.factions {
                    for voucher in vouchers {
                        let result = target
                            .entry(voucher.faction.clone())
                            .and_modify(|b| *b -= i64::from(e.amount))
                            .or_default();

                        if *result <= 0 {
                            target.remove(&voucher.faction);
                        }
                    }
                }
            }

            JournalEvent::PayBounties(_) => {}
            JournalEvent::Touchdown(_) => {}
            JournalEvent::ShipyardSell(_) => {}
            JournalEvent::ScanOrganic(_) => {}
            JournalEvent::RepairAll(_) => {}
            JournalEvent::DatalinkScan(_) => {}
            JournalEvent::NavBeaconScan(_) => {}
            JournalEvent::MultiSellExplorationData(_) => {}
            JournalEvent::Liftoff(_) => {}
            JournalEvent::EscapeInterdiction(_) => {}
            JournalEvent::ModuleBuy(_) => {}
            JournalEvent::USSDrop(_) => {}
            JournalEvent::ScanBaryCentre(_) => {}
            JournalEvent::Repair(_) => {}
            JournalEvent::Passengers(_) => {}
            JournalEvent::MissionRedirected(_) => {}
            JournalEvent::UpgradeWeapon(_) => {}
            JournalEvent::Resupply(_) => {}
            JournalEvent::Died(_) => {}
            JournalEvent::CodexEntry(_) => {}
            JournalEvent::ModuleSell(_) => {}
            JournalEvent::ModuleStore(_) => {}
            JournalEvent::ModuleRetrieve(_) => {}
            JournalEvent::ShipTargeted(_) => {}
            JournalEvent::EjectCargo(_) => {}
            JournalEvent::HullDamage(_) => {}

            JournalEvent::CrewAssign(e) => self.journal.push(e.into()),

            JournalEvent::DockFighter(e) => self.journal.push(e.into()),

            JournalEvent::CommunityGoal(_) => {}

            JournalEvent::LaunchFighter(e) => self.journal.push(e.into()),

            JournalEvent::Scanned(_) => {}
            JournalEvent::Friends(_) => {}
            JournalEvent::BackpackChange(_) => {}
            JournalEvent::SetUserShipName(_) => {}
            JournalEvent::FSSDiscoveryScan(_) => {}
            JournalEvent::SendText(_) => {}
            JournalEvent::BuyDrones(_) => {}
            JournalEvent::ShipyardBuy(_) => {}
            JournalEvent::Promotion(_) => {}
            JournalEvent::CollectCargo(_) => {}
            JournalEvent::HeatDamage(_) => {}
            JournalEvent::SAAScanComplete(_) => {}
            JournalEvent::CreateSuitLoadout(_) => {}
            JournalEvent::MaterialCollected(_) => {}
            JournalEvent::LaunchSRV(_) => {}
            JournalEvent::Synthesis(_) => {}
            JournalEvent::TradeMicroResources(_) => {}
            JournalEvent::CrimeVictim(_) => {}
            JournalEvent::JetConeBoost(_) => {}
            JournalEvent::DockingCancelled(_) => {}
            JournalEvent::FetchRemoteModule(_) => {}
            JournalEvent::PayFines(_) => {}
            JournalEvent::SearchAndRescue(_) => {}
            JournalEvent::ShipyardNew(_) => {}
            JournalEvent::CommunityGoalReward(_) => {}

            JournalEvent::CrewMemberJoins(e) => self.journal.push(e.into("joined")),

            JournalEvent::Interdicted(_) => {}
            JournalEvent::SellOrganicData(_) => {}
            JournalEvent::DockSRV(_) => {}

            JournalEvent::FighterDestroyed(e) => self.journal.push(e.into("Fighter", "destroyed")),

            JournalEvent::ModuleSwap(_) => {}
            JournalEvent::MaterialDiscovered(_) => {}
            JournalEvent::DataScanned(_) => {}
            JournalEvent::VehicleSwitch(_) => {}
            JournalEvent::MarketBuy(_) => {}
            JournalEvent::ModuleSellRemote(_) => {}
            JournalEvent::AfmuRepairs(_) => {}
            JournalEvent::CommunityGoalJoin(_) => {}

            JournalEvent::NpcCrewRank(e) => self.journal.push(e.into()),

            JournalEvent::LoadoutEquipModule(_) => {}

            JournalEvent::FighterRebuilt(e) => self.journal.push(e.into()),

            JournalEvent::PowerplayJoin(_) => {}

            JournalEvent::CrewMemberRoleChange(e) => self.journal.push(e.into()),

            JournalEvent::SelfDestruct(_) => {}
            JournalEvent::BookTaxi(_) => {}
            JournalEvent::MarketSell(_) => {}
            JournalEvent::SellWeapon(_) => {}
            JournalEvent::SystemsShutdown(_) => {}
            JournalEvent::ProspectedAsteroid(_) => {}
            JournalEvent::SRVDestroyed(_) => {}
            JournalEvent::DiscoveryScan(_) => {}

            JournalEvent::CrewLaunchFighter(e) => self.journal.push(e.into()),

            JournalEvent::BuyWeapon(_) => {}
            JournalEvent::RenameSuitLoadout(_) => {}
            JournalEvent::MiningRefined(_) => {}
            JournalEvent::CancelTaxi(_) => {}
            JournalEvent::EngineerContribution(_) => {}
            JournalEvent::SellMicroResources(_) => {}
            JournalEvent::UpgradeSuit(_) => {}
            JournalEvent::AppliedToSquadron(_) => {}

            JournalEvent::CrewMemberQuits(e) => self.journal.push(e.into("quit")),

            JournalEvent::ChangeCrewRole(e) => self.journal.push(e.into()),

            JournalEvent::AsteroidCracked(_) => {}
            JournalEvent::DatalinkVoucher(_) => {}
            JournalEvent::DeliverPowerMicroResources(_) => {}
            JournalEvent::Interdiction(_) => {}

            JournalEvent::EndCrewSession(e) => self.journal.push(e.into()),

            JournalEvent::BuySuit(_) => {}
            JournalEvent::SellSuit(_) => {}
            JournalEvent::DeleteSuitLoadout(_) => {}

            JournalEvent::Fileheader(_) => {}
            JournalEvent::LoadGame(_) => {
                self.nav_route.clear();
            }
            JournalEvent::DisbandedSquadron(_) => {}
            JournalEvent::InvitedToSquadron(_) => {}
            JournalEvent::JoinedSquadron(_) => {}
            JournalEvent::KickedFromSquadron(_) => {}
            JournalEvent::LeftSquadron(_) => {}
            JournalEvent::SharedBookmarkToSquadron(_) => {}
            JournalEvent::SquadronCreated(_) => {}
            JournalEvent::SquadronDemotion(_) => {}
            JournalEvent::SquadronPromotion(_) => {}
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
            JournalEvent::CancelDropship(_) => {}
            JournalEvent::DropItems(_) => {}
            JournalEvent::NewCommander(_) => {}
            JournalEvent::DockingTimeout(_) => {}
            JournalEvent::SellShipOnRebuy(_) => {}
            JournalEvent::CrewFire(_) => {}
            JournalEvent::CrewHire(_) => {}
            JournalEvent::ScientificResearch(_) => {}
            JournalEvent::TechnologyBroker(_) => {}
            JournalEvent::CargoDepot(_) => {}
            JournalEvent::MassModuleStore(_) => {}
            JournalEvent::SellDrones(_) => {}
            JournalEvent::PowerplayDefect(_) => {}
            JournalEvent::PowerplayDeliver(_) => {}
            JournalEvent::PowerplaySalary(_) => {}
            JournalEvent::PowerplayLeave(_) => {}
            JournalEvent::CapShipBond(_) => {}
            JournalEvent::PVPKill(_) => {}
            JournalEvent::Screenshot(_) => {}
            JournalEvent::SellExplorationData(_) => {}
            JournalEvent::MaterialDiscarded(_) => {}
            JournalEvent::FSSBodySignals(_) => {}
            JournalEvent::BuyExplorationData(_) => {}
            JournalEvent::ClearSavedGame(_) => {}
            JournalEvent::BuyTradeData(_) => {}
            JournalEvent::PowerplayFastTrack(_) => {}
            JournalEvent::PowerplayCollect(_) => {}
            JournalEvent::PowerplayVoucher(_) => {}
            JournalEvent::PowerplayVote(_) => {}
            JournalEvent::RepairDrone(_) => {}
            JournalEvent::QuitACrew(_) => {}
            JournalEvent::JoinACrew(_) => {}
            JournalEvent::RebootRepair(_) => {}
            JournalEvent::KickCrewMember(_) => {}
            JournalEvent::CockpitBreached(_) => {}
            JournalEvent::Continued(_) => {}
            JournalEvent::CommunityGoalDiscard(_) => {}
            JournalEvent::JetConeDamage(_) => {}
            JournalEvent::RefuelPartial(_) => {}

            // Wing events
            JournalEvent::WingAdd(_) => {}
            JournalEvent::WingInvite(_) => {}
            JournalEvent::WingJoin(_) => {}
            JournalEvent::WingLeave(_) => {}
        }
    }
}
