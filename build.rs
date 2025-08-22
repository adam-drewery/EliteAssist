use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};

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
    fn rust_string_lit(s: &str) -> String {
        // Turn a raw string into a Rust string literal
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

    fn sanitize_field(header: &str) -> String {
        let s = header.trim().to_lowercase();
        if s == "type" {
            return "r#type".to_string();
        }
        // Replace non-identifier characters with '_' and avoid leading digits
        let mut result = String::new();
        for (i, ch) in s.chars().enumerate() {
            let valid = ch.is_ascii_alphanumeric() || ch == '_';
            let ch = if valid { ch } else { '_' };
            if i == 0 && ch.is_ascii_digit() {
                result.push('_');
            }
            result.push(ch);
        }
        if result.is_empty() { "_".to_string() } else { result }
    }

    // Load a CSV file into (headers, rows)
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

    // Generic generator: emits struct (once per struct), DATA array, and PHF map
    fn gen_table(
        output: &mut String,
        data_dir: &Path,
        file_name: &str,
        struct_name: &str,
        const_prefix: &str,
        key_header: &str,
        emitted_structs: &mut HashSet<String>,
    ) {
        let (headers, rows) = load_csv(&data_dir.join(file_name)).expect("load csv");

        // Build field list and mapping to indices
        let mut used = HashMap::<String, usize>::new();
        let mut fields: Vec<(String, usize)> = Vec::new();
        for (i, h) in headers.iter().enumerate() {
            let base = sanitize_field(h);
            let name = if let Some(count) = used.get(&base) {
                let n = count + 1;
                used.insert(base.clone(), n);
                format!("{}_{n}", base)
            } else {
                used.insert(base.clone(), 0);
                base
            };
            fields.push((name, i));
        }

        // Find key column index using header name match (case-insensitive)
        let mut key_idx_opt = None;
        for (i, h) in headers.iter().enumerate() {
            if h.eq_ignore_ascii_case(key_header) {
                key_idx_opt = Some(i);
                break;
            }
        }
        let key_idx = key_idx_opt.expect("missing key header");

        // Emit struct if not already emitted
        if emitted_structs.insert(struct_name.to_string()) {
            output.push_str(&format!("pub struct {} {{\n", struct_name));
            for (fname, _) in &fields {
                output.push_str(&format!("    pub {}: &'static str,\n", fname));
            }
            output.push_str("}\n\n");
        }

        // Emit DATA array
        output.push_str(&format!("pub static {}_DATA: [{}; {}] = [\n", const_prefix, struct_name, rows.len()));
        for r in &rows {
            output.push_str(&format!("    {} {{ ", struct_name));
            for (idx, (fname, col)) in fields.iter().enumerate() {
                if idx > 0 { output.push_str(", "); }
                output.push_str(&format!("{}: {}", fname, rust_string_lit(&r[*col])));
            }
            output.push_str(" },\n");
        }
        output.push_str("];\n\n");

        // Build PHF map from lowercased key column to index
        let mut map = phf_codegen::Map::new();
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            let key_code = r[key_idx].to_lowercase();
            pairs.push((key_code, i.to_string()));
        }
        for (k, v) in &pairs {
            map.entry(k, v);
        }
        output.push_str(&format!("pub static {}_MAP: phf::Map<&'static str, usize> = ", const_prefix));
        output.push_str(&map.build().to_string());
        output.push_str(";\n\n");
    }

    let mut emitted_structs: HashSet<String> = HashSet::new();

    // Generate the main datasets using their headers
    gen_table(&mut output, &data_dir, "outfitting.csv", "Outfitting", "OUTFITTING", "symbol", &mut emitted_structs);
    gen_table(&mut output, &data_dir, "shipyard.csv",   "Shipyard",   "SHIPYARD",   "symbol", &mut emitted_structs);
    gen_table(&mut output, &data_dir, "material.csv",   "Material",   "MATERIAL",   "symbol", &mut emitted_structs);

    // Generate rank sets with shared Rank struct and key 'number'
    gen_table(&mut output, &data_dir, "CQCRank.csv",         "Rank", "CQC_RANK",         "number", &mut emitted_structs);
    gen_table(&mut output, &data_dir, "combatrank.csv",      "Rank", "COMBAT_RANK",      "number", &mut emitted_structs);
    gen_table(&mut output, &data_dir, "ExplorationRank.csv", "Rank", "EXPLORATION_RANK", "number", &mut emitted_structs);
    gen_table(&mut output, &data_dir, "TradeRank.csv",       "Rank", "TRADE_RANK",       "number", &mut emitted_structs);
    gen_table(&mut output, &data_dir, "FederationRank.csv",  "Rank", "FEDERATION_RANK",  "number", &mut emitted_structs);
    gen_table(&mut output, &data_dir, "EmpireRank.csv",      "Rank", "EMPIRE_RANK",      "number", &mut emitted_structs);

    // Write file
    let mut f = fs::File::create(&gen_path).expect("file created");
    f.write_all(output.as_bytes()).expect("file written");
}
