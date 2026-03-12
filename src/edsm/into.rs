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
        let is_ammonia_world = self.atmosphere_type.as_ref().is_some_and(|atm| atm.as_ref() == "Ammonia");
        let is_water_world = self.sub_type.as_ref() == "Water world";
        let is_earthlike = self.sub_type.as_ref() == "Earthlike body";
        let is_high_metal_content = self.sub_type.as_ref() == "High metal content body";
        let is_gas_giant = self.sub_type.as_ref().to_lowercase().contains("gas giant");

        let parent_id = self.parents.as_ref()
            .and_then(|p| p.first())
            .and_then(|f| {
                let (key, &val) = f.iter().next()?;
                if key.as_ref() == "Null" {
                    None
                } else {
                    Some(val as u8)
                }
            });

        state::fss::Body {
            id: self.body_id as u8,
            name: self.name,
            parent_id,
            r#type: Some(self.sub_type).filter(|s| !s.is_empty()),
            signals: Vec::new(),
            terraformable: self.terraforming_state.as_deref() == Some("Terraformable"),
            was_discovered: self.discovery.is_some(),
            discovery: self.discovery.map(|d| state::fss::BodyDiscovery {
                commander: d.commander,
                date: d.date,
            }),
            was_mapped: false,
            was_footfalled: false,
            atmosphere: self.atmosphere_type.clone(),
            atmosphere_type: self.atmosphere_type,
            volcanism: self.volcanism_type,
            is_landable: self.is_landable.unwrap_or_default(),
            rings: self.rings.unwrap_or_default().into_iter().map(|r| r.name).collect(),
            is_ammonia_world,
            is_water_world,
            is_high_metal_content,
            is_gas_giant,
            is_earthlike,
            has_life: false,
            distance_ls: self.distance_to_arrival,
            is_journal_scan: false,
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