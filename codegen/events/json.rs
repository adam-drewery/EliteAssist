use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

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
    // Optional override for the Rust struct name to support deduplication without renaming enum variants
    pub struct_name_hint: Option<String>,
}

#[derive(Debug, Clone)]
pub enum SchemaItems {
    Single(Box<SchemaObject>),
    Map(BTreeMap<String, SchemaObject>),
}

impl SchemaObject {
    fn from_value(value: &Value, base_dir: Option<&Path>, doc_root: Option<&Value>) -> Self {
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
                    .map(|(k, v)| (k.clone(), SchemaObject::from_value(v, base_dir, doc_root)))
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

        // Absolutely insane bullshit we have to perform just to parse array items
        // It could be an object with a "title" and "type", with a "properties" object,
        // It could also just be the raw object. No "properties" property, the schema definition is directly in "items".
        // It could also be a "simple" definition that specifies it is a string or somesuch.
        // And GUESS WHAT? It can also be a reference to some other file like "$ref": "../common/ShipLockerBackpack.json#definitions/Item"
        let mut items: Option<SchemaItems> = None;
        if r#type == "array" {
            if let Some(items_value) = value.get("items") {
                match items_value {
                    Value::Object(obj) => {
                        // Determine if the item has a type property
                        let item_type_opt = obj.get("type").and_then(|v| v.as_str());
                        let item_has_properties = obj.get("properties").and_then(|v| v.as_object()).is_some();

                        // Handle $ref in items (internal first, then external)
                        if let Some(Value::String(reference)) = obj.get("$ref") {
                            let mut handled_ref = false;
                            // Internal reference within the same document
                            if reference.starts_with('#') {
                                if let Some(root) = doc_root {
                                    if let Some(resolved) = SchemaObject::resolve_internal_ref(root, reference) {
                                        if let Some(props) = resolved.get("properties").and_then(|v| v.as_object()) {
                                            let props_map: BTreeMap<String, SchemaObject> = props
                                                .iter()
                                                .map(|(k, v)| (k.clone(), SchemaObject::from_value(v, base_dir, doc_root)))
                                                .collect();

                                            if let Some(inner_required) = resolved.get("required").and_then(|v| v.as_array()) {
                                                required = inner_required
                                                    .iter()
                                                    .filter_map(|v| v.as_str())
                                                    .map(|s| s.to_string())
                                                    .collect();
                                            }

                                            items = Some(SchemaItems::Map(props_map));
                                        } else {
                                            let item_schema = SchemaObject::from_value(&resolved, base_dir, doc_root);
                                            items = Some(SchemaItems::Single(Box::new(item_schema)));
                                        }
                                        handled_ref = true;
                                    }
                                }
                            }
                            // External reference in another file
                            if !handled_ref {
                                if let Some((resolved, new_base_dir_buf)) = SchemaObject::resolve_external_ref(reference, base_dir) {
                                    let new_base_dir = new_base_dir_buf.as_deref();
                                    if let Some(props) = resolved.get("properties").and_then(|v| v.as_object()) {
                                        let props_map: BTreeMap<String, SchemaObject> = props
                                            .iter()
                                            .map(|(k, v)| (k.clone(), SchemaObject::from_value(v, new_base_dir, None)))
                                            .collect();

                                        if let Some(inner_required) = resolved.get("required").and_then(|v| v.as_array()) {
                                            required = inner_required
                                                .iter()
                                                .filter_map(|v| v.as_str())
                                                .map(|s| s.to_string())
                                                .collect();
                                        }

                                        items = Some(SchemaItems::Map(props_map));
                                    } else {
                                        let item_schema = SchemaObject::from_value(&resolved, new_base_dir, None);
                                        items = Some(SchemaItems::Single(Box::new(item_schema)));
                                    }
                                    handled_ref = true;
                                }
                            }
                            if handled_ref {
                                // done
                            } else {
                                // Fall through to other handlers if unresolved
                            }
                        } else if item_type_opt.is_none() {
                            // No type property - treat the items object itself as the map
                            let props_map: BTreeMap<String, SchemaObject> = obj
                                .iter()
                                .map(|(k, v)| (k.clone(), SchemaObject::from_value(v, base_dir, doc_root)))
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
                                        .map(|(k, v)| (k.clone(), SchemaObject::from_value(v, base_dir, doc_root)))
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
                            let item_schema = SchemaObject::from_value(items_value, base_dir, doc_root);
                            items = Some(SchemaItems::Single(Box::new(item_schema)));
                        }
                    }
                    Value::Array(arr) => {
                        // Tuple validation: use the first item schema if present
                        if let Some(first) = arr.first() {
                            let item_schema = SchemaObject::from_value(first, base_dir, doc_root);
                            // If the first element is an object with properties, flatten it to Map
                            if let Some(props) = first.get("properties").and_then(|v| v.as_object()) {
                                let props_map: BTreeMap<String, SchemaObject> = props
                                    .iter()
                                    .map(|(k, v)| (k.clone(), SchemaObject::from_value(v, base_dir, doc_root)))
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
            struct_name_hint: None,
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
            let mut schema = SchemaObject::from_value(&value, path.parent(), Some(&value));

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

impl SchemaObject {
    // Resolve an internal $ref like "#/definitions/changeEntry" using the given document root
    fn resolve_internal_ref(root: &Value, reference: &str) -> Option<Value> {
        if !reference.starts_with('#') {
            return None;
        }
        let mut frag = &reference[1..]; // drop '#'
        if frag.starts_with('/') {
            frag = &frag[1..];
        }
        let mut current = root.clone();
        if frag.is_empty() {
            return Some(current);
        }
        for seg in frag.split('/') {
            match current {
                Value::Object(obj) => {
                    current = obj.get(seg)?.clone();
                }
                _ => return None,
            }
        }
        Some(current)
    }

    // Resolve an external $ref like "../common/ShipLockerBackpack.json#definitions/Component"
    // Returns the resolved JSON value and the new base directory for further nested refs
    fn resolve_external_ref(reference: &str, base_dir: Option<&Path>) -> Option<(Value, Option<PathBuf>)> {
        let mut parts = reference.splitn(2, '#');
        let path_str = parts.next().unwrap_or("");
        let frag_opt = parts.next();

        // Only handle refs that include a file path
        if path_str.is_empty() {
            return None;
        }

        let full_path = match base_dir {
            Some(dir) => dir.join(path_str),
            None => PathBuf::from(path_str),
        };

        let contents = fs::read_to_string(&full_path).ok()?;
        let mut value: Value = serde_json::from_str(&contents).ok()?;

        if let Some(fragment) = frag_opt {
            let mut frag = fragment.trim();
            if frag.starts_with('/') {
                frag = &frag[1..];
            }
            if !frag.is_empty() {
                for seg in frag.split('/') {
                    match &value {
                        Value::Object(obj) => {
                            value = obj.get(seg)?.clone();
                        }
                        _ => return None,
                    }
                }
            }
        }

        let new_base_dir = full_path.parent().map(|p| p.to_path_buf());
        Some((value, new_base_dir))
    }
}
