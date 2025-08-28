use crate::journal::event;
use crate::journal::format::prettify_date;
use thousands::Separable;

pub struct GameEventLog {
    pub time_display: Box<str>,
    pub verb: Box<str>,
    pub noun: Box<str>,
}

impl From<event::Embark> for GameEventLog {
    fn from(value: event::Embark) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp).into(),
            verb: "Embarked".into(),
            noun: join_location_parts(&value.star_system, &value.body, value.station_name.as_deref()),
        }
    }
}

impl From<event::Disembark> for GameEventLog {
    fn from(value: event::Disembark) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Disembarked".into(),
            noun: join_location_parts(&value.star_system, &value.body, value.station_name.as_deref()),
        }
    }
}

impl From<event::StartJump> for GameEventLog {
    fn from(value: event::StartJump) -> Self {
        match value.jump_type.as_ref() {
            "Supercruise" => GameEventLog {
                time_display: prettify_date(&value.timestamp),
                verb: "".into(),
                noun: "Entered supercruise".into(),
            },
            "Hyperspace" => GameEventLog {
                time_display: prettify_date(&value.timestamp),
                verb: "Jumped to".into(),
                noun: format!(
                    "{} ({})",
                    value.star_system.unwrap_or_default(),
                    value.star_class.unwrap_or_default()
                ).into(),
            },
            _ => panic!("Unknown jump type"),
        }
    }
}

// Crew-related events
impl From<event::CrewAssign> for GameEventLog {
    fn from(value: event::CrewAssign) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Assigned".into(),
            noun: format!("{} as {}", value.name, value.role).into(),
        }
    }
}

impl From<event::CrewMemberRoleChange> for GameEventLog {
    fn from(value: event::CrewMemberRoleChange) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Assigned role".into(),
            noun: format!("{} to {}", value.role, value.crew).into(),
        }
    }
}

impl From<event::EndCrewSession> for GameEventLog {
    fn from(value: event::EndCrewSession) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Ended".into(),
            noun: if value.telepresence.is_some_and(|x| x) { "remote session".into() } else { "crew session".into() },
        }
    }
}

impl From<event::NpcCrewRank> for GameEventLog {
    fn from(value: event::NpcCrewRank) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Promoted crew member".into(),
            noun: value.npc_crew_name,
        }
    }
}

impl From<event::ChangeCrewRole> for GameEventLog {
    fn from(value: event::ChangeCrewRole) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Changed role to".into(),
            noun: value.role,
        }
    }
}

impl From<event::NpcCrewPaidWage> for GameEventLog {
    fn from(value: event::NpcCrewPaidWage) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Paid".into(),
            noun: format!("{} to {}", value.amount.separate_with_commas(), value.npc_crew_name).into(),
        }
    }
}

// Fighter-related events
impl From<event::FighterRebuilt> for GameEventLog {
    fn from(value: event::FighterRebuilt) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Rebuilt".into(),
            noun: format!("Fighter {}", value.id.to_string()).into(),
        }
    }
}

impl From<event::DockFighter> for GameEventLog {
    fn from(value: event::DockFighter) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Docked".into(),
            noun: format!("Fighter {}", value.id.to_string()).into(),
        }
    }
}

impl From<event::CrewLaunchFighter> for GameEventLog {
    fn from(value: event::CrewLaunchFighter) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Launched".into(),
            noun: format!("Fighter by {}", value.crew).into(),
        }
    }
}

impl From<event::LaunchFighter> for GameEventLog {
    fn from(value: event::LaunchFighter) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Launched".into(),
            noun: format!("Fighter {}", value.id.to_string()).into(),
        }
    }
}

impl From<event::VehicleSwitch> for GameEventLog {
    fn from(value: event::VehicleSwitch) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Switched to".into(),
            noun: value.to,
        }
    }
}

// Cargo/Restock
impl From<event::RestockVehicle> for GameEventLog {
    fn from(value: event::RestockVehicle) -> Self {
        GameEventLog {
            time_display: prettify_date(&value.timestamp),
            verb: "Restocked vehicles for".into(),
            noun: format!("{}CR", value.cost.to_string().separate_with_commas()).into(),
        }
    }
}

fn join_location_parts(system: &str, body: &str, station: Option<&str>) -> Box<str> {
    let mut parts = Vec::new();

    if !system.is_empty() {
        parts.push(system);
    }
    if !body.is_empty() {
        parts.push(body);
    }
    if let Some(station) = station {
        if !station.is_empty() && !Some(station.to_string()).eq(&Some(body.to_string())) {
            parts.push(station);
        }
    }
    parts.join(" | ").as_str().into()
}
pub fn log_ship_equipment_purchase(e: event::ShipMaintenance, item: &str) -> GameEventLog {
    GameEventLog {
        time_display: prettify_date(&e.timestamp),
        verb: format!("Bought {} for", item).into(),
        noun: format!("{}CR", e.cost.to_string().separate_with_commas()).into(),
    }
}

pub fn log_crew_member(e: event::CrewMember, verb: &str) -> GameEventLog {
    GameEventLog {
        time_display: prettify_date(&e.timestamp),
        verb: format!("Crew {}", verb).into(),
        noun: format!(
            "{} {}",
            e.crew,
            if e.telepresence.is_some_and(|x| x) { "remotely" } 
            else { "to crew" }
        ).into(),
    }
}

pub fn log_damage(e: event::Damage, verb: &str, noun: &str) -> GameEventLog {
    GameEventLog {
        time_display: prettify_date(&e.timestamp),
        verb: verb.into(),
        noun: match e.id {
            None => noun.into(),
            Some(id) => format!("{} {}", noun, id).into(),
        },
    }
}
