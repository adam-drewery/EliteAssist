use serde::Deserialize;
use crate::{event, state};
use crate::fdev_ids::Outfitting;

impl Into<state::ShipModule> for event::LoadoutModule {
    fn into(self) -> state::ShipModule {

        let (class, rating, name, mount) = Outfitting::metadata(&self.item)
            .map(|details| (
                details.class.parse().unwrap_or(0),
                details.rating.chars().next().unwrap_or('X'),
                details.name.clone(),
                details.mount.clone()
            ))
            .unwrap_or((0, 'X', self.item.clone(), "".to_string()));

        state::ShipModule {
            slot: self.slot.into(),
            name,
            on: self.on,
            priority: self.priority,
            health: self.health,
            value: self.value,
            ammo_in_clip: self.ammo_in_clip,
            ammo_in_hopper: self.ammo_in_hopper,
            engineering: self.engineering.map(|e| e.into()),
            class,
            rating,
            mount
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct FuelCapacity {
    
    #[serde(rename = "Main")]
    pub main: f64,

    #[serde(rename = "Reserve")]
    pub reserve: f64,
}

impl Into<state::FuelCapacity> for FuelCapacity {
    fn into(self) -> state::FuelCapacity {
        state::FuelCapacity {
            main: self.main,
            reserve: self.reserve,
        }
    }
}