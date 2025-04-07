use crate::state;
use crate::text::title_case;
use chrono::{DateTime, Utc};
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

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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

#[derive(Deserialize, Debug, Clone)]
pub struct MarketBuy {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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

#[derive(Deserialize, Debug, Clone)]
pub struct MarketSell {

    #[serde(with = "crate::event::format::date")]
    pub timestamp: DateTime<Utc>,

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

impl Into<state::Market> for Market {
    fn into(self) -> state::Market {
        let mut groups = std::collections::HashMap::new();

        if let Some(mut items) = self.items {
            items.sort_by(|a, b| a.name_localised.cmp(&b.name_localised));
            for item in items {
                groups.entry(item.category_localised.clone())
                    .or_insert_with(Vec::new)
                    .push(item.into());
            }
        }

        state::Market {
            id: self.market_id,
            groups: {
                let mut groups: Vec<_> = groups.into_iter()
                    .map(|(category, items)| state::MarketItemGroup {
                        name: category,
                        items,
                    })
                    .collect();
                groups.sort_by(|a, b| a.name.cmp(&b.name));
                groups
            },
        }
    }
}

impl Into<state::MarketItem> for MarketItem {

    fn into(self) -> state::MarketItem {

        state::MarketItem {
            id: self.id,
            name: self.name_localised.unwrap_or(title_case(&self.name)),
            buy_price: self.buy_price,
            sell_price: self.sell_price,
            mean_price: self.mean_price,
            demand: self.demand,
            consumer: self.consumer,
            producer: self.producer,
            demand_bracket: self.demand_bracket,
            stock_bracket: self.stock_bracket,
            stock: self.stock,
            rare: self.rare,
        }
    }
}