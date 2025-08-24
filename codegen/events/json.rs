use std::path::Path;
use std::fs;
use serde_json::Value;
use anyhow::Result;

pub trait Property {}

pub struct ValueSchema {
    pub title: String,
    pub description: String,
    pub r#type: String,
    pub examples: Vec<String>,
    pub defaults_to_null: bool,
    pub is_required: bool,
}

pub struct ObjectSchema {
    pub title: String,
    pub description: String,
    pub properties: Vec<Box<dyn Property>>,
    pub defaults_to_null: bool,
    pub is_required: bool,
}

pub struct ArraySchema {
    pub title: String,
    pub description: String,
    pub item_type: Box<dyn Property>,
    pub examples: Vec<String>,
    pub defaults_to_null: bool,
    pub is_required: bool,
}

pub enum ValueType {
    String,
    Number,
    Integer,
    Boolean
}

impl Property for ValueSchema { }
impl Property for ArraySchema { }

impl Property for ObjectSchema { }

pub fn load_schemas(schema_root: &Path) -> Result<Vec<ObjectSchema>> {
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
            let json: Value = serde_json::from_str(&contents)?;

            if let Value::Object(obj) = json {
                let name = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let mut schema = ObjectSchema {
                    title: name,
                    description: obj
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string(),
                    properties: Vec::new(),
                    defaults_to_null: obj
                        .get("default")
                        .map(|v| v.is_null() || v.as_str().map(|s| s == "null").unwrap_or(false))
                        .unwrap_or(false),
                    is_required: obj
                        .get("required")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                };

                if let Some(Value::Object(props)) = obj.get("properties") {
                    for (name, schema_obj) in props {
                        let property = parse_schema_object(name, schema_obj);
                        schema.properties.push(property);
                    }
                }

                schemas.push(schema);
            }
        }
    }

    Ok(schemas)
}

fn parse_schema_object(name: &str, schema: &Value) -> Box<dyn Property> {
    let obj = match schema.as_object() {
        Some(o) => o,
        None => {
            // Fallback: treat non-object schema entries as simple string values
            return Box::new(ValueSchema {
                title: name.to_string(),
                description: String::new(),
                r#type: "string".to_string(),
                examples: Vec::new(),
                defaults_to_null: false,
                is_required: false,
            });
        }
    };

    let type_str = obj
        .get("type")
        .and_then(|v| v.as_str())
        .or_else(|| if obj.get("properties").is_some() { Some("object") } else { None })
        .or_else(|| if obj.get("items").is_some() { Some("array") } else { None })
        .unwrap_or("string");

    match type_str {
        "string" | "number" | "integer" | "boolean" => {
            Box::new(ValueSchema {
                title: name.to_string(),
                description: obj
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                r#type: type_str.to_string(),
                examples: obj
                    .get("examples")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().map(|x| x.to_string()).collect())
                    .unwrap_or_default(),
                defaults_to_null: obj
                    .get("default")
                    .map(|v| v.is_null() || v.as_str().map(|s| s == "null").unwrap_or(false))
                    .unwrap_or(false),
                is_required: obj
                    .get("required")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            })
        }
        "array" => {
            let item_type = match obj.get("items") {
                Some(items) => parse_schema_object("items", items),
                None => Box::new(ValueSchema {
                    title: "items".to_string(),
                    description: String::new(),
                    r#type: "string".to_string(),
                    examples: Vec::new(),
                    defaults_to_null: false,
                    is_required: false,
                }),
            };

            Box::new(ArraySchema {
                title: name.to_string(),
                description: obj
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                item_type,
                examples: obj
                    .get("examples")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().map(|x| x.to_string()).collect())
                    .unwrap_or_default(),
                defaults_to_null: obj
                    .get("default")
                    .map(|v| v.is_null() || v.as_str().map(|s| s == "null").unwrap_or(false))
                    .unwrap_or(false),
                is_required: obj
                    .get("required")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            })
        }
        "object" => {
            let mut properties = Vec::new();
            if let Some(Value::Object(props)) = obj.get("properties") {
                for (prop_name, prop_schema) in props {
                    properties.push(parse_schema_object(prop_name, prop_schema));
                }
            }

            Box::new(ObjectSchema {
                title: name.to_string(),
                description: obj
                    .get("description")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string(),
                properties,
                defaults_to_null: obj
                    .get("default")
                    .map(|v| v.is_null() || v.as_str().map(|s| s == "null").unwrap_or(false))
                    .unwrap_or(false),
                is_required: obj
                    .get("required")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
            })
        }
        _ => panic!("Unknown schema type: {}", type_str),
    }
}