use std::fs;
use std::path::{Path, PathBuf};
use serde_json::Value as JsonValue;

/// Rust build-time integration for generating journal events from schemas.
///
/// Phase A: invoke the existing PowerShell generator to ensure full functionality now,
/// while we work toward a pure Rust implementation in a later phase. This runs on every
/// build and sets rerun-if-changed on all schema JSONs for correctness.
pub fn generate(manifest_dir: &Path, out_dir: &Path) {
    let schema_root = manifest_dir.join("journal-schemas").join("schemas");

    // Ensure rebuilds when any schema JSON changes (recursive walk)
    if schema_root.is_dir() {
        for path in iter_files_recursive(&schema_root) {
            if path
                .extension()
                .and_then(|s| s.to_str())
                .map(|s| s.eq_ignore_ascii_case("json"))
                .unwrap_or(false)
            {
                println!("cargo:rerun-if-changed={}", path.display());
            }
        }
    } else {
        println!(
            "cargo:warning=journal-schemas/schemas not found; events codegen will attempt to run but may produce no changes"
        );
    }

    // We still scan schemas to ensure rerun-if-changed is accurate.
    if let Some(files) = collect_event_schema_files(&schema_root) {
        let (structs, order) = build_full_structs(&schema_root, &files);

        let mut out = String::new();
        out.push_str(&emit_header());
        for def in structs {
            out.push_str(&emit_struct(&def));
        }

        out.push_str("\n");
        out.push_str(&emit_event_enum(order));

        let path = out_dir.join("event.rs");
        if let Err(e) = fs::write(&path, out) {
            println!("cargo:warning=failed to write event.rs: {}", e);
        }
        println!("cargo:rerun-if-changed={}", path.display());

    } else {
        println!("cargo:warning=no schema files found for events");
    }
}

fn iter_files_recursive(root: &Path) -> Vec<PathBuf> {
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    let mut files: Vec<PathBuf> = Vec::new();
    while let Some(p) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&p) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    stack.push(path);
                } else if path.is_file() {
                    files.push(path);
                }
            }
        }
    }
    files
}

fn read_json(path: &Path) -> Option<JsonValue> {
    match fs::read_to_string(path) {
        Ok(s) => serde_json::from_str::<JsonValue>(&s).ok(),
        Err(_) => None,
    }
}

#[derive(Debug, Clone)]
struct EventDef {
    name: String,
    description: Option<String>,
    property_count: usize,
}

fn parse_event_schema(path: &Path) -> Option<EventDef> {
    let json = read_json(path)?;
    let name = json
        .get("title")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .or_else(|| path.file_stem().and_then(|s| s.to_str()).map(|s| s.to_string()))?;
    let description = json.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    let property_count = json
        .get("properties")
        .and_then(|v| v.as_object())
        .map(|o| o.len())
        .unwrap_or(0);
    Some(EventDef { name, description, property_count })
}

fn collect_event_schema_files(schema_root: &Path) -> Option<Vec<PathBuf>> {
    if !schema_root.is_dir() {
        return None;
    }
    let mut out: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = fs::read_dir(schema_root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            // Skip internal directories like _Event or common
            let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if name.starts_with('_') || name.eq_ignore_ascii_case("common") {
                continue;
            }
            let json_path = path.join(format!("{}.json", name));
            if json_path.is_file() {
                out.push(json_path);
            }
        }
    }
    out.sort_by(|a, b| a.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase().cmp(&b.file_stem().and_then(|s| s.to_str()).unwrap_or("").to_lowercase()));
    Some(out)
}

#[derive(Debug, Clone)]
struct RustField {
    name: String,        // snake_case
    json_name: String,   // original JSON property name
    ty: String,          // Rust type
    serde_attr: Option<String>,
    doc: Option<String>,
}

#[derive(Debug, Clone)]
struct RustStructDef {
    name: String,            // PascalCase
    doc: Option<String>,
    fields: Vec<RustField>,
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            }
            for lc in ch.to_lowercase() { out.push(lc); }
            prev_lower = false;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_lower = ch.is_ascii_lowercase();
        } else {
            out.push('_');
            prev_lower = false;
        }
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) { format!("_{}", out) } else { out }
}

fn emit_struct(def: &RustStructDef) -> String {
    let mut s = String::new();
    if let Some(doc) = &def.doc { s.push_str(&format!("/// {}\n", doc)); }
    s.push_str("#[derive(Clone, Debug, Deserialize)]\n");
    s.push_str(&format!("pub struct {} {{\n\n", def.name));
    for f in &def.fields {
        if let Some(doc) = &f.doc { s.push_str(&format!("    /// {}\n", doc)); }
        if let Some(attr) = &f.serde_attr {
            s.push_str(&format!("    #[serde({})]\n", attr));
        } else {
            s.push_str(&format!("    #[serde(rename = \"{}\")]\n", f.json_name));
        }
        s.push_str(&format!("    pub {}: {},\n\n", f.name, f.ty));
    }
    s.push_str("}\n\n");
    s
}

fn emit_header() -> String {
    let mut s = String::new();
    //s.push_str("#![allow(dead_code)]\n");
    s.push_str("// This file is auto-generated by Rust codegen (codegen/events.rs)\n");
    s.push_str(&format!("// Generated on: {}\n", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")));

    s.push_str("// Do not edit manually\n\n");
    s.push_str("use chrono::{DateTime, Utc};\n");
    s.push_str("use serde::Deserialize;\n\n");
    s
}

fn build_minimal_structs(files: &[PathBuf]) -> Vec<RustStructDef> {
    let mut defs: Vec<RustStructDef> = Vec::new();
    for f in files {
        if let Some(json) = read_json(f) {
            let name = json
                .get("title")
                .and_then(|v| v.as_str())
                .or_else(|| f.file_stem().and_then(|s| s.to_str()))
                .unwrap_or("Event");
            let doc = json.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
            let mut fields: Vec<RustField> = Vec::new();
            fields.push(RustField {
                name: "timestamp".to_string(),
                json_name: "timestamp".to_string(),
                ty: "DateTime<Utc>".to_string(),
                serde_attr: Some("with = \"crate::journal::format::date\"".to_string()),
                doc: Some("Event timestamp".to_string()),
            });
            defs.push(RustStructDef { name: name.to_string(), doc, fields });
        }
    }
    defs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    defs
}

fn json_required_list(json: &JsonValue) -> Vec<String> {
    json.get("required")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect::<Vec<_>>())
        .unwrap_or_default()
}

fn map_basic_type(prop_name: &str, schema: &JsonValue, optional: bool) -> (String, Option<String>) {
    // Determine type string
    let mut ty_str = "serde_json::Value".to_string();
    let mut serde_attr: Option<String> = None;

    // Handle union types like ["null","string"]
    let mut t_opt: Option<String> = None;
    if let Some(t) = schema.get("type").and_then(|v| v.as_str()).map(|s| s.to_string()) {
        t_opt = Some(t);
    } else if let Some(arr) = schema.get("type").and_then(|v| v.as_array()) {
        for v in arr {
            if let Some(s) = v.as_str() {
                if s != "null" { t_opt = Some(s.to_string()); break; }
            }
        }
    }

    let t = t_opt.unwrap_or_else(|| "object".to_string());
    match t.as_str() {
        "string" => {
            let is_dt = schema.get("format").and_then(|v| v.as_str()).map(|s| s.eq_ignore_ascii_case("date-time")).unwrap_or(false)
                || prop_name.eq_ignore_ascii_case("timestamp");
            if is_dt {
                ty_str = "DateTime<Utc>".to_string();
                serde_attr = Some("with = \"crate::journal::format::date\"".to_string());
            } else {
                ty_str = "String".to_string();
            }
        }
        "integer" => { ty_str = "u64".to_string(); }
        "number" => { ty_str = "f64".to_string(); }
        "boolean" => { ty_str = "bool".to_string(); }
        "array" => { ty_str = "Vec<serde_json::Value>".to_string(); }
        "object" => { ty_str = "serde_json::Value".to_string(); }
        _ => { ty_str = "serde_json::Value".to_string(); }
    }

    // Optional wrapping and special case for optional date
    if optional {
        if ty_str == "DateTime<Utc>" {
            serde_attr = Some("with = \"crate::journal::format::optional_date\"".to_string());
            ty_str = "Option<DateTime<Utc>>".to_string();
        } else {
            ty_str = format!("Option<{}>", ty_str);
        }
    }

    (ty_str, serde_attr)
}

fn build_basic_structs(files: &[PathBuf]) -> Vec<RustStructDef> {
    let mut defs: Vec<RustStructDef> = Vec::new();
    for f in files {
        if let Some(json) = read_json(f) {
            let struct_name = json
                .get("title")
                .and_then(|v| v.as_str())
                .or_else(|| f.file_stem().and_then(|s| s.to_str()))
                .unwrap_or("Event");
            let doc = json.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
            let mut fields: Vec<RustField> = Vec::new();

            let props_opt = json.get("properties").and_then(|v| v.as_object());
            let required = json_required_list(&json);
            let is_req = |name: &str| required.iter().any(|r| r == name);

            if let Some(props) = props_opt {
                // stable order by key
                let mut keys = props.keys().cloned().collect::<Vec<_>>();
                keys.sort();
                for key in keys {
                    if let Some(schema) = props.get(&key) {
                        let optional = !is_req(&key);
                        let (mut ty, serde_attr) = map_basic_type(&key, schema, optional);
                        let mut rust_name = to_snake_case(&key);
                        if rust_name == "type" { rust_name = "r#type".to_string(); }
                        let doc = schema.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                        fields.push(RustField {
                            name: rust_name,
                            json_name: key.clone(),
                            ty: ty,
                            serde_attr,
                            doc,
                        });
                    }
                }
            }

            // Ensure timestamp exists at top for readability if not included in schema
            if !fields.iter().any(|f| f.json_name == "timestamp") {
                fields.insert(0, RustField {
                    name: "timestamp".to_string(),
                    json_name: "timestamp".to_string(),
                    ty: "DateTime<Utc>".to_string(),
                    serde_attr: Some("with = \"crate::journal::format::date\"".to_string()),
                    doc: Some("Event timestamp".to_string()),
                });
            }

            defs.push(RustStructDef { name: struct_name.to_string(), doc, fields });
        }
    }

    // Deterministic ordering by struct name (alphabetical)
    defs.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    defs
}


// --- Added pure-Rust generator helpers below ---
use std::collections::{HashMap, HashSet};

fn to_pascal_case(s: &str) -> String {
    if s.is_empty() { return String::new(); }
    // If there are no separators and the string is already alphanumeric, preserve existing Camel/PascalCase:
    if s.chars().all(|c| c.is_ascii_alphanumeric()) {
        let mut chars = s.chars();
        if let Some(f) = chars.next() {
            let mut out = String::new();
            out.extend(f.to_uppercase());
            // Keep the rest intact to preserve inner capitals
            out.push_str(chars.as_str());
            if out.ends_with("ID") {
                out = out.trim_end_matches("ID").to_string() + "Id";
            }
            return out;
        }
        return String::new();
    }

    // Otherwise, split on non-alphanumeric and TitleCase each part (lowercasing the rest of each part)
    let mut parts: Vec<String> = Vec::new();
    let mut cur = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() { cur.push(ch); } else if !cur.is_empty() { parts.push(cur.clone()); cur.clear(); }
    }
    if !cur.is_empty() { parts.push(cur); }
    let mut out = String::new();
    for p in parts {
        let mut it = p.chars();
        if let Some(f) = it.next() {
            out.extend(f.to_uppercase());
            out.push_str(&it.as_str().to_lowercase());
        }
    }
    if out.ends_with("ID") { out = out.trim_end_matches("ID").to_string() + "Id"; }
    out
}

fn singularize(name: &str) -> String {
    if name.ends_with("ies") { return format!("{}y", &name[..name.len()-3]); }
    if name.ends_with('s') && !name.ends_with("ss") { return name[..name.len()-1].to_string(); }
    name.to_string()
}

fn emit_event_enum(event_names: Vec<String>) -> String {
    let mut s = String::new();
    s.push_str("#[derive(Clone, Debug, Deserialize)]\n");
    s.push_str("#[serde(tag = \"event\")]\n");
    s.push_str("pub enum Event {\n");
    let mut names = event_names.to_vec();
    names.sort_by(|a,b| a.to_lowercase().cmp(&b.to_lowercase()));
    for name in names {
        s.push_str(&format!("    #[serde(rename = \"{}\")]\n", name));
        s.push_str(&format!("    {}({}),\n\n", name, name));
    }
    s.push_str("}\n\n");
    s
}

fn build_full_structs(schema_root: &Path, files: &[PathBuf]) -> (Vec<RustStructDef>, Vec<String>) {
    let mut structs: Vec<RustStructDef> = Vec::new();
    let mut emitted: HashSet<String> = HashSet::new();
    let mut events: Vec<String> = Vec::new();

    for f in files {
        if let Some(top) = read_json(f) {
            let struct_name = top
                .get("title").and_then(|v| v.as_str())
                .or_else(|| f.file_stem().and_then(|s| s.to_str()))
                .unwrap_or("Event");
            let doc = top.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
            events.push(struct_name.to_string());

            // Build fields for this event struct
            let mut fields: Vec<RustField> = Vec::new();
            // Ensure timestamp first
            fields.push(RustField{
                name: "timestamp".to_string(),
                json_name: "timestamp".to_string(),
                ty: "DateTime<Utc>".to_string(),
                serde_attr: Some("with = \"crate::journal::format::date\"".to_string()),
                doc: Some("Event timestamp".to_string()),
            });

            let props_opt = top.get("properties").and_then(|v| v.as_object());
            let required = json_required_list(&top);
            if let Some(props) = props_opt {
                let mut keys = props.keys().cloned().collect::<Vec<_>>();
                keys.sort();
                for key in keys {
                    if key == "timestamp" { continue; }
                    if let Some(schema) = props.get(&key) {
                        let optional = !required.iter().any(|r| r == &key);
                        let (ty, serde_attr) = type_for_property(struct_name, &key, schema, &top, &mut structs, &mut emitted);
                        let mut rust_name = to_snake_case(&key);
                        if rust_name == "type" { rust_name = "r#type".to_string(); }
                        let doc = schema.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let mut actual_ty = ty;
                        let mut with_attr = serde_attr;
                        if optional {
                            // Special-case optional DateTime
                            if actual_ty == "DateTime<Utc>" {
                                actual_ty = "DateTime<Utc>".to_string();
                                with_attr = Some("with = \"crate::journal::format::optional_date\"".to_string());
                            }
                        }
                        let final_ty = if optional { format!("Option<{}>", actual_ty) } else { actual_ty };
                        // Build combined serde attribute: always include rename, and include with if present
                        let mut parts: Vec<String> = Vec::new();
                        parts.push(format!("rename = \"{}\"", key));
                        if let Some(w) = with_attr { parts.push(w); }
                        let final_attr = Some(parts.join(", "));
                        fields.push(RustField { name: rust_name, json_name: key.clone(), ty: final_ty, serde_attr: final_attr, doc });
                    }
                }
            }

            let mut def = RustStructDef{ name: struct_name.to_string(), doc, fields };
            // Add top-level struct if not already
            if emitted.insert(def.name.clone()) {
                structs.push(def);
            }
        }
    }
    (structs, events)
}

fn type_for_property(parent: &str, prop: &str, schema: &JsonValue, top: &JsonValue, structs: &mut Vec<RustStructDef>, emitted: &mut HashSet<String>) -> (String, Option<String>) {
    // Handle $ref first
    if let Some(r) = schema.get("$ref").and_then(|v| v.as_str()) {
        if let Some(defname) = r.strip_prefix("#/definitions/") {
            if let Some(def) = top.get("definitions").and_then(|d| d.get(defname)) {
                let child_name = format!("{}{}", parent, to_pascal_case(defname));
                ensure_struct_for_schema_object(&child_name, def, top, structs, emitted);
                return (child_name, None);
            }
        }
        // Unknown ref -> fallback to Value
        return ("serde_json::Value".to_string(), None);
    }

    // anyOf/oneOf with null + type
    if let Some(arr) = schema.get("anyOf").or_else(|| schema.get("oneOf")).and_then(|v| v.as_array()) {
        for v in arr {
            if v.get("type").and_then(|t| t.as_str()) == Some("null") { continue; }
            return type_for_property(parent, prop, v, top, structs, emitted);
        }
    }

    // Determine main type
    let mut ty_str: Option<&str> = schema.get("type").and_then(|v| v.as_str());
    // arrays like ["null","string"] are handled above; but handle array here too
    if ty_str.is_none() {
        if let Some(arr) = schema.get("type").and_then(|v| v.as_array()) {
            for v in arr { if let Some(s) = v.as_str() { if s != "null" { ty_str = Some(s); break; } } }
        }
    }
    let t = ty_str.unwrap_or("object");

    match t {
        "string" => {
            let is_dt = schema.get("format").and_then(|v| v.as_str()).map(|s| s.eq_ignore_ascii_case("date-time")).unwrap_or(false)
                || prop.eq_ignore_ascii_case("timestamp");
            if is_dt { ("DateTime<Utc>".to_string(), Some("with = \"crate::journal::format::date\"".to_string())) }
            else { ("String".to_string(), None) }
        }
        "integer" => ("u64".to_string(), None),
        "number" => ("f64".to_string(), None),
        "boolean" => ("bool".to_string(), None),
        "array" => {
            if let Some(items) = schema.get("items") {
                // If items is a $ref
                if let Some(r) = items.get("$ref").and_then(|v| v.as_str()) {
                    if let Some(defname) = r.strip_prefix("#/definitions/") {
                        if let Some(def) = top.get("definitions").and_then(|d| d.get(defname)) {
                            let item_name = format!("{}{}", parent, to_pascal_case(defname));
                            ensure_struct_for_schema_object(&item_name, def, top, structs, emitted);
                            return (format!("Vec<{}>", item_name), None);
                        }
                    }
                }
                // If items.type == object
                if items.get("type").and_then(|v| v.as_str()) == Some("object") || items.get("properties").is_some() {
                    let singular = singularize(prop);
                    let item_name = format!("{}{}", parent, to_pascal_case(&singular));
                    ensure_struct_for_schema_object(&item_name, items, top, structs, emitted);
                    return (format!("Vec<{}>", item_name), None);
                }
                // Primitive items
                let (inner, _) = type_for_property(parent, prop, items, top, structs, emitted);
                return (format!("Vec<{}>", inner), None);
            }
            ("Vec<serde_json::Value>".to_string(), None)
        }
        _ /* object or unknown */ => {
            // explicit properties
            if schema.get("properties").is_some() {
                let child_name = format!("{}{}", parent, to_pascal_case(prop));
                ensure_struct_for_schema_object(&child_name, schema, top, structs, emitted);
                (child_name, None)
            } else {
                ("serde_json::Value".to_string(), None)
            }
        }
    }
}

fn ensure_struct_for_schema_object(name: &str, schema: &JsonValue, top: &JsonValue, structs: &mut Vec<RustStructDef>, emitted: &mut HashSet<String>) {
    if !emitted.insert(name.to_string()) { return; }
    let mut fields: Vec<RustField> = Vec::new();
    let props_opt = schema.get("properties").and_then(|v| v.as_object());
    let required = json_required_list(schema);
    if let Some(props) = props_opt {
        let mut keys = props.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        for key in keys {
            if let Some(sub) = props.get(&key) {
                let optional = !required.iter().any(|r| r == &key);
                let (mut ty, serde_attr) = type_for_property(name, &key, sub, top, structs, emitted);
                if optional { ty = format!("Option<{}>", ty); }
                let mut rust_name = to_snake_case(&key);
                if rust_name == "type" { rust_name = "r#type".to_string(); }
                let doc = sub.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
                let final_attr = if let Some(attr) = serde_attr { Some(attr) } else { Some(format!("rename = \"{}\"", key)) };
                fields.push(RustField { name: rust_name, json_name: key.clone(), ty, serde_attr: final_attr, doc });
            }
        }
    }
    let doc = schema.get("description").and_then(|v| v.as_str()).map(|s| s.to_string());
    structs.push(RustStructDef { name: name.to_string(), doc, fields });
}
