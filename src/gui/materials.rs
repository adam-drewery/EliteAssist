use crate::event::{Event, Material};
use crate::image::*;
use crate::state::State;
use crate::theme::*;
use iced::widget::svg::Handle;
use iced::widget::{column, row, scrollable, svg, text, Column, Row};
use iced::{Element, Fill, Left, Top};
use once_cell::sync::Lazy;
use std::collections::{BTreeMap, HashMap};

static CATEGORY_NAMES: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("1", "Light Metals and Metalloids");
    m.insert("2", "Reactive Nonmetals and Transition Metals");
    m.insert("3", "Chalcogens and Transition Metals");
    m.insert("4", "Base Metals and Post-Transition Metals");
    m.insert("5", "Coinage and Industrial Metals");
    m.insert("6", "Heavy Metals and Metalloids");
    m.insert("7", "Diverse Utility Elements");
    m
});

pub fn materials(state: &State) -> Row<Event> {
    
    row![
        materials_list("RAW", state.materials.raw.clone()),
        materials_list("MANUFACTURED", state.materials.manufactured.clone()),
        materials_list("ENCODED", state.materials.encoded.clone()),
    ]
    .align_y(Top)
    .height(Fill)
}

fn materials_list(title: &str, items: Vec<Material>) -> Column<Event> {
    iced::widget::column![
        text(title).size(20).color(ORANGE),
        scrollable(column(
            group_and_order_items(items)
                .into_iter()
                .flat_map(|(category, items)| {
                    let category_display = CATEGORY_NAMES
                        .get(category.as_str())
                        .copied()
                        .unwrap_or(category.as_str())
                        .to_string();

                    let mut rows = vec![row![text(category_display).size(16).color(GRAY)].padding(2)];

                    rows.extend(items.into_iter().map(|item| {
                        let svg_handle = match item.info().rarity.as_str() {
                            "Very Common" => Handle::from_memory(GRADE_1),
                            "Common" => Handle::from_memory(GRADE_2),
                            "Standard" => Handle::from_memory(GRADE_3),
                            "Rare" => Handle::from_memory(GRADE_4),
                            "Very Rare" => Handle::from_memory(GRADE_5),
                            _ => Handle::from_memory(COURIER_ICON),
                        };

                        row![
                            column![svg(svg_handle).height(16).width(16)].padding([0, 5]),
                            text(item.count).size(16).color(YELLOW).width(36),
                            text(item.display_name()).size(16),
                        ]
                        .padding(2)
                    }));
                    rows
                })
                .map(Element::from)
        ))
        .width(Fill)
    ]
        .align_x(Left)
}

fn group_and_order_items(items: Vec<Material>) -> BTreeMap<String, Vec<Material>> {
    let mut grouped: HashMap<String, Vec<Material>> = HashMap::new();

    for item in items {
        grouped
            .entry(item.info().category.clone())
            .or_default()
            .push(item);
    }

    let mut sorted: BTreeMap<String, Vec<Material>> = BTreeMap::new();

    for (category, mut items) in grouped {
        items.sort_by(|a, b| {
            let rarity_order = |r: &str| match r {
                "Very Common" => 1,
                "Common" => 2,
                "Standard" => 3,
                "Rare" => 4,
                "Very Rare" => 5,
                _ => 6,
            };

            let rarity_cmp = rarity_order(a.info().rarity.as_str())
                .cmp(&rarity_order(b.info().rarity.as_str()));

            if rarity_cmp == std::cmp::Ordering::Equal {
                a.name.cmp(&b.name)
            } else {
                rarity_cmp
            }
        });
        sorted.insert(category, items);
    }

    sorted
}
