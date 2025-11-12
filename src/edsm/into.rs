use crate::{edsm, state};

// ------------------------------ Stations ------------------------------
impl Into<Vec<state::navigation::Station>> for edsm::Stations {
    fn into(self) -> Vec<state::navigation::Station> {
        self.stations.unwrap_or_default().into_iter().map(Into::into).collect()
    }
}

impl Into<state::navigation::Station> for edsm::Station {
    fn into(self) -> state::navigation::Station {
        state::navigation::Station {
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

impl Into<state::navigation::StationBody> for edsm::stations::Body {
    fn into(self) -> state::navigation::StationBody {
        state::navigation::StationBody {
            id: self.id,
            name: self.name,
            latitude: self.latitude,
            longitude: self.longitude,
        }
    }
}

impl Into<state::navigation::LastUpdated> for edsm::UpdateTime {
    fn into(self) -> state::navigation::LastUpdated {
        state::navigation::LastUpdated {
            information: self.information,
            market: self.market,
            shipyard: self.shipyard,
            outfitting: self.outfitting,
        }
    }
}

// ------------------------------ Bodies ------------------------------
impl Into<Vec<state::fss::Body>> for edsm::Bodies {
    fn into(self) -> Vec<state::fss::Body> {
        self.bodies.unwrap_or_default().into_iter().map(Into::into).collect()
    }
}

impl Into<state::fss::Body> for edsm::bodies::Body {
    fn into(self) -> state::fss::Body {
        state::fss::Body {
            id: self.body_id as u8,
            name: self.name,
            parent_id: None,
            r#type: None,
            signals: Vec::new(),
            terraform_state: self.terraforming_state,
            was_discovered: self.discovery.is_some(),
            was_mapped: false
        }
    }
}

// ------------------------------ Counts ------------------------------
impl Into<state::navigation::Counts> for edsm::Counts {
    fn into(self) -> state::navigation::Counts {
        state::navigation::Counts { day: self.day, week: self.week, total: self.total }
    }
}

impl Into<state::navigation::Counts> for edsm::Traffic {
    fn into(self) -> state::navigation::Counts { self.traffic.unwrap_or_default().into() }
}

impl Into<state::navigation::Counts> for edsm::Deaths {
    fn into(self) -> state::navigation::Counts { self.deaths.unwrap_or_default().into() }
}