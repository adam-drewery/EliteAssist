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
    pub buy_price: u32,
    pub sell_price: u32,
    pub mean_price: u32,
    pub stock_bracket: u32,
    pub demand_bracket: u32,
    pub stock: u32,
    pub demand: u32,
    pub consumer: bool,
    pub producer: bool,
    pub rare: bool,
}