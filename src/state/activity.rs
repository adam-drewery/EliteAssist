use chrono::{DateTime, Utc};
use ed_journals::logs::change_crew_role_event::{ChangeCrewRoleEvent, ChangeCrewRoleEventRole};
use ed_journals::logs::crew_assign_event::{CrewAssignEvent, CrewAssignEventRole};
use ed_journals::logs::crew_launch_fighter_event::CrewLaunchFighterEvent;
use ed_journals::logs::crew_member_role_change_event::{CrewMemberRoleChangeEvent, CrewMemberRoleChangeEventRole};
use ed_journals::logs::disembark_event::DisembarkEvent;
use ed_journals::logs::dock_fighter_event::DockFighterEvent;
use ed_journals::logs::embark_event::EmbarkEvent;
use ed_journals::logs::end_crew_session_event::EndCrewSessionEvent;
use ed_journals::logs::fighter_rebuilt_event::FighterRebuiltEvent;
use ed_journals::logs::launch_fighter_event::LaunchFighterEvent;
use ed_journals::logs::npc_crew_rank_event::NPCCrewRankEvent;
use ed_journals::logs::npc_crew_wage_paid_event::NPCCrewWagePaidEvent;
use ed_journals::logs::restock_vehicle_event::RestockVehicleEvent;
use ed_journals::logs::start_jump_event::{StartJumpEvent, StartJumpType};
use ed_journals::logs::vehicle_switch_event::{VehicleSwitchEvent, VehicleSwitchEventTo};
use crate::journal::format::prettify_date;
use thousands::Separable;

pub struct GameEventLog {
    pub time_display: String,
    pub verb: String,
    pub noun: String,
}

impl GameEventLog {
    pub fn from_embark(value: EmbarkEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Embarked".to_owned(),
            noun: join_location_parts(&value.star_system, &value.body, &value.station_name),
        }
    }

    pub fn from_disembark(value: DisembarkEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Disembarked".to_owned(),
            noun: join_location_parts(&value.star_system, &value.body, &value.station_name),
        }
    }

    pub fn from_start_jump(value: StartJumpEvent, timestamp: DateTime<Utc>) -> Self {
        match value.jump {
            StartJumpType::Supercruise => GameEventLog {
                time_display: prettify_date(&timestamp),
                verb: "".into(),
                noun: "Entered supercruise".into(),
            },
            StartJumpType::Hyperspace { star_system, system_address: _, star_class } => GameEventLog {
                time_display: prettify_date(&timestamp),
                verb: "Jumped to".into(),
                noun: format!(
                    "{} ({})",
                    star_system,
                    star_class
                ),
            }
        }
    }

    pub fn from_crew_assign(value: CrewAssignEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Assigned".into(),
            noun: match value.role {
                CrewAssignEventRole::Active => {
                    format!("{} as active", value.name)
                }
                CrewAssignEventRole::OnShoreLeave => {
                    format!("{} to shore leave", value.name)
                }
            },
        }
    }

    pub fn from_crew_member_role_change(value: CrewMemberRoleChangeEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Assigned role".to_string(),
            noun: format!("{} to {}", match value.role {
                CrewMemberRoleChangeEventRole::Idle => "Idle".to_string(),
                CrewMemberRoleChangeEventRole::FireCon => "Fire con".to_string(),
                CrewMemberRoleChangeEventRole::FighterCon => "Fighter con".to_string(),
                CrewMemberRoleChangeEventRole::OnFoot => "On foot".to_string(),
                CrewMemberRoleChangeEventRole::Helm => "Helm".to_string(),
            }, value.crew),
        }
    }

    pub fn from_end_crew_session(value: EndCrewSessionEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Ended".into(),
            noun: if value.telepresence { "remote session".into() } else { "crew session".into() },
        }
    }

    pub fn from_npc_crew_rank(value: NPCCrewRankEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Promoted crew member".into(),
            noun: value.npc_crew_name,
        }
    }

    pub fn from_change_crew_role(value: ChangeCrewRoleEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Changed role to".into(),
            noun: match value.role {
                ChangeCrewRoleEventRole::Idle => "Idle".into(),
                ChangeCrewRoleEventRole::FireCon => "Fire con".into(),
                ChangeCrewRoleEventRole::FighterCon => "Fighter con".into(),
                ChangeCrewRoleEventRole::OnFoot => "On foot".into(),
                ChangeCrewRoleEventRole::Helm => "Helm".into(),
            },
        }
    }

    pub fn from_npc_crew_wage_paid(value: NPCCrewWagePaidEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Paid".into(),
            noun: format!("{} to {}", value.amount.separate_with_commas(), value.npc_crew_name),
        }
    }

    pub fn from_fighter_rebuilt(value: FighterRebuiltEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Rebuilt".into(),
            noun: format!("Fighter {}", value.id.to_string()),
        }
    }

    pub fn from_dock_fighter(value: DockFighterEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Docked".into(),
            noun: format!("Fighter {}", value.id.to_string()),
        }
    }

    pub fn from_crew_launch_fighter(value: CrewLaunchFighterEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Launched".into(),
            noun: format!("Fighter by {}", value.name),
        }
    }

    pub fn from_launch_fighter(value: LaunchFighterEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Launched".into(),
            noun: format!("Fighter {}", value.id.to_string()),
        }
    }

    pub fn from_vehicle_switch(value: VehicleSwitchEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Switched to".into(),
            noun: match value.to {
                VehicleSwitchEventTo::Fighter => "Fighter".into(),
                VehicleSwitchEventTo::Mothership => "Mothership".into()
            },
        }
    }

    pub fn from_restock_vehicle(value: RestockVehicleEvent, timestamp: DateTime<Utc>) -> Self {
        GameEventLog {
            time_display: prettify_date(&timestamp),
            verb: "Restocked vehicles for".into(),
            noun: format!("{}CR", value.cost.to_string().separate_with_commas()),
        }
    }
}

fn join_location_parts(system: &String, body: &String, station: &Option<String>) -> String {
    let mut parts = Vec::new();

    if !system.is_empty() {
        parts.push(system.as_str());
    }
    if !body.is_empty() {
        parts.push(body.as_str());
    }
    if let Some(station) = station {
        if !station.is_empty() && !Some(station.to_string()).eq(&Some(body.to_string())) {
            parts.push(station.as_str());
        }
    }
    parts.join(" | ")
}
