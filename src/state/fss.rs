use std::collections::HashMap;
use crate::journal::event;

#[derive(Default, Clone, Debug)]
pub struct FssState {
    pub discovery: Option<FssDiscovery>,
    pub all_bodies_found: Option<FssAllBodiesFound>,
    pub last_scan: Option<ScanSummary>,
    pub body_signals: HashMap<u64, BodySignals>,
    pub system_signals: Vec<SystemSignal>,
}

#[derive(Default, Clone, Debug)]
pub struct FssDiscovery {
    pub system_name: Box<str>,
    pub progress: f32,
    pub body_count: u32,
    pub non_body_count: u32,
}

impl From<event::FSSDiscoveryScan> for FssDiscovery {
    fn from(value: event::FSSDiscoveryScan) -> Self {
        Self {
            system_name: value.system_name,
            progress: value.progress as f32,
            body_count: value.body_count as u32,
            non_body_count: value.non_body_count as u32,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct FssAllBodiesFound {
    pub system_name: Box<str>,
    pub count: u32,
}

impl From<event::FSSAllBodiesFound> for FssAllBodiesFound {
    fn from(value: event::FSSAllBodiesFound) -> Self {
        Self {
            system_name: value.system_name,
            count: value.count as u32,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct ScanSummary {
    pub body_name: Box<str>,
    pub body_id: u64,
    pub terraform_state: Option<Box<str>>,
    pub was_discovered: bool,
    pub was_mapped: bool,
}

impl From<event::Scan> for ScanSummary {
    fn from(value: event::Scan) -> Self {
        Self {
            body_name: value.body_name,
            body_id: value.body_id as u64,
            terraform_state: value.terraform_state,
            was_discovered: value.was_discovered,
            was_mapped: value.was_mapped,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct SignalCount {
    pub kind: Box<str>,
    pub count: u32,
}

#[derive(Default, Clone, Debug)]
pub struct BodySignals {
    pub body_name: Box<str>,
    pub signals: Vec<SignalCount>,
}

impl From<event::FSSBodySignals> for BodySignals {
    fn from(value: event::FSSBodySignals) -> Self {
        let mut signals: Vec<SignalCount> = Vec::new();
        for sig in value.signals {
            let kind = if let Some(local) = sig.type_localised {
                if !local.is_empty() { local } else { sig.r#type }
            } else { sig.r#type };
            signals.push(SignalCount { kind, count: sig.count as u32 });
        }
        Self { body_name: value.body_name, signals }
    }
}

#[derive(Default, Clone, Debug)]
pub struct SystemSignal {
    pub name: Box<str>,
    pub kind: Option<Box<str>>,
    pub is_station: bool,
}

impl From<event::FSSSignalDiscovered> for SystemSignal {
    fn from(value: event::FSSSignalDiscovered) -> Self {
        let name = value.signal_name_localised.unwrap_or(value.signal_name);
        Self {
            name,
            kind: value.signal_type,
            is_station: value.is_station.unwrap_or(false),
        }
    }
}
