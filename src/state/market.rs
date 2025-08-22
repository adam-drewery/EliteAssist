#[derive(Default)]
pub struct Market {
    pub groups: Vec<MarketItemGroup>,
}

pub struct MarketItemGroup {
    pub name: String,
    pub items: Vec<MarketItem>,
}

pub struct MarketItem {
    pub name: String,
    pub buy_price: u64,
    pub sell_price: u64,
    pub stock: u64,
    pub demand: u64,
    pub consumer: bool,
    pub producer: bool,
    pub rare: bool,
}