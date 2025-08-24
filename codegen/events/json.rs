use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Clone, Deserialize)]
pub struct SchemaObject {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,

    #[serde(rename = "type", default)]
    pub r#type: String,

    pub format: Option<String>,

    // properties is a dictionary<string, obj>
    #[serde(default)]
    pub properties: BTreeMap<String, SchemaObject>,

    #[serde(default)]
    pub items: Option<Box<SchemaObject>>,

    #[serde(default)]
    pub required: Vec<String>,
}

pub fn load_schemas(schema_root: &Path) -> Result<Vec<SchemaObject>> {
    let entries = fs::read_dir(schema_root)?;
    let mut schemas = Vec::new();

    for entry in entries {
        let path = entry?.path();

        println!("cargo:warning=Loading schema: {}", path.display());

        if path.is_dir() {
            // Recurse into subdirectories
            let mut nested = load_schemas(&path)?;
            schemas.append(&mut nested);
            continue;
        }

        let is_json = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|s| s.eq_ignore_ascii_case("json"))
            .unwrap_or(false);

        if is_json {
            let contents = fs::read_to_string(&path)?;
            // Parse to Value first to gracefully handle possible duplicate keys (last-wins), then deserialize
            let value: Value = serde_json::from_str(&contents)?;
            let mut schema: SchemaObject = serde_json::from_value(value)?;

            // Use filename as title if title is empty/missing
            if schema.title.is_empty() {
                schema.title = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string();
            }

            schemas.push(schema);
        }
    }

    Ok(schemas)
}