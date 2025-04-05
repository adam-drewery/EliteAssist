use crate::text::title_case;

#[derive(Default)]
pub struct ShipLocker {
    pub items: Vec<ShipLockerItem>,
    pub components: Vec<ShipLockerItem>,
    pub consumables: Vec<ShipLockerItem>,
    pub data: Vec<ShipLockerItem>
}

pub struct ShipLockerItem {
    pub name: String,
    pub count: u64,
    pub for_mission: bool,
}

impl ShipLockerItem {
    pub fn display_name(&self) -> String {
        title_case(&self.name)
    }
}