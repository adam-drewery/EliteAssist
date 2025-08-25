use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SchemaObject {
    pub title: Option<String>,
    pub description: Option<String>,
    pub r#type: String,
    pub format: Option<String>,
    pub properties: Option<BTreeMap<String, SchemaObject>>,
    pub items: Option<SchemaItems>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum SchemaItems {
    Single(Box<SchemaObject>),
    Map(BTreeMap<String, SchemaObject>),
}

impl SchemaObject {
    fn from_value(value: &Value) -> Self {
        let title = value.get("title")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let description = value.get("description")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let r#type = value.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let format = value.get("format")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let properties = value.get("properties")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), SchemaObject::from_value(v)))
                    .collect()
            });
        
        let required = value.get("required")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        let items = Some(SchemaItems::Map(BTreeMap::new()));

        //
        // // TODO: Handle the items you fucking dummy
        // let items = value.get("items").map(|items_value| {
        //     if let Some(items_obj) = items_value.as_object() {
        //         if items_obj.contains_key("properties") {
        //             // This is a schema object with properties
        //             let mut map = BTreeMap::new();
        //             if let Some(props) = items_obj.get("properties").and_then(|v| v.as_object()) {
        //                 for (k, v) in props {
        //                     map.insert(k.clone(), SchemaObject::from_value(v));
        //                 }
        //             }
        //             SchemaItems::Map(map)
        //         } else {
        //             // This is a simple schema object
        //             SchemaItems::Single(Box::new(SchemaObject::from_value(items_value)))
        //         }
        //     } else {
        //         // This is a simple schema object
        //         SchemaItems::Single(Box::new(SchemaObject::from_value(items_value)))
        //     }
        // });
        //

        SchemaObject {
            title,
            description,
            r#type,
            format,
            properties,
            items,
            required,
        }
    }
}

pub fn load_schemas(schema_root: &Path) -> Result<Vec<SchemaObject>> {
    let entries = fs::read_dir(schema_root)?;
    let mut schemas = Vec::new();

    for entry in entries {
        let path = entry?.path();

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
            let value: Value = serde_json::from_str(&contents)?;
            let mut schema = SchemaObject::from_value(&value);

        if schema.title.is_none() {
            schema.title = Some(path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Unknown")
                .to_string());
        }

            schemas.push(schema);
        }
    }

    Ok(schemas)
}