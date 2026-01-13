use std::collections::HashMap;
use crate::{edsm, BoxStrOptionExt};
use crate::journal::event;

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
    pub parent_id: Option<u64>,
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
}

impl Body {
    //
    // pub fn icons(&self) -> Vec<&[u8]> {
    //     let result = Vec::new();
    //
    // }

    pub fn has_ammonia_based_life(&self) -> bool {
        self.is_ammonia_world && self.has_life
    }

    pub fn has_water_based_life(&self) -> bool {
        self.has_life && self.is_water_world
    }

    // todo: how to figure out high value?


    // todo: not u64 plz
    pub fn get_parent_id(event: &event::Scan) -> Option<u64> {
        if let Some(parents) = &event.parents {
            // ignore rings, they're annoying.
            let planet_ids: Vec<u64> = parents.iter().filter_map(|p| p.planet).collect();
            let star_ids: Vec<u64> = parents.iter().filter_map(|p| p.star).collect();

            if planet_ids.len() > 0 { Some(planet_ids[0]) }
            else if star_ids.len() > 0 { Some(star_ids[0]) }
            else { None }


        }
        else { None }
    }

    pub fn update_from_scan(&mut self, event: event::Scan) {
        self.parent_id = Self::get_parent_id(&event);
        self.name = event.body_name;
        self.id = event.body_id as u8;
        self.was_discovered = event.was_discovered;
        self.was_mapped = event.was_mapped;
        self.terraformable = event.terraform_state.as_deref() == Some("Terraformable");
        self.atmosphere = event.atmosphere;
        self.r#type = event.planet_class
            .or_else(||
                event.star_type.map(|s|
                    format!("{}{:?}", s, event.subclass.unwrap_or_default()).into()));
    }

    pub fn update_from_query(&mut self, response: edsm::bodies::Body) {
        self.name = response.name;
        self.id = response.body_id as u8;
        //todo: self.terraformable = response.terraforming_state.as_deref() == Some("Terraformable");
        self.was_discovered = response.discovery.is_some();
    }
}

#[derive(Default, Clone, Debug)]
pub struct SignalCount {
    pub kind: Box<str>,
    pub count: u32,
}

impl From<event::Scan> for Body {
    fn from(value: event::Scan) -> Self {
        Self {
            name: value.body_name,
            id: value.body_id as u8,
            terraformable: value.terraform_state.as_deref() == Some("Terraformable"),
            was_discovered: value.was_discovered,
            was_mapped: value.was_mapped,
            signals: Vec::new(),
            parent_id: None,
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
            has_life: false
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
}

impl From<event::FSSSignalDiscovered> for Signal {
    fn from(value: event::FSSSignalDiscovered) -> Self {
        let name = value.signal_name_localised.unwrap_or(value.signal_name);
        Self {
            name,
            kind: value.signal_type,
            is_station: value.is_station.unwrap_or(false),
        }
    }
}
