use std::collections::HashMap;
use crate::edsm;
use crate::journal::event;

// todo:
// FSSBodySignals: when we scan a body and find bio/geo signals
// FSSDiscoveryScan: when we do a honk


#[derive(Default, Clone, Debug)]
pub struct Fss {
    pub progress: Option<ScanProgress>,
    pub bodies: HashMap<u64, ScannedBody>,
    pub signals: Vec<Signal>,
}

#[derive(Default, Clone, Debug)]
pub struct ScanProgress {
    pub progress: f32,
    pub body_count: u32,
    pub non_body_count: u32,
}

impl From<event::FSSDiscoveryScan> for ScanProgress {
    fn from(value: event::FSSDiscoveryScan) -> Self {
        Self {
            progress: value.progress as f32,
            body_count: value.body_count as u32,
            non_body_count: value.non_body_count as u32,
        }
    }
}

impl ScannedBody {

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
        self.planet_class = event.planet_class;
        self.body_name = event.body_name;
        self.body_id = event.body_id;
        self.was_discovered = event.was_discovered;
        self.was_mapped = event.was_mapped;
        self.terraform_state = event.terraform_state;
    }

    pub fn update_from_query(&mut self, response: edsm::bodies::Body) {
        self.body_name = response.name;
        self.body_id = response.id;
        self.terraform_state = response.terraforming_state;
        self.was_discovered = response.discovery.is_some();
    }
}

#[derive(Default, Clone, Debug)]
pub struct SignalCount {
    pub kind: Box<str>,
    pub count: u32,
}

#[derive(Default, Clone, Debug)]
pub struct ScannedBody {
    pub body_id: u64,
    pub body_name: Box<str>,
    pub planet_class: Option<Box<str>>,
    pub parent_id: Option<u64>,
    pub signals: Vec<SignalCount>,
    pub terraform_state: Option<Box<str>>,
    pub was_discovered: bool,
    pub was_mapped: bool,
}

impl From<event::Scan> for ScannedBody {
    fn from(value: event::Scan) -> Self {
        Self {
            body_name: value.body_name,
            body_id: value.body_id,
            planet_class: value.planet_class.filter(|s| !s.is_empty()),
            terraform_state: value.terraform_state,
            was_discovered: value.was_discovered,
            was_mapped: value.was_mapped,
            signals: Vec::new(),
            parent_id: None
        }
    }
}

impl From<event::FSSBodySignals> for ScannedBody {

    fn from(value: event::FSSBodySignals) -> Self {

        Self { 
            body_id: value.body_id,
            body_name: value.body_name,
            planet_class: None,
            parent_id: None,
            signals: value.signals.into_iter().map(|sig| {
                SignalCount {
                    count: sig.count as u32,
                    kind: sig.type_localised
                        .filter(|s| !s.is_empty())
                        .unwrap_or(sig.r#type)
                }
            }).collect(),
            terraform_state: None,
            was_discovered: false,
            was_mapped: false 
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
