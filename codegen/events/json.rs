use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::Result;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct SchemaObject {
    pub title: String,
    pub description: String,
    pub r#type: String,
    pub format: Option<String>,
    pub properties: BTreeMap<String, SchemaObject>,
    pub items: Option<SchemaItems>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum SchemaItems {
    Single(Box<SchemaObject>),
    Map(BTreeMap<String, SchemaObject>),
}

fn looks_like_schema_object(v: &Value) -> bool {
    if let Some(obj) = v.as_object() {
        let schema_keys = [
            "type",
            "properties",
            "required",
            "$ref",
            "format",
            "enum",
            "oneOf",
            "anyOf",
            "allOf",
            "items",
            "additionalProperties",
            "patternProperties",
            "minimum",
            "maximum",
            "minItems",
            "maxItems",
            "minLength",
            "maxLength",
            "title",
            "description",
        ];
        schema_keys.iter().any(|k| obj.contains_key(*k))
    } else {
        false
    }
}

impl SchemaObject {
    fn from_value(value: &Value) -> Result<Self> {
        let title = value.get("title")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let description = value.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

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
                    .map(|(k, v)| (k.clone(), SchemaObject::from_value(v).unwrap()))
                    .collect()
            })
            .unwrap_or_default();

        let items = if let Some(items_value) = value.get("items") {

            let type_ = items_value.get("type").map(|t| t.as_str().unwrap_or(""));

            println!("cargo:warning=TYPE: {}", type_.unwrap_or(""));

            if items_value.get("title").is_some() {

                println!("cargo:warning=ITS A SINGLE: {}", title);

                // Single schema object
                Some(SchemaItems::Single(Box::new(SchemaObject::from_value(items_value)?)))
            } else {
                println!("cargo:warning=ITS A MAP: {}", title);
                // Map of schema objects
                Some(SchemaItems::Map(
                    items_value.as_object()
                        .map(|obj| {
                            if obj.is_empty() {
                                println!("cargo:warning=Empty items object in schema");
                            }
                            obj.iter()
                                .map(|(k, v)| {
                                    if !v.get("properties").is_some() {
                                        println!("cargo:warning=Missing 'properties' in items object '{}' for key {}", value, k);
                                    }
                                    (k.clone(), SchemaObject::from_value(v.get("properties").unwrap_or(v)).unwrap())
                                })
                                .collect()
                        })
                        .unwrap_or_default()
                ))
            }
        } else {
            None
        };

        let required = value.get("required")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        Ok(SchemaObject {
            title,
            description,
            r#type,
            format,
            properties,
            items,
            required,
        })
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
            let mut schema = SchemaObject::from_value(&value)?;

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