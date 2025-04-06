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

            Event::Materials(e) => {
                if e.is_empty() { return; }
                self.materials = e.into();
            }

            Event::Location(e) => {
                self.current_system = e.star_system.clone();
                self.current_body = e.body.clone();
                //self.journal.push(location.into());
            }

            Event::ShipLocker(e) => {
                if e.is_empty() { return; }
                self.ship_locker = e.into();
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

            Event::Disembark(e) => {
                self.current_body = e.body.clone();
                self.journal.push(e.into("Disembarked"));
            }

            Event::Embark(e) => {
                self.current_body = e.body.clone();
                self.journal.push(e.into("Embarked"));
            }

            Event::NavigateTo(e) => {
                self.active_screen = e;
            }

            Event::Docked(e) => {
                if let Some(active_fine) = e.active_fine {
                    self.crime.active_fine = active_fine;
                }
                if let Some(wanted) = e.wanted {
                    self.crime.wanted = wanted;
                }
            }

            Event::ReceiveText(e) => {
                if e.channel == "npc" || e.channel == "starsystem" { return; }
                self.messages.push(e.into());
            }

            Event::Market(e) => {
                if e.items.is_none() { return; }
                self.market = e.into();
            }

            Event::FileHeader(_) => {}
            Event::Rank(_) => {}
            Event::Progress(_) => {}
            Event::Reputation(_) => {}
            Event::EngineerProgress(_) => {}
            Event::SquadronStartup(_) => {}
            Event::LoadGame(_) => {}
            Event::Statistics(_) => {}
            Event::Powerplay(_) => {}
            Event::Music(_) => {}
            Event::SuitLoadout(_) => {}
            Event::Backpack(_) => {}
            Event::Missions(_) => {}
            Event::Shutdown(_) => {}
            Event::Loadout(_) => {}
            Event::BuyAmmo(e) => {
                self.journal.push(e.into())
            }
            Event::RestockVehicle(e) => {
                self.journal.push(e.into())
            }
            Event::BuyMicroResources(_) => {}
            Event::NpcCrewPaidWage(e) => {

                if e.amount == 0 { return; }
                self.journal.push(e.into())
            }
            Event::Cargo(_) => {}
            Event::BookDropship(_) => {}
            Event::StartJump(_) => {}
            Event::LaunchDrone(_) => {}
            Event::SupercruiseEntry(_) => {}
            Event::SupercruiseExit(_) => {}
            Event::Resurrect(_) => {}
            Event::FSSSignalDiscovered(_) => {}
            Event::NavRoute(_) => {}
            Event::Shipyard(_) => {}
            Event::ApproachSettlement(_) => {}
            Event::StoredShips(_) => {}
            Event::SwitchSuitLoadout(_) => {}
            Event::MissionAccepted(_) => {}
            Event::FSDTarget(_) => {}
            Event::ShipyardSwap(_) => {}
            Event::ShipyardTransfer(_) => {}
            Event::RefuelAll(_) => {}
            Event::ClearImpound(_) => {}
            Event::ModuleInfo(_) => {}
            Event::Undocked(_) => {}
            Event::CommitCrime(_) => {}
            Event::UnderAttack(_) => {}
            Event::CollectItems(_) => {}
            Event::LeaveBody(_) => {}
            Event::FSDJump(_) => {}
            Event::NavRouteClear(_) => {}
            Event::Bounty(_) => {}
            Event::ReservoirReplenished(_) => {}
            Event::UseConsumable(_) => {}
            Event::Outfitting(_) => {}
            Event::DockingDenied(_) => {}
            Event::MissionFailed(_) => {}
            Event::SupercruiseDestinationDrop(_) => {}
            Event::MissionAbandoned(_) => {}
            Event::EngineerCraft(_) => {}
            Event::DropshipDeploy(_) => {}
            Event::FuelScoop(_) => {}
            Event::ApproachBody(_) => {}
            Event::Scan(_) => {}
            Event::StoredModules(_) => {}
            Event::SAASignalsFound(_) => {}
            Event::DockingRequested(_) => {}
            Event::DockingGranted(_) => {}
            Event::HeatWarning(_) => {}
            Event::ShieldState(_) => {}
            Event::MissionCompleted(_) => {}
            Event::MaterialTrade(_) => {}
            Event::FSSAllBodiesFound(_) => {}
            Event::FactionKillBond(_) => {}
            Event::RedeemVoucher(_) => {}
            Event::PayBounties(_) => {}
            Event::Touchdown(_) => {}
            Event::ShipyardSell(_) => {}
            Event::ScanOrganic(_) => {}
            Event::RepairAll(_) => {}
            Event::DatalinkScan(_) => {}
            Event::NavBeaconScan(_) => {}
            Event::MultiSellExplorationData(_) => {}
            Event::Liftoff(_) => {}
            Event::EscapeInterdiction(_) => {}
            Event::ModuleBuy(_) => {}
            Event::USSDrop(_) => {}
            Event::ScanBaryCentre(_) => {}
            Event::Repair(_) => {}
            Event::Passengers(_) => {}
            Event::MissionRedirected(_) => {}
            Event::UpgradeWeapon(_) => {}
            Event::Resupply(_) => {}
            Event::Died(_) => {}
            Event::SrvDestroyed(_) => {}
            Event::CodexEntry(_) => {}
            Event::ModuleSell(_) => {}
            Event::ModuleStore(_) => {}
            Event::ModuleRetrieve(_) => {}
            Event::ShipTargeted(_) => {}
            Event::EjectCargo(_) => {}
            Event::HullDamage(_) => {}
            Event::CrewAssign(e) => {
                self.journal.push(e.into())
            }
            Event::DockFighter(e) => {
                self.journal.push(e.into())
            }
            Event::CommunityGoal(_) => {}
            Event::LaunchFighter(e) => {
                self.journal.push(e.into())
            }
            Event::Scanned(_) => {}
            Event::Friends(_) => {}
            Event::BackpackChange(_) => {}
            Event::SetUserShipName(_) => {}
            Event::FSSDiscoveryScan(_) => {}
            Event::SendText(_) => {}
            Event::BuyDrones(_) => {}
            Event::ShipyardBuy(_) => {}
            Event::Promotion(_) => {}
            Event::CollectCargo(_) => {}
            Event::HeatDamage(_) => {}
            Event::SAAScanComplete(_) => {}
            Event::CreateSuitLoadout(_) => {}
            Event::MaterialCollected(_) => {}
            Event::LaunchSRV(_) => {}
            Event::Synthesis(_) => {}
            Event::TradeMicroResources(_) => {}
            Event::CrimeVictim(_) => {}
            Event::JetConeBoost(_) => {}
            Event::DockingCancelled(_) => {}
            Event::FetchRemoteModule(_) => {}
            Event::PayFines(_) => {}
            Event::SearchAndRescue(_) => {}
            Event::ShipyardNew(_) => {}
            Event::CommunityGoalReward(_) => {}
            Event::CrewMemberJoins(e) => {
                self.journal.push(e.into())
            }
            Event::Interdicted(_) => {}
            Event::SellOrganicData(_) => {}
            Event::DockSRV(_) => {}
            Event::FighterDestroyed(e) => {
                self.journal.push(e.into())
            }
            Event::ModuleSwap(_) => {}
            Event::MaterialDiscovered(_) => {}
            Event::DataScanned(_) => {}
            Event::VehicleSwitch(_) => {}
            Event::MarketBuy(_) => {}
            Event::ModuleSellRemote(_) => {}
            Event::AfmuRepairs(_) => {}
            Event::CommunityGoalJoin(_) => {}
            Event::NpcCrewRank(e) => {
                self.journal.push(e.into())
            }
            Event::LoadoutEquipModule(_) => {}
            Event::FighterRebuilt(e) => {
                self.journal.push(e.into())
            }
            Event::PowerplayJoin(_) => {}
            Event::CrewMemberRoleChange(e) => {
                self.journal.push(e.into())
            }
            Event::SelfDestruct(_) => {}
            Event::BookTaxi(_) => {}
            Event::MarketSell(_) => {}
            Event::SellWeapon(_) => {}
            Event::SystemsShutdown(_) => {}
            Event::ProspectedAsteroid(_) => {}
            Event::SRVDestroyed(_) => {}
            Event::DiscoveryScan(_) => {}
            Event::CrewLaunchFighter(e) => {
                self.journal.push(e.into())
            }
            Event::BuyWeapon(_) => {}
            Event::RenameSuitLoadout(_) => {}
            Event::MiningRefined(_) => {}
            Event::CancelTaxi(_) => {}
            Event::EngineerContribution(_) => {}
            Event::SellMicroResources(_) => {}
            Event::UpgradeSuit(_) => {}
            Event::AppliedToSquadron(_) => {}
            Event::CrewMemberQuits(e) => {
                self.journal.push(e.into())
            }
            Event::ChangeCrewRole(e) => {
                self.journal.push(e.into())
            }
            Event::AsteroidCracked(_) => {}
            Event::DatalinkVoucher(_) => {}
            Event::DeliverPowerMicroResources(_) => {}
            Event::Interdiction(_) => {}
            Event::EndCrewSession(e) => {
                self.journal.push(e.into())
            }
            Event::BuySuit(_) => {}
            Event::SellSuit(_) => {}
            Event::DeleteSuitLoadout(_) => {}
            Event::None => {}
        }
    }
}
