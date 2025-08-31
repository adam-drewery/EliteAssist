use crate::state::*;
use crate::journal;
use crate::journal::format;
use crate::query;
use iced::Task;
use log::warn;
use thousands::Separable;
use crate::message::Message;

impl journal::Event {
    pub fn update(self, state: &mut State) -> Task<Message> {
        use journal::Event;

        match self {
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
                state.combat_bonds
                    .entry(e.awarding_faction.clone())
                    .and_modify(|v| *v = v.saturating_add(e.reward as u32))
                    .or_insert(e.reward as u32);
            }

            Event::Bounty(e) => {
                for bounty in e.rewards.unwrap_or_default() {
                    state.bounties
                        .entry(bounty.faction.clone())
                        .and_modify(|v| *v = v.saturating_add(bounty.reward as u32))
                        .or_insert(bounty.reward as u32);
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

            Event::CrewAssign(e) => state.logs.push(e.into()),

            Event::CrewMemberRoleChange(e) => state.logs.push(e.into()),

            Event::CrewLaunchFighter(e) => state.logs.push(e.into()),

            Event::ChangeCrewRole(e) => state.logs.push(e.into()),

            Event::EndCrewSession(e) => state.logs.push(e.into()),

            Event::NpcCrewRank(e) => state.logs.push(e.into()),

            Event::CrewMemberJoins(e) => state.logs.push(log_crew_member(e, "joined")),

            Event::CrewMemberQuits(e) => state.logs.push(log_crew_member(e, "quit")),

            Event::NpcCrewPaidWage(e) => {
                if e.amount != 0 {
                    state.logs.push(e.into())
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
                let target = match e.r#type.as_ref() {
                    "CombatBond" => &mut state.combat_bonds,
                    "bounty" => &mut state.bounties,
                    "codex" => &mut state.discoveries,
                    _ => {
                        warn!("Unknown voucher type: {}", e.r#type);
                        return Task::none();
                    }
                };

                if let Some(faction) = e.faction {
                    let result = target
                        .entry(faction.clone())
                        .and_modify(|b| *b = b.saturating_sub(e.amount as u32))
                        .or_default();

                    if *result <= 0 {
                        target.remove(&faction);
                    }
                } else if let Some(vouchers) = e.factions {
                    for voucher in vouchers {
                        let result = target
                            .entry(voucher.faction.clone())
                            .and_modify(|b| *b = b.saturating_sub(e.amount as u32))
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

            Event::EngineerProgress(e) => state.engineers = e.into(),

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
            Event::VehicleSwitch(e) => state.logs.push(e.into()),

            Event::LaunchFighter(e) => state.logs.push(e.into()),

            Event::FighterRebuilt(e) => state.logs.push(e.into()),

            Event::DockFighter(e) => state.logs.push(e.into()),

            Event::FighterDestroyed(e) => {
                state.logs.push(log_damage(e, "Destroyed", "Fighter"))
            }

            // FSD
            Event::Interdiction(_) => {}
            Event::Interdicted(_) => {}
            Event::EscapeInterdiction(_) => {}
            Event::SupercruiseEntry(_) => {}
            Event::SupercruiseExit(_) => {}
            Event::SupercruiseDestinationDrop(_) => {}

            Event::FSDTarget(_) => {}

            Event::StartJump(e) => state.logs.push(e.into()),

            Event::FSDJump(e) => {
                // trim the new system from the start of our nav route if it matches.
                if !state.nav_route.is_empty() {
                    if let Some(first) = state.nav_route.first() {
                        if first.star_system == e.star_system {
                            state.nav_route.remove(0);
                        }
                    }
                }

                state.current_system = e.star_system.clone();
                state.current_body = String::new().into();
                state.location = e.into();

                if state.journal_loaded {
                    return query::system(
                        state.current_system.as_ref(),
                        state.ship_loadout.max_jump_range);
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
                    state.market = e.into();
                }
            }

            // MATERIALS
            Event::MaterialDiscarded(_) => {}
            Event::MaterialCollected(_) => {}
            Event::MaterialDiscovered(_) => {}
            Event::MaterialTrade(_) => {}
            Event::Synthesis(_) => {}

            Event::Materials(e) => {
                let is_empty = e.encoded.is_empty()
                    && e.manufactured.is_empty()
                    && e.raw.is_empty();

                if !is_empty {
                    state.materials = e.into();
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
                state.missions.push(e.into());
            }

            Event::MissionFailed(e) => {
                state.missions.retain(|m| m.mission_id != e.mission_id);
            }

            Event::MissionAbandoned(e) => {
                state.missions.retain(|m| m.mission_id != e.mission_id);
            }

            Event::MissionCompleted(e) => {
                state.missions.retain(|m| m.mission_id != e.mission_id);
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
                let route: Vec<NavRouteStep> = e.into();

                // The journal file gives us blank NavRoute events when we plot one. Kinda weird.
                if !route.is_empty() {
                    state.nav_route = route;
                }
            }

            Event::NavRouteClear(_) => {
                state.nav_route.clear();
            }

            Event::Disembark(e) => {
                state.current_body = e.body.clone();
                state.logs.push(e.into());
            }

            Event::Embark(e) => {
                state.current_body = e.body.clone();
                state.logs.push(e.into());
            }

            Event::Docked(e) => {
                if let Some(active_fine) = e.active_fine {
                    state.crime.active_fine = active_fine;
                }
                if let Some(wanted) = e.wanted {
                    state.crime.wanted = wanted;
                }
            }

            Event::Location(e) => {
                state.current_system = e.star_system.clone();

                if e.body_type.as_ref() != "Star" {
                    state.current_body = e.body.clone();
                }

                state.location = e.into();
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

            Event::Loadout(e) => state.ship_loadout = e.into(),

            // PASSENGERS
            Event::Passengers(_) => {}
            Event::SearchAndRescue(_) => {}

            // PERSONAL
            Event::Statistics(_) => {}
            Event::Promotion(promotion) => {
                // CQC isn't handled here because we can't rank up in that outside of CQC mode.

                if let Some(combat) = promotion.combat {
                    state.rank.combat = combat as u8;
                    state.progress.combat = 0;
                }
                if let Some(trade) = promotion.trade {
                    state.rank.trade = trade as u8;
                    state.progress.trade = 0;
                }
                if let Some(explore) = promotion.explore {
                    state.rank.explore = explore as u8;
                    state.progress.explore = 0;
                }
                if let Some(soldier) = promotion.soldier {
                    state.rank.soldier = soldier as u8;
                    state.progress.soldier = 0;
                }
                if let Some(exobiologist) = promotion.exobiologist {
                    state.rank.exobiologist = exobiologist as u8;
                    state.progress.exobiologist = 0;
                }
            }

            Event::Commander(commander) => {
                state.commander_name = ("CMDR ".to_string() + commander.name.as_ref()).into();
            }

            Event::Status(e) => {
                if let Some(balance) = e.balance {
                    state.credits = (balance.separate_with_commas() + " CR").into();
                }
                if let Some(legal_state) = e.legal_state {
                    state.crime.legal_state = legal_state;
                }

                if e.body_name.is_some() {
                    state.current_body = e.body_name.unwrap()
                }
            }

            Event::Rank(e) => state.rank = e.into(),

            Event::Progress(e) => state.progress = e.into(),

            Event::Reputation(e) => state.reputation = e.into(),

            // POWERPLAY
            Event::Powerplay(e) => {
                state.powerplay.power = Some(e.power);
                state.powerplay.rank = Some(e.rank as u8);
                state.powerplay.merits = e.merits;
                state.powerplay.time_pledged = e.time_pledged;
            }
            Event::PowerplayJoin(e) => {
                state.powerplay.power = Some(e.power);
            }
            Event::PowerplayMerits(e) => {
                state.powerplay.merits = e.total_merits;
            }
            Event::PowerplayRank(e) => {
                state.powerplay.rank = Some(e.rank as u8);
            }
            Event::PowerplayFastTrack(_) => {}
            Event::PowerplayCollect(_) => {}
            Event::PowerplayVoucher(_) => {}
            Event::PowerplayVote(_) => {}
            Event::PowerplayDefect(e) => {
                state.powerplay.power = Some(e.to_power);
            }
            Event::PowerplayDeliver(_) => {}
            Event::PowerplaySalary(e) => {
                state.powerplay.last_salary = Some(e.amount);
            }
            Event::PowerplayLeave(_) => {
                state.powerplay = Default::default();
            }

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
                state.nav_route.clear();
            }

            Event::ReceiveText(e) => {
                if state.first_message_timestamp == 0 {
                    state.first_message_timestamp = e.timestamp.timestamp();
                }
                else {
                    state.latest_message_timestamp = e.timestamp.timestamp();
                    state.latest_message_timestamp_formatted = format::prettify_date(&e.timestamp)
                }

                if e.channel.as_ref() != "npc" {
                    state.messages.push(e.into());
                }
            }

            Event::Shutdown(_) => {
                state.nav_route.clear();
            }

            // SHIP LOCKER
            Event::ShipLockerMaterials(_) => {}

            Event::ShipLocker(e) => {
                let is_empty = e.items.is_none()
                    && e.components.is_none()
                    && e.consumables.is_none()
                    && e.data.is_none();

                if !is_empty {
                    state.ship_locker = e.into();
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

            Event::RestockVehicle(e) => state.logs.push(e.into()),

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
            Event::ShipyardBankDeposit(_) => {}

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

            Event::SuitLoadout(e) => state.suit_loadout = e.into(),

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

            Event::BuyAmmo(e) => state.logs.push(log_ship_equipment_purchase(e, "ammo")),

            // WING
            Event::WingAdd(_) => {}
            Event::WingInvite(_) => {}
            Event::WingJoin(_) => {}
            Event::WingLeave(_) => {}
            Event::ShipLockerBackpack(_) => {}
        }

        Task::none()
    }
}
