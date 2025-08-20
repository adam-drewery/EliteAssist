#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use log::info;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use scraper::{Html, Selector, ElementRef};
use thiserror::Error;
use url::Url;

#[cfg(test)]
mod tests;

/// A simple scraper for fetching material locations from Inara.
///
/// It mirrors the behavior of generate_material_locations.ps1 by:
/// - Requesting https://inara.cz/elite/components/
/// - Parsing the first (Item) and fourth (Location) columns from tables under
///   the Items and Components tabs (supports both legacy ids `#tab_items`/`#tab_components`
///   and current live ids `#items`/`#components`; falls back to the first table if the tab
///   is not found),
/// - Normalizing whitespace,
/// - De-duplicating by first occurrence per Item,
/// - Splitting the Location column by commas and yielding one pair per location part
///   (or an empty string if no location is present).
pub struct Scraper {
    client: reqwest::Client,
    base_url: Url,
}

#[derive(Debug, Error)]
pub enum InaraError {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("invalid base url: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("parse error: {0}")]
    Parse(String),
}

impl Scraper {
    /// Creates a new Scraper with a default User-Agent and base URL
    /// `https://inara.cz/elite/components/`.
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("EliteAssist-Scraper/1.0 (+https://github.com/adam/EliteAssist)"),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        let base_url = Url::parse("https://inara.cz/elite/components/").unwrap();
        Self { client, base_url }
    }

    /// Downloads and parses the Inara components tab and returns
    /// a map of item -> list of locations.
    ///
    /// - Each unique item is considered only on its first occurrence in the table.
    /// - The Location cell is split by commas; the list contains each part.
    /// - If the Location cell is empty, the list will contain a single empty string.
    pub async fn material_locations(&self) -> Result<HashMap<String, Vec<String>>, InaraError> {
        // Target the components tab explicitly
        let mut url = self.base_url.clone();
        url.set_fragment(Some("components"));

        info!("GET {}", url);
        let resp = self.client.get(url).send().await?.error_for_status()?;
        let html = resp.text().await?;
        Ok(parse_tab_locations_from_html(&html, "components"))
    }

    /// Downloads and parses the Inara items tab and returns
    /// a map of item -> list of locations.
    pub async fn item_locations(&self) -> Result<HashMap<String, Vec<String>>, InaraError> {
        let mut url = self.base_url.clone();
        url.set_fragment(Some("items"));

        info!("GET {}", url);
        let resp = self.client.get(url).send().await?.error_for_status()?;
        let html = resp.text().await?;
        Ok(parse_tab_locations_from_html(&html, "items"))
    }
}

/// Parses both item and component tabs and merges them, preferring the first
/// occurrence of an item across tabs (items first, then components).
pub fn parse_material_locations_from_html(html: &str) -> HashMap<String, Vec<String>> {
    let mut combined: HashMap<String, Vec<String>> = HashMap::new();
    let items = parse_tab_locations_from_html(html, "tab_items");
    for (k, v) in items {
        combined.entry(k).or_insert(v);
    }
    let comps = parse_tab_locations_from_html(html, "tab_components");
    for (k, v) in comps {
        combined.entry(k).or_insert(v);
    }
    combined
}

fn find_tables_in_tab<'a>(document: &'a Html, tab_id: &str) -> Vec<ElementRef<'a>> {
    // Support both legacy 'tab_*' and current ids ('items', 'components').
    let mut candidates: Vec<String> = Vec::new();
    candidates.push(tab_id.to_string());
    if let Some(stripped) = tab_id.strip_prefix("tab_") {
        candidates.push(stripped.to_string());
    } else {
        candidates.push(format!("tab_{}", tab_id));
    }

    for id in candidates {
        if let Ok(sel) = Selector::parse(&format!("#{} table", id)) {
            let tables: Vec<ElementRef<'_>> = document.select(&sel).collect();
            if !tables.is_empty() {
                return tables;
            }
        }
    }

    Vec::new()
}

fn collect_locations_from_table(table: ElementRef<'_>, out: &mut HashMap<String, Vec<String>>, seen: &mut HashSet<String>) {
    let tr_sel = Selector::parse("tr").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    for tr in table.select(&tr_sel) {
        let tds: Vec<ElementRef<'_>> = tr.select(&td_sel).collect();
        if tds.len() < 4 { continue; }

        let item = normalize(&inner_text(&tds[0]));
        if item.is_empty() { continue; }

        // Only process first occurrence per item (matching PS script behavior)
        if !seen.insert(item.clone()) {
            continue;
        }

        let location_raw = normalize(&inner_text(&tds[3]));
        let mut parts: Vec<String> = location_raw
            .split(',')
            .map(|s| normalize(s))
            .filter(|s| !s.is_empty())
            .collect();

        if parts.is_empty() {
            parts.push(String::new());
        }
        out.insert(item, parts);
    }
}

fn inner_text(el: &ElementRef<'_>) -> String {
    el.text().collect::<Vec<_>>().join("")
}

fn normalize(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    trimmed.split_whitespace().collect::<Vec<_>>().join(" ")
}



/// Parses a single tab (by id) and returns a map of item -> locations found in that tab.
pub fn parse_tab_locations_from_html(html: &str, tab_id: &str) -> HashMap<String, Vec<String>> {
    let document = Html::parse_document(html);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen: HashSet<String> = HashSet::new();

    let tables = find_tables_in_tab(&document, tab_id);
    if tables.is_empty() {
        // Fallback to the first table in the whole document if the expected tab wasn't found
        if let Some(table) = Selector::parse("table").ok().and_then(|s| document.select(&s).next()) {
            collect_locations_from_table(table, &mut map, &mut seen);
        }
    } else {
        for table in tables {
            collect_locations_from_table(table, &mut map, &mut seen);
        }
    }

    map
}
