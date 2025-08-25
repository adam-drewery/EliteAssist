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
        
        // Collect required fields (for objects) by default from the current schema
        let mut required = value.get("required")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();

        // Parse array items if this schema represents an array
        let mut items: Option<SchemaItems> = None;
        if r#type == "array" {
            if let Some(items_value) = value.get("items") {
                match items_value {
                    Value::Object(obj) => {
                        // Determine if the item has a type property
                        let item_type_opt = obj.get("type").and_then(|v| v.as_str());
                        let item_has_properties = obj.get("properties").and_then(|v| v.as_object()).is_some();

                        if item_type_opt.is_none() {
                            // No type property - treat the items object itself as the map
                            let props_map: BTreeMap<String, SchemaObject> = obj
                                .iter()
                                .map(|(k, v)| (k.clone(), SchemaObject::from_value(v)))
                                .collect();
                            items = Some(SchemaItems::Map(props_map));
                        } else if item_type_opt == Some("object") || item_has_properties {
                            // Build a map of properties for the item object
                            let props_map: BTreeMap<String, SchemaObject> = obj
                                .get("properties")
                                .and_then(|v| v.as_object())
                                .map(|props| {
                                    props
                                        .iter()
                                        .map(|(k, v)| (k.clone(), SchemaObject::from_value(v)))
                                        .collect()
                                })
                                .unwrap_or_default();

                            // Propagate inner required fields up to the array schema so that
                            // codegen can mark fields correctly when generating the item struct
                            if let Some(inner_required) = obj.get("required").and_then(|v| v.as_array()) {
                                required = inner_required
                                    .iter()
                                    .filter_map(|v| v.as_str())
                                    .map(|s| s.to_string())
                                    .collect();
                            }

                            items = Some(SchemaItems::Map(props_map));
                        } else {
                            // Primitive or other non-object item schema
                            let item_schema = SchemaObject::from_value(items_value);
                            items = Some(SchemaItems::Single(Box::new(item_schema)));
                        }
                    }
                    Value::Array(arr) => {
                        // Tuple validation: use the first item schema if present
                        if let Some(first) = arr.first() {
                            let item_schema = SchemaObject::from_value(first);
                            // If the first element is an object with properties, flatten it to Map
                            if let Some(props) = first.get("properties").and_then(|v| v.as_object()) {
                                let props_map: BTreeMap<String, SchemaObject> = props
                                    .iter()
                                    .map(|(k, v)| (k.clone(), SchemaObject::from_value(v)))
                                    .collect();

                                if let Some(inner_required) = first.get("required").and_then(|v| v.as_array()) {
                                    required = inner_required
                                        .iter()
                                        .filter_map(|v| v.as_str())
                                        .map(|s| s.to_string())
                                        .collect();
                                }

                                items = Some(SchemaItems::Map(props_map));
                            } else {
                                items = Some(SchemaItems::Single(Box::new(item_schema)));
                            }
                        }
                    }
                    _ => {
                        // Unsupported form; leave as None
                        items = None;
                    }
                }
            }
        }

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