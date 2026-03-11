pub mod engineering;
pub mod gui;
pub mod planet;
pub mod settlement;
pub mod ship;
pub mod ship_modules;
pub mod station;

pub const LOADING_PNG: &[u8] = include_bytes!("../img/loading.svg");

pub const FUEL_STAR_PNG: &[u8] = include_bytes!("../img/fuel_star.png");

pub const STAR: &[u8] = include_bytes!("../img/star.svg");

pub const POI: &[u8] = include_bytes!("../img/poi.svg");

pub const POI_EMPTY: &[u8] = include_bytes!("../img/poi-empty.svg");
