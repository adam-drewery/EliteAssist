use crate::image::{planet, station};
use crate::journal::event;
use crate::{edsm, image, BoxStrOptionExt};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Fss {
    pub progress: Option<ScanProgress>,
    pub bodies: HashMap<u8, Body>,
    pub signals: Vec<Signal>,
}

#[derive(Default, Clone, Debug)]
pub struct ScanProgress {
    pub progress: u8,
    pub body_count: u8,
    pub non_body_count: u8,
}

impl From<event::FSSDiscoveryScan> for ScanProgress {
    fn from(value: event::FSSDiscoveryScan) -> Self {
        Self {
            progress: value.progress as u8,
            body_count: value.body_count as u8,
            non_body_count: value.non_body_count as u8,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Body {
    pub id: u8,
    pub name: Box<str>,
    pub r#type: Option<Box<str>>,
    pub parent_id: Option<u8>,
    pub signals: Vec<SignalCount>,
    pub terraformable: bool,
    pub was_discovered: bool,
    pub was_mapped: bool,
    pub was_footfalled: bool,
    pub atmosphere: Option<Box<str>>,
    pub atmosphere_type: Option<Box<str>>,
    pub volcanism: Option<Box<str>>,
    pub is_landable: bool,
    pub rings: Vec<Box<str>>,
    pub is_ammonia_world: bool,
    pub is_water_world: bool,
    pub is_high_metal_content: bool,
    pub is_gas_giant: bool,
    pub is_earthlike: bool,
    pub has_life: bool,
    pub distance_ls: f64,
    pub is_journal_scan: bool,
}

pub struct BodyIcon {
    pub data: &'static [u8],
    pub tooltip: &'static str,
}

impl Body {
    pub fn icons(&self) -> Vec<BodyIcon> {
        let mut result = Vec::new();

        if self.is_earthlike || self.is_water_world || self.is_ammonia_world || self.terraformable {
            result.push(BodyIcon { data: planet::HIGH_VALUE, tooltip: "High Value Body" });
        }

        if self.is_earthlike { result.push(BodyIcon { data: planet::EARTHLIKE, tooltip: "Earth-like World" }); }
        else if self.is_water_world { result.push(BodyIcon { data: planet::WATER_WORLD, tooltip: "Water World" }); }
        else if self.is_ammonia_world { result.push(BodyIcon { data: planet::AMMONIA_WORLD, tooltip: "Ammonia World" }); }
        else if self.is_gas_giant { result.push(BodyIcon { data: planet::GAS_GIANT, tooltip: "Gas Giant" }); }
        else if self.is_high_metal_content { result.push(BodyIcon { data: planet::HIGH_METAL_CONTENT, tooltip: "High Metal Content Body" }); }
        else if self.r#type.is_some() { result.push(BodyIcon { data: planet::PLANET, tooltip: "Planet" }); }
        else { result.push(BodyIcon { data: planet::EMPTY, tooltip: "Unknown Body" }); }

        if self.terraformable { result.push(BodyIcon { data: planet::TERRAFORMABLE, tooltip: "Terraformable" }); }

        if self.is_landable && self.atmosphere.is_some() {
            result.push(BodyIcon { data: planet::ATMOSPHERE_LANDABLE, tooltip: "Landable with Atmosphere" });
        } else {
            if self.is_landable { result.push(BodyIcon { data: planet::LANDABLE, tooltip: "Landable" }); }
            if self.atmosphere.is_some() { result.push(BodyIcon { data: planet::ATMOSPHERE, tooltip: "Atmosphere Present" }); }
        }
        if self.was_footfalled { result.push(BodyIcon { data: planet::LANDER, tooltip: "First Footfall" }); }

        if !self.rings.is_empty() { result.push(BodyIcon { data: planet::RINGED, tooltip: "Ringed" }); }
        if self.volcanism.is_some() { result.push(BodyIcon { data: planet::VOLCANIC, tooltip: "Volcanism Detected" }); }

        if self.has_ammonia_based_life() { result.push(BodyIcon { data: planet::AMMONIA_BASED_LIFE, tooltip: "Ammonia-based Life" }); }
        else if self.has_water_based_life() { result.push(BodyIcon { data: planet::WATER_BASED_LIFE, tooltip: "Water-based Life" }); }
        else if self.has_life { result.push(BodyIcon { data: planet::LIFE, tooltip: "Biological Signals" }); }

        result
    }

    pub fn has_ammonia_based_life(&self) -> bool {
        self.is_ammonia_world && self.has_life
    }

    pub fn has_water_based_life(&self) -> bool {
        self.has_life && self.is_water_world
    }

    // todo: not u64 plz
    pub fn get_parent_id(event: &event::Scan) -> Option<u8> {
        if let Some(parents) = &event.parents {
            // ignore rings, they're annoying.
            let planet_ids: Vec<u64> = parents.iter().filter_map(|p| p.planet).collect();
            let star_ids: Vec<u64> = parents.iter().filter_map(|p| p.star).collect();

            if planet_ids.len() > 0 { Some(planet_ids[0] as u8) }
            else if star_ids.len() > 0 { Some(star_ids[0] as u8) }
            else { None }
        }
        else { None }
    }

    pub fn update_from_scan(&mut self, event: event::Scan) {
        self.is_journal_scan = true;
        self.parent_id = Self::get_parent_id(&event);
        self.name = event.body_name;
        self.id = event.body_id as u8;
        self.was_discovered = event.was_discovered;
        self.was_mapped = event.was_mapped;
        self.terraformable = event.terraform_state.as_deref() == Some("Terraformable");
        self.atmosphere_type = event.atmosphere_type;
        self.volcanism = event.volcanism;
        self.is_landable = event.landable.unwrap_or_default();
        self.distance_ls = event.distance_from_arrival_ls;
        self.r#type = event.planet_class.clone().filter(|s| !s.is_empty());

        self.is_ammonia_world = self.atmosphere_type.as_ref().is_some_and(|atm| atm.as_ref() == "Ammonia");
        self.is_water_world = event.planet_class.as_ref().is_some_and(|pc| pc.as_ref() == "Water world");
        self.is_earthlike = event.planet_class.as_ref().is_some_and(|pc| pc.as_ref() == "Earthlike body");
        self.is_high_metal_content = event.planet_class.as_ref().is_some_and(|pc| pc.as_ref() == "High metal content body");
        self.is_gas_giant = event.planet_class.as_ref().is_some_and(|pc| pc.as_ref().to_lowercase().contains("gas giant"));

        if let Some(rings) = event.rings {
            self.rings = rings.into_iter().map(|r| r.name).collect();
        }

        if self.r#type.is_none() {
            self.r#type = event.star_type.map(|s|
                    format!("{}{:?}", s, event.subclass.unwrap_or_default()).into());
        }
    }

    pub fn update_from_query(&mut self, response: edsm::bodies::Body) {
        if self.is_journal_scan {
            return;
        }

        self.name = response.name;
        self.id = response.body_id as u8;
        self.was_discovered = response.discovery.is_some();
        self.distance_ls = response.distance_to_arrival;
        self.is_landable = response.is_landable.unwrap_or_default();
        self.terraformable = response.terraforming_state.as_deref() == Some("Terraformable");
        self.atmosphere = response.atmosphere_type.clone();
        self.atmosphere_type = response.atmosphere_type.clone();
        self.volcanism = response.volcanism_type.clone();

        self.is_ammonia_world = response.atmosphere_type.as_ref().is_some_and(|atm| atm.as_ref() == "Ammonia");
        self.is_water_world = response.sub_type.as_ref() == "Water world";
        self.is_earthlike = response.sub_type.as_ref() == "Earthlike body";
        self.is_high_metal_content = response.sub_type.as_ref() == "High metal content body";
        self.is_gas_giant = response.sub_type.as_ref().to_lowercase().contains("gas giant");
        self.r#type = Some(response.sub_type).filter(|s| !s.is_empty());

        if let Some(rings) = response.rings {
            self.rings = rings.into_iter().map(|r| r.name).collect();
        }
        if let Some(parents) = response.parents {
            let mut planet_id = None;
            let mut star_id = None;

            for parent in parents.iter() {
                for (key, &val) in parent.iter() {
                    match key.as_ref() {
                        "Planet" => {
                            planet_id = Some(val as u8);
                            break;
                        }
                        "Star" if star_id.is_none() => {
                            star_id = Some(val as u8);
                        }
                        "Null" => continue,
                        _ => {}
                    }
                }
                if planet_id.is_some() {
                    break;
                }
            }

            self.parent_id = planet_id.or(star_id);
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct SignalCount {
    pub kind: Box<str>,
    pub count: u32,
}

impl From<event::Scan> for Body {
    fn from(value: event::Scan) -> Self {
        let parent_id = Self::get_parent_id(&value);
        Self {
            name: value.body_name,
            id: value.body_id as u8,
            terraformable: value.terraform_state.as_deref() == Some("Terraformable"),
            was_discovered: value.was_discovered,
            was_mapped: value.was_mapped,
            signals: Vec::new(),
            parent_id,
            atmosphere: value.atmosphere.none_if_empty(),
            atmosphere_type: value.atmosphere_type.none_if_empty(),
            volcanism: value.volcanism.none_if_empty(),
            is_landable: value.landable.unwrap_or_default(),
            rings: value.rings.unwrap_or_default().into_iter().map(|ring| ring.name).collect(),
            was_footfalled: value.was_footfalled.unwrap_or_default(),
            is_ammonia_world: value.atmosphere_type.is_some_and(|atm| atm.as_ref() == "Ammonia"),
            is_water_world: value.planet_class.as_ref().is_some_and(|pc| pc.as_ref() == "Water world"),
            is_earthlike: value.planet_class.as_ref().is_some_and(|pc| pc.as_ref() == "Earthlike body"),
            is_high_metal_content: value.planet_class.as_ref().is_some_and(|pc| pc.as_ref() == "High metal content body"),
            is_gas_giant: value.planet_class.as_ref().is_some_and(|pc| pc.as_ref().to_lowercase().contains("gas giant")),
            r#type: value.planet_class.filter(|s| !s.is_empty()),
            has_life: false,
            distance_ls: value.distance_from_arrival_ls,
            is_journal_scan: true,
        }
    }
}

impl From<event::FSSBodySignals> for Body {

    fn from(value: event::FSSBodySignals) -> Self {

        Self { 
            id: value.body_id as u8,
            name: value.body_name,
            signals: value.signals.into_iter().map(|sig| {
                SignalCount {
                    count: sig.count as u32,
                    kind: sig.type_localised
                        .filter(|s| !s.is_empty())
                        .unwrap_or(sig.r#type)
                }
            }).collect(),
            ..Default::default()
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct Signal {
    pub name: Box<str>,
    pub kind: Option<Box<str>>,
    pub is_station: bool,
    pub uss_type: Option<Box<str>>,
    pub spawning_state: Option<Box<str>>,
    pub spawning_faction: Option<Box<str>>,
    pub threat_level: Option<u32>,
    pub time_remaining: Option<f64>,
}

impl From<event::FSSSignalDiscovered> for Signal {
    fn from(value: event::FSSSignalDiscovered) -> Self {
        let name = value.signal_name_localised.clone().unwrap_or(value.signal_name.clone());
        Self {
            name,
            kind: value.signal_type,
            is_station: value.is_station.unwrap_or(false),
            uss_type: value.uss_type_localised.or(value.uss_type),
            spawning_state: value.spawning_state_localised.or(value.spawning_state),
            spawning_faction: value.spawning_faction_localised.or(value.spawning_faction),
            threat_level: value.threat_level.map(|v| v as u32),
            time_remaining: value.time_remaining,
        }
    }
}

impl Signal {
    pub(crate) fn get_icon(&self) -> Option<&'static [u8]> {
        match self.kind.as_deref() {
            Some("Installation") => Some(station::OUTPOST),
            Some("FleetCarrier") => Some(station::FLEET_CARRIER),
            Some("StationCoriolis") => Some(station::CORIOLIS_STARPORT),
            Some("StationOcellus") => Some(station::OCELLUS_STARPORT),
            Some("StationOrbis") => Some(station::ORBIS_STARPORT),
            Some("StationAsteroid") => Some(station::ASTEROID_BASE),
            Some("StationMegaShip") | Some("Megaship") => Some(station::MEGASHIP),
            Some("Outpost") => Some(station::OUTPOST),
            _ => Some(image::POI),
        }
    }
}
