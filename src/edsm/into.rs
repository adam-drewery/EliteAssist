use crate::{edsm, state};

// ------------------------------ Stations ------------------------------
impl Into<Vec<state::Station>> for edsm::Stations {
    fn into(self) -> Vec<state::Station> {
        self.stations.into_iter().map(Into::into).collect()
    }
}

impl Into<state::Station> for edsm::Station {
    fn into(self) -> state::Station {
        state::Station {
            id: self.id,
            market_id: self.market_id,
            type_field: self.r#type,
            name: self.name,
            body: self.body.map(Into::into),
            distance_to_arrival: self.distance_to_arrival,
            allegiance: self.allegiance,
            government: self.government,
            economy: self.economy,
            second_economy: self.second_economy,
            have_market: self.have_market,
            have_shipyard: self.have_shipyard,
            have_outfitting: self.have_outfitting,
            other_services: self.other_services,
            controlling_faction: self.controlling_faction.into(),
            update_time: self.update_time.into(),
        }
    }
}

impl Into<state::StationBody> for edsm::stations::Body {
    fn into(self) -> state::StationBody {
        state::StationBody {
            id: self.id,
            name: self.name,
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

impl Into<state::FactionRef> for edsm::stations::ControllingFaction {
    fn into(self) -> state::FactionRef {
        state::FactionRef { id: self.id, name: self.name }
    }
}

impl Into<state::StationUpdateTime> for edsm::UpdateTime {
    fn into(self) -> state::StationUpdateTime {
        state::StationUpdateTime {
            information: self.information,
            market: self.market,
            shipyard: self.shipyard,
            outfitting: self.outfitting,
        }
    }
}

// ------------------------------ System ------------------------------
impl Into<state::SystemMeta> for edsm::System {
    fn into(self) -> state::SystemMeta {
        state::SystemMeta {
            coords: vec![self.coords.x, self.coords.y, self.coords.z],
            permit_required: self.require_permit,
            primary_star: self.primary_star.into(),
        }
    }
}

impl Into<state::PrimaryStarMeta> for edsm::PrimaryStar {
    fn into(self) -> state::PrimaryStarMeta {
        state::PrimaryStarMeta {
            type_field: self.type_field,
            name: self.name,
            is_scoopable: self.is_scoopable,
        }
    }
}

// ------------------------------ Bodies ------------------------------
impl Into<Vec<state::BodyInfo>> for edsm::Bodies {
    fn into(self) -> Vec<state::BodyInfo> {
        self.bodies.into_iter().map(Into::into).collect()
    }
}

impl Into<state::BodyInfo> for edsm::bodies::Body {
    fn into(self) -> state::BodyInfo {
        state::BodyInfo {
            name: self.name,
            type_field: self.body_type,
            sub_type: self.sub_type,
            distance_to_arrival: self.distance_to_arrival,
            is_main_star: self.is_main_star.unwrap_or(false),
            is_scoopable: self.is_scoopable.unwrap_or(false),
        }
    }
}

// ------------------------------ Factions ------------------------------
impl Into<state::FactionsMeta> for edsm::Factions {
    fn into(self) -> state::FactionsMeta {
        state::FactionsMeta {
            controlling_faction: state::FactionRef { id: self.controlling_faction.id as i64, name: self.controlling_faction.name.clone() },
            factions: self.factions.into_iter().map(Into::into).collect(),
        }
    }
}

impl Into<state::FactionExtra> for edsm::Faction {
    fn into(self) -> state::FactionExtra {
        state::FactionExtra {
            id: self.id,
            name: self.name,
            allegiance: self.allegiance,
            government: self.government,
            influence: self.influence as f64,
            state: self.state,
            happiness: self.happiness,
            is_player: self.is_player,
            active_states: self.active_states.into_iter().map(Into::into).collect(),
            pending_states: self.pending_states.into_iter().map(Into::into).collect(),
            recovering_states: self.recovering_states.into_iter().map(Into::into).collect(),
            last_update: self.last_update,
        }
    }
}

impl Into<state::FactionNamedState> for edsm::FactionState {
    fn into(self) -> state::FactionNamedState {
        state::FactionNamedState { state: self.state }
    }
}

impl Into<state::FactionTrend> for edsm::TrendingState {
    fn into(self) -> state::FactionTrend {
        state::FactionTrend { state: self.state, trend: self.trend as i32 }
    }
}

// ------------------------------ Counts ------------------------------
impl Into<state::Counts> for edsm::Counts {
    fn into(self) -> state::Counts {
        state::Counts { day: self.day, week: self.week, total: self.total }
    }
}

impl Into<state::Counts> for edsm::Traffic {
    fn into(self) -> state::Counts { self.traffic.into() }
}

impl Into<state::Counts> for edsm::Deaths {
    fn into(self) -> state::Counts { self.deaths.into() }
}