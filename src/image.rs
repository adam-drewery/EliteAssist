mod gui;
pub use gui::SETTINGS;
pub mod engineering;
pub mod ship;
pub mod ship_modules;

pub const LOADING_PNG: &[u8] = include_bytes!("../img/loading.svg");

pub const FUEL_STAR_PNG: &[u8] = include_bytes!("../img/fuel_star.png");



