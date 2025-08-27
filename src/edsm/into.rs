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
            controlling_faction: self.controlling_faction.map(|f| f.name),
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

impl Into<state::LastUpdated> for edsm::UpdateTime {
    fn into(self) -> state::LastUpdated {
        state::LastUpdated {
            information: self.information,
            market: self.market,
            shipyard: self.shipyard,
            outfitting: self.outfitting,
        }
    }
}

// ------------------------------ Bodies ------------------------------
impl Into<Vec<state::Body>> for edsm::Bodies {
    fn into(self) -> Vec<state::Body> {
        self.bodies.into_iter().map(Into::into).collect()
    }
}

impl Into<state::Body> for edsm::bodies::Body {
    fn into(self) -> state::Body {
        state::Body {
            name: self.name,
            type_field: self.body_type,
            sub_type: self.sub_type,
            distance_to_arrival: self.distance_to_arrival,
            is_main_star: self.is_main_star.unwrap_or(false),
            is_scoopable: self.is_scoopable.unwrap_or(false),
        }
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