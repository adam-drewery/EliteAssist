use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::Write;
use std::path::Path;

use crate::codegen::util::{load_csv, rust_string_lit, sanitize_field};

pub fn generate(manifest_dir: &Path, out_dir: &Path) {
    let data_dir = manifest_dir.join("fdev-ids");
    let gen_path = out_dir.join("fdev_ids_gen.rs");

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
            for (name, _) in &fields {
                output.push_str(&format!("    pub {}: &'static str,\n", name));
            }
            output.push_str("}\n\n");
        }

        // Emit DATA array
        output.push_str(&format!(
            "pub static {}_DATA: [{}; {}] = [\n",
            const_prefix,
            struct_name,
            rows.len()
        ));
        for r in &rows {
            output.push_str(&format!("    {} {{ ", struct_name));
            for (idx, (name, col)) in fields.iter().enumerate() {
                if idx > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}: {}", name, rust_string_lit(&r[*col])));
            }
            output.push_str(" },\n");
        }
        output.push_str("];\n\n");

        // Build PHF map from the lowercased key column to index
        let mut map = phf_codegen::Map::new();
        let mut pairs: Vec<(String, String)> = Vec::new();
        for (i, r) in rows.iter().enumerate() {
            let key_code = r[key_idx].to_lowercase();
            pairs.push((key_code, i.to_string()));
        }
        for (k, v) in &pairs {
            map.entry(k, v);
        }
        output.push_str(&format!(
            "pub static {}_MAP: phf::Map<&'static str, usize> = ",
            const_prefix
        ));
        output.push_str(&map.build().to_string());
        output.push_str(";\n\n");
    }

    let mut emitted_structs: HashSet<String> = HashSet::new();

    // Generate the main datasets using their headers
    gen_table(
        &mut output,
        &data_dir,
        "outfitting.csv",
        "Outfitting",
        "OUTFITTING",
        "symbol",
        &mut emitted_structs,
    );
    gen_table(
        &mut output,
        &data_dir,
        "shipyard.csv",
        "Shipyard",
        "SHIPYARD",
        "symbol",
        &mut emitted_structs,
    );
    gen_table(
        &mut output,
        &data_dir,
        "material.csv",
        "Material",
        "MATERIAL",
        "symbol",
        &mut emitted_structs,
    );

    // Generate rank sets with shared Rank struct and key 'number'
    gen_table(
        &mut output,
        &data_dir,
        "CQCRank.csv",
        "Rank",
        "CQC_RANK",
        "number",
        &mut emitted_structs,
    );
    gen_table(
        &mut output,
        &data_dir,
        "combatrank.csv",
        "Rank",
        "COMBAT_RANK",
        "number",
        &mut emitted_structs,
    );
    gen_table(
        &mut output,
        &data_dir,
        "ExplorationRank.csv",
        "Rank",
        "EXPLORATION_RANK",
        "number",
        &mut emitted_structs,
    );
    gen_table(
        &mut output,
        &data_dir,
        "TradeRank.csv",
        "Rank",
        "TRADE_RANK",
        "number",
        &mut emitted_structs,
    );
    gen_table(
        &mut output,
        &data_dir,
        "FederationRank.csv",
        "Rank",
        "FEDERATION_RANK",
        "number",
        &mut emitted_structs,
    );
    gen_table(
        &mut output,
        &data_dir,
        "EmpireRank.csv",
        "Rank",
        "EMPIRE_RANK",
        "number",
        &mut emitted_structs,
    );

    let mut f = fs::File::create(&gen_path).expect("file created");
    f.write_all(output.as_bytes()).expect("file written");
}
