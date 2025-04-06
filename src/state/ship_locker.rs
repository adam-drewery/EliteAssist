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