use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let gen_path = out_dir.join("fdev_ids_gen.rs");

    // CSV source directory
    let data_dir = manifest_dir.join("fdev-ids");

    // Ensure rebuilds when CSVs change
    let csv_files = [
        "outfitting.csv",
        "shipyard.csv",
        "material.csv",
        "CQCRank.csv",
        "combatrank.csv",
        "ExplorationRank.csv",
        "TradeRank.csv",
        "FederationRank.csv",
        "EmpireRank.csv",
    ];
    for f in &csv_files {
        println!("cargo:rerun-if-changed={}", data_dir.join(f).display());
    }

    let mut output = String::new();

    // Helpers
    fn rust_escape(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 2);
        out.push('"');
        for ch in s.chars() {
            match ch {
                '\\' => out.push_str("\\\\"),
                '"' => out.push_str("\\\""),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                _ => out.push(ch),
            }
        }
        out.push('"');
        out
    }

    // Load a CSV file into Vec<Vec<(header, value)>>
    fn load_csv(path: &Path) -> anyhow::Result<(Vec<String>, Vec<Vec<String>>)> {
        let mut rdr = csv::Reader::from_path(path)?;
        let headers = rdr
            .headers()?
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let mut rows = Vec::new();
        for result in rdr.records() {
            let rec = result?;
            rows.push(rec.iter().map(|s| s.to_string()).collect::<Vec<_>>());
        }
        Ok((headers, rows))
    }

    // Generate Outfitting
    {
        let (headers, rows) = load_csv(&data_dir.join("outfitting.csv")).expect("load outfitting.csv");
        // expect columns: id, symbol, category, name, mount, guidance, ship, class, rating, entitlement
        let idx = |name: &str| headers.iter().position(|h| h.eq_ignore_ascii_case(name)).expect("missing column");
        let id_i = idx("id");
        let symbol_i = idx("symbol");
        let category_i = idx("category");
        let name_i = idx("name");
        let mount_i = idx("mount");
        let guidance_i = idx("guidance");
        let ship_i = idx("ship");
        let class_i = idx("class");
        let rating_i = idx("rating");
        let entitlement_i = idx("entitlement");

        output.push_str("pub static OUTFITTING_DATA: [Outfitting; ");
        output.push_str(&rows.len().to_string());
        output.push_str("] = [\n");
        for r in &rows {
            output.push_str("    Outfitting { ");
            output.push_str("id: ");
            output.push_str(&rust_escape(&r[id_i]));
            output.push_str(", symbol: ");
            output.push_str(&rust_escape(&r[symbol_i]));
            output.push_str(", category: ");
            output.push_str(&rust_escape(&r[category_i]));
            output.push_str(", name: ");
            output.push_str(&rust_escape(&r[name_i]));
            output.push_str(", mount: ");
            output.push_str(&rust_escape(&r[mount_i]));
            output.push_str(", guidance: ");
            output.push_str(&rust_escape(&r[guidance_i]));
            output.push_str(", ship: ");
            output.push_str(&rust_escape(&r[ship_i]));
            output.push_str(", class: ");
            output.push_str(&rust_escape(&r[class_i]));
            output.push_str(", rating: ");
            output.push_str(&rust_escape(&r[rating_i]));
            output.push_str(", entitlement: ");
            output.push_str(&rust_escape(&r[entitlement_i]));
            output.push_str(" },\n");
        }
        output.push_str("];\n\n");

        let mut map = phf_codegen::Map::new();
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            let key_lit = r[symbol_i].to_lowercase();
            let val_lit = i.to_string();
            pairs.push((key_lit, val_lit));
        }
        for (k, v) in &pairs {
            map.entry(k, v);
        }
        output.push_str("pub static OUTFITTING_MAP: phf::Map<&'static str, usize> = ");
        output.push_str(&map.build().to_string());
        output.push_str(";\n\n");
    }

    // Generate Shipyard
    {
        let (headers, rows) = load_csv(&data_dir.join("shipyard.csv")).expect("load shipyard.csv");
        let idx = |name: &str| headers.iter().position(|h| h.eq_ignore_ascii_case(name)).expect("missing column");
        let id_i = idx("id");
        let symbol_i = idx("symbol");
        let name_i = idx("name");
        let entitlement_i = idx("entitlement");

        output.push_str("pub static SHIPYARD_DATA: [Shipyard; ");
        output.push_str(&rows.len().to_string());
        output.push_str("] = [\n");
        for r in &rows {
            output.push_str("    Shipyard { ");
            output.push_str("id: ");
            output.push_str(&rust_escape(&r[id_i]));
            output.push_str(", symbol: ");
            output.push_str(&rust_escape(&r[symbol_i]));
            output.push_str(", name: ");
            output.push_str(&rust_escape(&r[name_i]));
            output.push_str(", entitlement: ");
            output.push_str(&rust_escape(&r[entitlement_i]));
            output.push_str(" },\n");
        }
        output.push_str("];\n\n");

        let mut map = phf_codegen::Map::new();
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            let key_lit = r[symbol_i].to_lowercase();
            let val_lit = i.to_string();
            pairs.push((key_lit, val_lit));
        }
        for (k, v) in &pairs {
            map.entry(k, v);
        }
        output.push_str("pub static SHIPYARD_MAP: phf::Map<&'static str, usize> = ");
        output.push_str(&map.build().to_string());
        output.push_str(";\n\n");
    }

    // Generate Material
    {
        let (headers, rows) = load_csv(&data_dir.join("material.csv")).expect("load material.csv");
        let idx = |name: &str| headers.iter().position(|h| h.eq_ignore_ascii_case(name)).expect("missing column");
        let id_i = idx("id");
        let symbol_i = idx("symbol");
        let rarity_i = idx("rarity");
        let type_i = idx("type");
        let category_i = idx("category");
        let name_i = idx("name");

        output.push_str("pub static MATERIAL_DATA: [Material; ");
        output.push_str(&rows.len().to_string());
        output.push_str("] = [\n");
        for r in &rows {
            output.push_str("    Material { ");
            output.push_str("id: ");
            output.push_str(&rust_escape(&r[id_i]));
            output.push_str(", symbol: ");
            output.push_str(&rust_escape(&r[symbol_i]));
            output.push_str(", rarity: ");
            output.push_str(&rust_escape(&r[rarity_i]));
            output.push_str(", r#type: ");
            output.push_str(&rust_escape(&r[type_i]));
            output.push_str(", category: ");
            output.push_str(&rust_escape(&r[category_i]));
            output.push_str(", name: ");
            output.push_str(&rust_escape(&r[name_i]));
            output.push_str(" },\n");
        }
        output.push_str("];\n\n");

        let mut map = phf_codegen::Map::new();
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            let key_lit = r[symbol_i].to_lowercase();
            let val_lit = i.to_string();
            pairs.push((key_lit, val_lit));
        }
        for (k, v) in &pairs {
            map.entry(k, v);
        }
        output.push_str("pub static MATERIAL_MAP: phf::Map<&'static str, usize> = ");
        output.push_str(&map.build().to_string());
        output.push_str(";\n\n");
    }

    // Helper to generate a rank set from a file name and const prefix
    fn gen_rank(output: &mut String, data_dir: &Path, file_name: &str, const_prefix: &str) {
        let (headers, rows) = load_csv(&data_dir.join(file_name)).expect("load rank csv");
        let idx = |name: &str| headers.iter().position(|h| h.eq_ignore_ascii_case(name)).expect("missing column");
        let number_i = idx("number");
        let name_i = idx("name");

        output.push_str(&format!("pub static {}_RANK_DATA: [Rank; {}] = [\n", const_prefix, rows.len()));
        for r in &rows {
            output.push_str("    Rank { number: ");
            output.push_str(&rust_escape(&r[number_i]));
            output.push_str(", name: ");
            output.push_str(&rust_escape(&r[name_i]));
            output.push_str(" },\n");
        }
        output.push_str("];\n\n");

        let mut map = phf_codegen::Map::new();
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            let key_lit = r[number_i].to_lowercase();
            let val_lit = i.to_string();
            pairs.push((key_lit, val_lit));
        }
        for (k, v) in &pairs {
            map.entry(k, v);
        }
        output.push_str(&format!("pub static {}_RANK_MAP: phf::Map<&'static str, usize> = ", const_prefix));
        output.push_str(&map.build().to_string());
        output.push_str(";\n\n");
    }

    gen_rank(&mut output, &data_dir, "CQCRank.csv", "CQC");
    gen_rank(&mut output, &data_dir, "combatrank.csv", "COMBAT");
    gen_rank(&mut output, &data_dir, "ExplorationRank.csv", "EXPLORATION");
    gen_rank(&mut output, &data_dir, "TradeRank.csv", "TRADE");
    gen_rank(&mut output, &data_dir, "FederationRank.csv", "FEDERATION");
    gen_rank(&mut output, &data_dir, "EmpireRank.csv", "EMPIRE");

    // Write file
    let mut f = fs::File::create(&gen_path).expect("file created");
    f.write_all(output.as_bytes()).expect("file written");
}
