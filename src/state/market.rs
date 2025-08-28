use crate::journal::event;
use crate::journal::format::title_case;

#[derive(Default)]
pub struct Market {
    pub groups: Vec<MarketItemGroup>,
}

pub struct MarketItemGroup {
    pub name: Box<str>,
    pub items: Vec<MarketItem>,
}

pub struct MarketItem {
    pub name: Box<str>,
    pub buy_price: u64,
    pub sell_price: u64,
    pub stock: u64,
    pub demand: u64,
    pub consumer: bool,
    pub producer: bool,
    pub rare: bool,
}

impl From<event::Market> for Market {
    fn from(value: event::Market) -> Self {
        let mut groups = std::collections::HashMap::new();

        if let Some(mut items) = value.items {
            items.sort_by(|a, b| a.name_localised.cmp(&b.name_localised));
            for item in items {
                groups
                    .entry(item.category_localised.clone())
                    .or_insert_with(Vec::new)
                    .push(item.into());
            }
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

impl From<event::MarketItem> for MarketItem {
    fn from(value: event::MarketItem) -> Self {
        MarketItem {
            name: value.name_localised.unwrap_or(title_case(value.name.as_ref()).into()),
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