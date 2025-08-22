use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;

use phf_codegen::Map as PhfMap;

use crate::build::util::rust_string_lit;

pub fn generate(out_dir: &Path) {
    let gen_path = out_dir.join("inara_gen.rs");

    // Always run; no rerun-if-changed since network data

    let mut output = String::new();

    match fetch_and_parse("https://inara.cz/elite/components/#items", "items") {
        Ok(map) => emit_dataset(&mut output, "ITEM", map),
        Err(e) => {
            println!("cargo:warning=INARA items scraping failed: {}", e);
            emit_empty_dataset(&mut output, "ITEM");
        }
    }

    match fetch_and_parse("https://inara.cz/elite/components/#components", "components") {
        Ok(map) => emit_dataset(&mut output, "MATERIAL", map),
        Err(e) => {
            println!("cargo:warning=INARA materials scraping failed: {}", e);
            emit_empty_dataset(&mut output, "MATERIAL");
        }
    }

    let mut f = fs::File::create(&gen_path).expect("create inara_gen.rs");
    f.write_all(output.as_bytes()).expect("write inara_gen.rs");
}

fn emit_empty_dataset(out: &mut String, prefix: &str) {
    out.push_str(&format!(
        "pub static {}_LOCATION_LISTS: &[&[&str]] = &[];\n",
        prefix
    ));
    out.push_str(&format!(
        "pub static {}_LOCATIONS_MAP: phf::Map<&'static str, usize> = {};
",
        prefix,
        PhfMap::new().build().to_string()
    ));
    out.push('\n');
}

fn emit_dataset(out: &mut String, prefix: &str, mut map: HashMap<String, Vec<String>>) {
    // Sort keys for deterministic output
    let mut entries: Vec<(String, Vec<String>)> = map.drain().collect();
    entries.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

    out.push_str(&format!("pub static {}_LOCATION_LISTS: &[&[&str]] = &[\n", prefix));
    for (_, locs) in &entries {
        out.push_str("    &[");
        for (i, s) in locs.iter().enumerate() {
            if i > 0 { out.push_str(", "); }
            out.push_str(&rust_string_lit(s));
        }
        out.push_str("],\n");
    }
    out.push_str("];\n\n");

    let mut phf = PhfMap::new();
    let mut pairs: Vec<(String, String)> = Vec::new();
    for (i, (name, _)) in entries.iter().enumerate() {
        let key = rust_string_lit(&name.to_lowercase());
        pairs.push((key, i.to_string()));
    }
    for (k, v) in &pairs {
        phf.entry(k, v);
    }
    out.push_str(&format!(
        "pub static {}_LOCATIONS_MAP: phf::Map<&'static str, usize> = {};",
        prefix,
        phf.build().to_string()
    ));
    out.push_str("\n\n");
}

fn fetch_and_parse(url: &str, tab_id: &str) -> Result<HashMap<String, Vec<String>>, String> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("EliteAssist-BuildScraper/1.0 (+https://github.com/adam-drewery/EliteAssist)")
        .build()
        .map_err(|e| e.to_string())?;
    let res = client
        .get(url)
        .send()
        .and_then(|r| r.error_for_status())
        .map_err(|e| e.to_string())?;
    let html = res.text().map_err(|e| e.to_string())?;
    Ok(parse_tab_locations_from_html(&html, tab_id))
}

fn parse_tab_locations_from_html(html: &str, tab_id: &str) -> HashMap<String, Vec<String>> {
    let document = scraper::Html::parse_document(html);
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut seen: HashSet<String> = HashSet::new();

    let tables = find_tables_in_tab(&document, tab_id);
    if tables.is_empty() {
        if let Some(table) = scraper::Selector::parse("table").ok().and_then(|s| document.select(&s).next()) {
            collect_locations_from_table(table, &mut map, &mut seen);
        }
    } else {
        for table in tables {
            collect_locations_from_table(table, &mut map, &mut seen);
        }
    }

    map
}

fn find_tables_in_tab<'a>(document: &'a scraper::Html, tab_id: &str) -> Vec<scraper::ElementRef<'a>> {
    let mut candidates: Vec<String> = Vec::new();
    candidates.push(tab_id.to_string());
    if let Some(stripped) = tab_id.strip_prefix("tab_") {
        candidates.push(stripped.to_string());
    } else {
        candidates.push(format!("tab_{}", tab_id));
    }

    for id in candidates {
        if let Ok(sel) = scraper::Selector::parse(&format!("#{} table", id)) {
            let tables: Vec<scraper::ElementRef<'_>> = document.select(&sel).collect();
            if !tables.is_empty() {
                return tables;
            }
        }
    }

    Vec::new()
}

fn collect_locations_from_table(table: scraper::ElementRef<'_>, out: &mut HashMap<String, Vec<String>>, seen: &mut HashSet<String>) {
    let tr_sel = scraper::Selector::parse("tr").unwrap();
    let td_sel = scraper::Selector::parse("td").unwrap();

    for tr in table.select(&tr_sel) {
        let tds: Vec<scraper::ElementRef<'_>> = tr.select(&td_sel).collect();
        if tds.len() < 4 { continue; }

        let item = normalize(&inner_text(&tds[0]));
        if item.is_empty() { continue; }

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

fn inner_text(el: &scraper::ElementRef<'_>) -> String {
    el.text().collect::<Vec<_>>().join("")
}

fn normalize(s: &str) -> String {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    trimmed.split_whitespace().collect::<Vec<_>>().join(" ")
}
