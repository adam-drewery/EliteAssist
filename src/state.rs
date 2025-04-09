mod activity;
mod market;
mod material;
mod message;
mod ship;
mod suit;
mod personal;
mod engineering;

pub use activity::*;
pub use engineering::*;
pub use market::*;
pub use material::*;
pub use message::*;
pub use personal::*;
pub use ship::*;
pub use suit::*;

use crate::event::JournalEvent;
use serde::Deserialize;
use thousands::Separable;

#[derive(Default)]
pub struct State {
    pub commander_name: String,
    pub credits: String,
    pub current_system: String,
    pub current_body: String,
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
}

#[derive(Debug, Default, Deserialize, Clone)]
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

    pub fn update_from(&mut self, event: JournalEvent) {

        match event {

            JournalEvent::Commander(commander) => {
                self.commander_name = "CMDR ".to_owned() + &commander.name.to_uppercase();
            }

            JournalEvent::Materials(e) => {
                if e.is_empty() { return; }
                self.materials = e.into();
            }

            JournalEvent::Location(e) => {
                self.current_system = e.star_system.clone();
                self.current_body = e.body.clone();
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
                self.journal.push(e.into("Disembarked"));
            }

            JournalEvent::Embark(e) => {
                self.current_body = e.body.clone();
                self.journal.push(e.into("Embarked"));
            }

            JournalEvent::NavigateTo(e) => { self.active_screen = e; }

            JournalEvent::Docked(e) => {
                if let Some(active_fine) = e.active_fine { self.crime.active_fine = active_fine; }
                if let Some(wanted) = e.wanted { self.crime.wanted = wanted; }
            }

            JournalEvent::ReceiveText(e) => {
                if e.channel == "npc" || e.channel == "starsystem" { return; }
                self.messages.push(e.into());
            }

            JournalEvent::Market(e) => {
                if e.items.is_none() { return; }
                self.market = e.into();
            }

            JournalEvent::Rank(e) => { self.rank = e.into() }

            JournalEvent::Progress(e) => { self.rank = e.into() }

            JournalEvent::Reputation(e) => { self.reputation = e.into() }

            JournalEvent::EngineerProgress(e) => { self.engineers = e.into() }
            JournalEvent::SquadronStartup(_) => {}
            JournalEvent::Statistics(_) => {}
            JournalEvent::Powerplay(_) => {}
            JournalEvent::Music(_) => {}

            JournalEvent::SuitLoadout(e) => { self.suit_loadout = e.into() }

            JournalEvent::Backpack(_) => {}
            JournalEvent::Missions(_) => {}
            JournalEvent::Shutdown(_) => {}

            JournalEvent::Loadout(e) => { self.ship_loadout = e.into() }

            JournalEvent::BuyAmmo(e) => { self.journal.push(e.into()) }

            JournalEvent::RestockVehicle(e) => { self.journal.push(e.into()) }

            JournalEvent::BuyMicroResources(_) => {}

            JournalEvent::NpcCrewPaidWage(e) => {

                if e.amount == 0 { return; }
                self.journal.push(e.into())
            }

            JournalEvent::Cargo(_) => {}
            JournalEvent::BookDropship(_) => {}
            JournalEvent::StartJump(_) => {}
            JournalEvent::LaunchDrone(_) => {}
            JournalEvent::SupercruiseEntry(_) => {}
            JournalEvent::SupercruiseExit(_) => {}
            JournalEvent::Resurrect(_) => {}
            JournalEvent::FSSSignalDiscovered(_) => {}
            JournalEvent::NavRoute(_) => {}
            JournalEvent::Shipyard(_) => {}
            JournalEvent::ApproachSettlement(_) => {}
            JournalEvent::StoredShips(_) => {}
            JournalEvent::SwitchSuitLoadout(_) => {}
            JournalEvent::MissionAccepted(_) => {}
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
            JournalEvent::FSDJump(_) => {}
            JournalEvent::NavRouteClear(_) => {}
            JournalEvent::Bounty(_) => {}
            JournalEvent::ReservoirReplenished(_) => {}
            JournalEvent::UseConsumable(_) => {}
            JournalEvent::Outfitting(_) => {}
            JournalEvent::DockingDenied(_) => {}
            JournalEvent::MissionFailed(_) => {}
            JournalEvent::SupercruiseDestinationDrop(_) => {}
            JournalEvent::MissionAbandoned(_) => {}
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
            JournalEvent::MissionCompleted(_) => {}
            JournalEvent::MaterialTrade(_) => {}
            JournalEvent::FSSAllBodiesFound(_) => {}
            JournalEvent::FactionKillBond(_) => {}
            JournalEvent::RedeemVoucher(_) => {}
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
            JournalEvent::SrvDestroyed(_) => {}
            JournalEvent::CodexEntry(_) => {}
            JournalEvent::ModuleSell(_) => {}
            JournalEvent::ModuleStore(_) => {}
            JournalEvent::ModuleRetrieve(_) => {}
            JournalEvent::ShipTargeted(_) => {}
            JournalEvent::EjectCargo(_) => {}
            JournalEvent::HullDamage(_) => {}

            JournalEvent::CrewAssign(e) => { self.journal.push(e.into()) }

            JournalEvent::DockFighter(e) => { self.journal.push(e.into()) }

            JournalEvent::CommunityGoal(_) => {}

            JournalEvent::LaunchFighter(e) => { self.journal.push(e.into()) }

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

            JournalEvent::CrewMemberJoins(e) => { self.journal.push(e.into()) }

            JournalEvent::Interdicted(_) => {}
            JournalEvent::SellOrganicData(_) => {}
            JournalEvent::DockSRV(_) => {}

            JournalEvent::FighterDestroyed(e) => { self.journal.push(e.into()) }

            JournalEvent::ModuleSwap(_) => {}
            JournalEvent::MaterialDiscovered(_) => {}
            JournalEvent::DataScanned(_) => {}
            JournalEvent::VehicleSwitch(_) => {}
            JournalEvent::MarketBuy(_) => {}
            JournalEvent::ModuleSellRemote(_) => {}
            JournalEvent::AfmuRepairs(_) => {}
            JournalEvent::CommunityGoalJoin(_) => {}

            JournalEvent::NpcCrewRank(e) => { self.journal.push(e.into()) }

            JournalEvent::LoadoutEquipModule(_) => {}

            JournalEvent::FighterRebuilt(e) => { self.journal.push(e.into()) }

            JournalEvent::PowerplayJoin(_) => {}

            JournalEvent::CrewMemberRoleChange(e) => { self.journal.push(e.into()) }

            JournalEvent::SelfDestruct(_) => {}
            JournalEvent::BookTaxi(_) => {}
            JournalEvent::MarketSell(_) => {}
            JournalEvent::SellWeapon(_) => {}
            JournalEvent::SystemsShutdown(_) => {}
            JournalEvent::ProspectedAsteroid(_) => {}
            JournalEvent::SRVDestroyed(_) => {}
            JournalEvent::DiscoveryScan(_) => {}

            JournalEvent::CrewLaunchFighter(e) => { self.journal.push(e.into()) }

            JournalEvent::BuyWeapon(_) => {}
            JournalEvent::RenameSuitLoadout(_) => {}
            JournalEvent::MiningRefined(_) => {}
            JournalEvent::CancelTaxi(_) => {}
            JournalEvent::EngineerContribution(_) => {}
            JournalEvent::SellMicroResources(_) => {}
            JournalEvent::UpgradeSuit(_) => {}
            JournalEvent::AppliedToSquadron(_) => {}

            JournalEvent::CrewMemberQuits(e) => { self.journal.push(e.into()) }

            JournalEvent::ChangeCrewRole(e) => { self.journal.push(e.into()) }

            JournalEvent::AsteroidCracked(_) => {}
            JournalEvent::DatalinkVoucher(_) => {}
            JournalEvent::DeliverPowerMicroResources(_) => {}
            JournalEvent::Interdiction(_) => {}

            JournalEvent::EndCrewSession(e) => { self.journal.push(e.into()) }

            JournalEvent::BuySuit(_) => {}
            JournalEvent::SellSuit(_) => {}
            JournalEvent::DeleteSuitLoadout(_) => {},

            // ignore these events, they seem pointless
            JournalEvent::FileHeader(_) => {},
            JournalEvent::LoadGame(_) => {}
        }
    }
}
