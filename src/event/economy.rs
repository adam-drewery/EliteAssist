use crate::{event, state};
use crate::text::title_case;

impl Into<state::Market> for event::Market {
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

impl Into<state::MarketItem> for event::MarketItem {

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