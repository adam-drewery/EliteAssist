use ed_journals::market::MarketEntry;
use crate::journal::format::title_case;

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

impl From<ed_journals::market::Market> for Market {
    fn from(value: ed_journals::market::Market) -> Self {
        let mut groups = std::collections::HashMap::new();

            let mut items = value.items.clone();
            items.sort_by(|a, b| a.name_localized.cmp(&b.name_localized));

            for item in items {
                groups
                    .entry(item.category_localized.clone())
                    .or_insert_with(Vec::new)
                    .push(item.into());
            }

        Market {
            groups: {
                let mut groups: Vec<_> = groups
                    .into_iter()
                    .map(|(category, items)| MarketItemGroup {
                        name: category.unwrap_or_default(),
                        items,
                    })
                    .collect();
                groups.sort_by(|a, b| a.name.cmp(&b.name));
                groups
            },
        }
    }
}

impl From<MarketEntry> for MarketItem {
    fn from(value: MarketEntry) -> Self {
        MarketItem {
            name: value.name_localized.unwrap_or(title_case(&value.name.to_string())),
            buy_price: value.buy_price,
            sell_price: value.sell_price,
            demand: value.demand,
            consumer: value.consumer,
            producer: value.producer,
            stock: value.stock,
            rare: value.rare,
        }
    }
}