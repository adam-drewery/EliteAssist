use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MarketItem {

    pub id: u64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: Option<String>,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Category_Localised")]
    pub category_localised: String,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "MeanPrice")]
    pub mean_price: u32,

    #[serde(rename = "StockBracket")]
    pub stock_bracket: u32,

    #[serde(rename = "DemandBracket")]
    pub demand_bracket: u32,

    #[serde(rename = "Stock")]
    pub stock: u32,

    #[serde(rename = "Demand")]
    pub demand: u32,

    #[serde(rename = "Consumer")]
    pub consumer: bool,

    #[serde(rename = "Producer")]
    pub producer: bool,

    #[serde(rename = "Rare")]
    pub rare: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Market {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "Items")]
    pub items: Option<Vec<MarketItem>>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MarketBuy {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "BuyPrice")]
    pub buy_price: u32,

    #[serde(rename = "TotalCost")]
    pub total_cost: u32,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct MarketSell {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: u64,

    #[serde(rename = "Type")]
    pub r#type: String,

    #[serde(rename = "Type_Localised")]
    pub type_localised: Option<String>,

    #[serde(rename = "Count")]
    pub count: u32,

    #[serde(rename = "SellPrice")]
    pub sell_price: u32,

    #[serde(rename = "TotalSale")]
    pub total_sale: u32,

    #[serde(rename = "AvgPricePaid")]
    pub avg_price_paid: u64,
}