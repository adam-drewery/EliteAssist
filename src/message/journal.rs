use crate::state::*;
use crate::journal;
use crate::journal::format;
use crate::query;
use iced::Task;
use log::warn;
use thousands::Separable;
use crate::journal::Event::*;
use crate::message::Message;

impl journal::Event {
    pub fn update(self, state: &mut State) -> Task<Message> {

        match self {
            // BACKPACK
            Backpack(_) => {}
            BackpackChange(_) => {}
            DropItems(_) => {}
            CollectItems(_) => {}
            UseConsumable(_) => {}

            // CARGO
            Cargo(_) => {}
            CargoTransfer(_) => {}
            CargoDepot(_) => {}
            CollectCargo(_) => {}
            EjectCargo(_) => {}

            // CARRIER
            CarrierLocation(_) => {}
            CarrierJump(_) => {}
            CarrierBuy(_) => {}
            CarrierStats(_) => {}
            CarrierJumpRequest(_) => {}
            CarrierDecommission(_) => {}
            CarrierCancelDecommission(_) => {}
            CarrierBankTransfer(_) => {}
            CarrierDepositFuel(_) => {}
            CarrierCrewServices(_) => {}
            CarrierFinance(_) => {}
            CarrierShipPack(_) => {}
            CarrierModulePack(_) => {}
            CarrierTradeOrder(_) => {}
            CarrierDockingPermission(_) => {}
            CarrierNameChange(_) => {}
            CarrierJumpCancelled(_) => {}
            FCMaterials(_) => {}

            // COLONISATION
            ColonisationBeaconDeployed(_) => {}
            ColonisationConstructionDepot(_) => {}
            ColonisationContribution(_) => {}
            ColonisationSystemClaim(_) => {}
            ColonisationSystemClaimRelease(_) => {}

            // COMBAT
            CapShipBond(_) => {}
            UnderAttack(_) => {}
            PVPKill(_) => {}

            FactionKillBond(e) => {
                state.combat_bonds
                    .entry(e.awarding_faction.clone())
                    .and_modify(|v| *v = v.saturating_add(e.reward as u32))
                    .or_insert(e.reward as u32);
            }

            Bounty(e) => {
                for bounty in e.rewards.unwrap_or_default() {
                    state.bounties
                        .entry(bounty.faction.clone())
                        .and_modify(|v| *v = v.saturating_add(bounty.reward as u32))
                        .or_insert(bounty.reward as u32);
                }
            }

            // COMMUNITY GOAL
            CommunityGoalJoin(_) => {}
            CommunityGoalDiscard(_) => {}
            CommunityGoalReward(_) => {}
            CommunityGoal(_) => {}
            ScientificResearch(_) => {}

            // CREW
            QuitACrew(_) => {}
            JoinACrew(_) => {}
            CrewFire(_) => {}
            CrewHire(_) => {}
            KickCrewMember(_) => {}

            CrewAssign(e) => state.logs.push(e.into()),

            CrewMemberRoleChange(e) => state.logs.push(e.into()),

            CrewLaunchFighter(e) => state.logs.push(e.into()),

            ChangeCrewRole(e) => state.logs.push(e.into()),

            EndCrewSession(e) => state.logs.push(e.into()),

            NpcCrewRank(e) => state.logs.push(e.into()),

            CrewMemberJoins(e) => state.logs.push(log_crew_member(e, "joined")),

            CrewMemberQuits(e) => state.logs.push(log_crew_member(e, "quit")),

            NpcCrewPaidWage(e) => {
                if e.amount != 0 {
                    state.logs.push(e.into())
                }
            }

            // CRIME
            ClearImpound(_) => {}
            CommitCrime(_) => {}
            CrimeVictim(_) => {}
            PayBounties(_) => {}
            PayFines(_) => {}
            HoloscreenHacked(_) => {}

            // DATA MARKET
            SellExplorationData(_) => {}
            BuyExplorationData(_) => {}
            BuyTradeData(_) => {}
            SellOrganicData(_) => {}
            MultiSellExplorationData(_) => {}

            RedeemVoucher(e) => {
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
            EngineerLegacyConvert(_) => {}
            EngineerContribution(_) => {}
            EngineerCraft(_) => {}

            EngineerProgress(e) => state.engineers = e.into(),

            // ENVIRONMENT
            JetConeDamage(_) => {}
            CockpitBreached(_) => {}
            HeatWarning(_) => {}
            HeatDamage(_) => {}
            ShipTargeted(_) => {}
            HullDamage(_) => {}
            SelfDestruct(_) => {}
            SystemsShutdown(_) => {}
            ShieldState(_) => {}
            LaunchDrone(_) => {}
            DatalinkVoucher(_) => {}
            Scanned(_) => {}

            // FIGHTER
            VehicleSwitch(e) => state.logs.push(e.into()),

            LaunchFighter(e) => state.logs.push(e.into()),

            FighterRebuilt(e) => state.logs.push(e.into()),

            DockFighter(e) => state.logs.push(e.into()),

            FighterDestroyed(e) => {
                state.logs.push(log_damage(e, "Destroyed", "Fighter"))
            }

            // FSD
            Interdiction(_) => {}
            Interdicted(_) => {}
            EscapeInterdiction(_) => {}
            SupercruiseEntry(_) => {}
            SupercruiseExit(_) => {}
            SupercruiseDestinationDrop(_) => {}

            FSDTarget(_) => {}

            StartJump(e) => state.logs.push(e.into()),

            FSDJump(e) => {
                // trim matching systems from the start of our nav route 
                state.trim_nav_route(e.system_address);

                state.location.body_name = String::new().into();
                state.location = e.into();
                state.fss = Default::default();

                if state.journal_loaded {
                    return query::system(
                        state.location.system_name.as_ref(),
                        state.ship_loadout.max_jump_range);
                }
            }

            // FUEL
            FuelScoop(_) => {}
            ReservoirReplenished(_) => {}

            // MARKET
            MarketBuy(_) => {}
            MarketSell(_) => {}
            TechnologyBroker(_) => {}

            Market(e) => {
                if !e.items.is_none() {
                    state.market = e.into();
                }
            }

            // MATERIALS
            MaterialDiscarded(_) => {}
            MaterialCollected(_) => {}
            MaterialDiscovered(_) => {}
            MaterialTrade(_) => {}
            Synthesis(_) => {}

            Materials(e) => {
                let is_empty = e.encoded.is_empty()
                    && e.manufactured.is_empty()
                    && e.raw.is_empty();

                if !is_empty {
                    state.materials = e.into();
                }
            }

            // MICRO RESOURCES
            RequestPowerMicroResources(_) => {}
            TransferMicroResources(_) => {}
            DeliverPowerMicroResources(_) => {}
            SellMicroResources(_) => {}
            TradeMicroResources(_) => {}
            BuyMicroResources(_) => {}

            // MINING
            ProspectedAsteroid(_) => {}
            AsteroidCracked(_) => {}
            MiningRefined(_) => {}

            // MISSIONS
            Missions(_) => { /* this doesn't give us all the info we need */ }
            MissionRedirected(_) => {}

            MissionAccepted(e) => state.missions.push(e.into()),

            MissionFailed(e) => state.missions.retain(|m| m.mission_id != e.mission_id),

            MissionAbandoned(e) => state.missions.retain(|m| m.mission_id != e.mission_id),

            MissionCompleted(e) => state.missions.retain(|m| m.mission_id != e.mission_id),

            // NAVIGATION
            ApproachBody(_) => {}
            LeaveBody(_) => {}
            ApproachSettlement(_) => {}
            DockingRequested(_) => {}
            DockingGranted(_) => {}
            DockingTimeout(_) => {}
            DockingDenied(_) => {}
            DockingCancelled(_) => {}
            USSDrop(_) => {}
            Touchdown(_) => {}
            Liftoff(_) => {}
            Undocked(_) => {}
            JetConeBoost(_) => {}

            NavRoute(e) => {
                let route: Vec<NavRouteStep> = e.into();

                // The journal file gives us blank NavRoute events when we plot one. Kinda weird.
                if !route.is_empty() {
                    state.nav_route = route;
                }
            }

            NavRouteClear(_) => {
                state.nav_route.clear();
            }

            Disembark(e) => {
                state.location.body_name = e.body.clone();
                state.logs.push(e.into());
            }

            Embark(e) => {
                state.location.body_name = e.body.clone();
                state.logs.push(e.into());
            }

            Docked(e) => {
                if let Some(active_fine) = e.active_fine {
                    state.crime.active_fine = active_fine;
                }
                if let Some(wanted) = e.wanted {
                    state.crime.wanted = wanted;
                }
            }

            Location(e) => {
                state.location.system_name = e.star_system.clone();

                if e.body_type.as_ref() != "Star" {
                    state.location.body_name = e.body.clone();
                }

                state.location = e.into();
            }

            // OUTFITTING
            Outfitting(_) => {}
            ModuleInfo(_) => {}
            ModuleBuyAndStore(_) => {}
            ModuleSell(_) => {}
            ModuleStore(_) => {}
            ModuleRetrieve(_) => {}
            MassModuleStore(_) => {}
            ModuleSwap(_) => {}
            ModuleBuy(_) => {}
            ModuleSellRemote(_) => {}
            FetchRemoteModule(_) => {}
            StoredModules(_) => {}

            Loadout(e) => state.ship_loadout = e.into(),

            // PASSENGERS
            Passengers(_) => {}
            SearchAndRescue(_) => {}

            // PERSONAL
            Statistics(_) => {}
            Promotion(promotion) => {
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

            Commander(commander) => {
                state.commander_name = ("CMDR ".to_string() + commander.name.as_ref()).into();
            }

            Status(e) => {
                if let Some(balance) = e.balance {
                    state.credits = (balance.separate_with_commas() + " CR").into();
                }
                if let Some(legal_state) = e.legal_state {
                    state.crime.legal_state = legal_state;
                }

                if e.body_name.is_some() {
                    state.location.body_name = e.body_name.unwrap()
                }
            }

            Rank(e) => state.rank = e.into(),

            Progress(e) => state.progress = e.into(),

            Reputation(e) => state.reputation = e.into(),

            // POWERPLAY
            Powerplay(e) => {
                state.powerplay.power = Some(e.power);
                state.powerplay.rank = Some(e.rank as u8);
                state.powerplay.merits = e.merits;
                state.powerplay.time_pledged = e.time_pledged;
            }

            PowerplayJoin(e) => {
                state.powerplay.power = Some(e.power);
            }

            PowerplayMerits(e) => {
                state.powerplay.merits = e.total_merits;
            }

            PowerplayRank(e) => {
                state.powerplay.rank = Some(e.rank as u8);
            }

            PowerplayFastTrack(_) => {}
            PowerplayCollect(_) => {}
            PowerplayVoucher(_) => {}
            PowerplayVote(_) => {}

            PowerplayDefect(e) => {
                state.powerplay.power = Some(e.to_power);
            }

            PowerplayDeliver(_) => {}
            PowerplaySalary(e) => {
                state.powerplay.last_salary = Some(e.amount);
            }

            PowerplayLeave(_) => {
                state.powerplay = Default::default();
            }

            // SCAN
            Scan(event) => {

                if let Some(progress) = &mut state.fss.progress {
                    progress.body_count += 1;
                }

                let body = state.fss.bodies
                    .entry(event.body_id)
                    .or_insert_with(|| ScannedBody::default());

                body.parent_id = ScannedBody::get_parent_id(&event);
                body.terraform_state = event.terraform_state;
                body.was_discovered = event.was_discovered;
                body.was_mapped = event.was_mapped;
            }

            ScanBaryCentre(_) => {}
            ScanOrganic(_) => {}
            CodexEntry(_) => {}
            DatalinkScan(_) => {}
            NavBeaconScan(_) => {}
            DiscoveryScan(_) => {}
            DataScanned(_) => {}

            FSSBodySignals(e) => {
                let body = state.fss.bodies.entry(e.body_id).or_default();

                body.signals = e.signals.into_iter().map(|s|{
                    SignalCount {
                        kind: s.type_localised.unwrap_or(s.r#type),
                        count: s.count as u32
                    }
                }).collect()
            }

            FSSDiscoveryScan(e) => {
                state.fss.progress = Some(e.into());
            }

            FSSAllBodiesFound(_) => {}

            FSSSignalDiscovered(e) => {
                state.fss.signals.push(e.into());
            }

            SAASignalsFound(_) => {}
            SAAScanComplete(_) => {}

            // SESSION
            Continued(_) => {}
            NewCommander(_) => {}
            Friends(_) => {}
            ClearSavedGame(_) => {}
            Screenshot(_) => {}
            Fileheader(_) => {}
            SendText(_) => {}
            Died(_) => {}
            Resurrect(_) => {}
            Music(_) => {}

            LoadGame(_) => {
                state.nav_route.clear();
            }

            ReceiveText(e) => {
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

            Shutdown(_) => {}

            // SHIP LOCKER
            ShipLockerMaterials(_) => warn!("Ship locker materials is supposed to be discontinued."),

            ShipLocker(e) => {
                let is_empty = e.items.is_none()
                    && e.components.is_none()
                    && e.consumables.is_none()
                    && e.data.is_none();

                if !is_empty {
                    state.ship_locker = e.into();
                }
            }

            // SHIP MAINTENANCE
            RefuelAll(_) => {}
            RefuelPartial(_) => {}
            RepairAll(_) => {}
            Repair(_) => {}
            Resupply(_) => {}
            BuyDrones(_) => {}
            RepairDrone(_) => {}
            SellDrones(_) => {}
            RebootRepair(_) => {}
            AfmuRepairs(_) => {}

            RestockVehicle(e) => state.logs.push(e.into()),

            // SHIPYARD
            Shipyard(_) => {}
            ShipyardNew(_) => {}
            ShipyardRedeem(_) => {}
            ShipyardBuy(_) => {}
            ShipRedeemed(_) => {}
            ShipyardSwap(_) => {}
            ShipyardSell(_) => {}
            ShipyardTransfer(_) => {}
            SellShipOnRebuy(_) => {}
            StoredShips(_) => {}
            SetUserShipName(_) => {}
            ShipyardBankDeposit(_) => {}

            // SQUADRON
            SquadronStartup(_) => {}
            SquadronCreated(_) => {}
            SquadronDemotion(_) => {}
            SquadronPromotion(_) => {}
            DisbandedSquadron(_) => {}
            InvitedToSquadron(_) => {}
            AppliedToSquadron(_) => {}
            JoinedSquadron(_) => {}
            KickedFromSquadron(_) => {}
            LeftSquadron(_) => {}
            SharedBookmarkToSquadron(_) => {}

            // SRV
            DockSRV(_) => {}
            LaunchSRV(_) => {}
            SRVDestroyed(_) => {}

            // SUIT LOADOUT
            BuySuit(_) => {}
            SellSuit(_) => {}
            UpgradeSuit(_) => {}
            CreateSuitLoadout(_) => {}
            RenameSuitLoadout(_) => {}
            DeleteSuitLoadout(_) => {}

            SwitchSuitLoadout(e) => state.suit_loadout = e.into(),

            SuitLoadout(e) => state.suit_loadout = e.into(),

            // TAXI
            BookTaxi(_) => {}
            CancelTaxi(_) => {}
            BookDropship(_) => {}
            CancelDropship(_) => {}
            DropshipDeploy(_) => {}

            // WEAPON
            BuyWeapon(_) => {}
            SellWeapon(_) => {}
            UpgradeWeapon(_) => {}
            LoadoutRemoveModule(_) => {}
            LoadoutEquipModule(_) => {}

            BuyAmmo(e) => state.logs.push(log_ship_equipment_purchase(e, "ammo")),

            // WING
            WingAdd(_) => {}
            WingInvite(_) => {}
            WingJoin(_) => {}
            WingLeave(_) => {}
            ShipLockerBackpack(_) => {}
        }

        Task::none()
    }
}
