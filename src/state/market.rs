#[derive(Default)]
pub struct Market {
    pub id: u64,
    pub groups: Vec<MarketItemGroup>,
}

pub struct MarketItemGroup {
    pub name: String,
    pub items: Vec<MarketItem>,
}

pub struct MarketItem {
    pub id: u64,
    pub name: String,
    pub buy_price: u64,
    pub sell_price: u64,
    pub mean_price: u64,
    pub stock_bracket: u64,
    pub demand_bracket: u64,
    pub stock: u64,
    pub demand: u64,
    pub consumer: bool,
    pub producer: bool,
    pub rare: bool,
}