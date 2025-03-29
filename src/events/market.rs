use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MarketItem {

    pub id: i64,

    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Name_Localised")]
    pub name_localised: String,

    #[serde(rename = "Category")]
    pub category: String,

    #[serde(rename = "Category_Localised")]
    pub category_localised: String,

    #[serde(rename = "BuyPrice")]
    pub buy_price: i64,

    #[serde(rename = "SellPrice")]
    pub sell_price: i64,

    #[serde(rename = "MeanPrice")]
    pub mean_price: i64,

    #[serde(rename = "StockBracket")]
    pub stock_bracket: i64,

    #[serde(rename = "DemandBracket")]
    pub demand_bracket: i64,

    #[serde(rename = "Stock")]
    pub stock: i64,

    #[serde(rename = "Demand")]
    pub demand: i64,

    #[serde(rename = "Consumer")]
    pub consumer: bool,

    #[serde(rename = "Producer")]
    pub producer: bool,

    #[serde(rename = "Rare")]
    pub rare: bool,
}

#[derive(Deserialize, Debug)]
pub struct Market {

    pub timestamp: String,

    #[serde(rename = "MarketID")]
    pub market_id: i64,

    #[serde(rename = "StationName")]
    pub station_name: String,

    #[serde(rename = "StationType")]
    pub station_type: String,

    #[serde(rename = "StarSystem")]
    pub star_system: String,

    #[serde(rename = "Items")]
    pub items: Vec<MarketItem>,
}