use crate::codegen::events::json::{SchemaItems, SchemaObject};
use crate::codegen::events::{dedupe, text, r#override};
use anyhow::Result;
use std::collections::{BTreeMap, HashSet};

pub fn build(mut schemas: Vec<SchemaObject>) -> Result<codegen::Scope> {
    // Detect duplicate structures and set struct name hints; do not rename titles
    dedupe::merge(&mut schemas);

    let mut scope = codegen::Scope::new();
    let mut generated: HashSet<String> = HashSet::new();
    scope.import("chrono", "{DateTime, Utc}");
    scope.import("serde", "Deserialize");

    // 1) Generate all structs (deduped by actual struct name), capturing each top-level schema's struct name
    let mut top_level: Vec<(String, Option<String>, String)> = Vec::new(); // (variant_name, description, struct_name)
    for schema in &schemas {
        let title = schema.title.clone().expect("Top-level schema missing title");
        let struct_name = build_struct(&mut scope, &mut generated, schema, None);
        top_level.push((title, schema.description.clone(), struct_name));
    }

    // 2) Generate the Event enum after structs so variants never collapse
    scope.raw("#[derive(Clone, Debug, Deserialize)]");
    scope.raw("#[serde(tag = \"event\")]");
    let enum_ = scope.new_enum("Event").vis("pub");

    for (variant_name, description, struct_name) in top_level {
        let variant = enum_.new_variant(variant_name.as_str()).tuple(struct_name.as_str());
        if let Some(desc) = description { variant.annotation(format!("/// {}", desc)); }
    }

    Ok(scope)
}

fn build_struct(scope: &mut codegen::Scope, generated: &mut HashSet<String>, schema: &SchemaObject, parent_prop_name: Option<String>) -> String {
    let is_top_level = parent_prop_name.is_none();

    // Generate a name for the struct based on struct_name_hint, schema title, or the parent-provided name
    let struct_name = match parent_prop_name {
        None => match &schema.title {
            None => panic!("Schema missing title and no title hint provided from parent: {:?}", schema),
            Some(title) => schema
                .struct_name_hint
                .clone()
                .unwrap_or_else(|| text::to_pascal_case(&title)),
        },
        Some(name) => schema.struct_name_hint.clone().unwrap_or(name),
    };

    // Prevent duplicate struct generation
    if !generated.insert(struct_name.clone()) {
        // Struct with this name already generated; just return the actual struct name
        return struct_name;
    }

    let struct_ = scope
        .new_struct(struct_name.as_str())
        .vis("pub");

    // Add derives for all structs so serde field attributes work on nested types too
    struct_.derive("Clone")
        .derive("Debug")
        .derive("Deserialize");

    // Add a doc comment for the struct if a description is provided
    if let Some(description) = &schema.description {
        struct_.doc(description.as_str());
    }

    if is_top_level {
        struct_
            .new_field("timestamp", "DateTime<Utc>")
            .annotation("#[serde(with = \"crate::journal::format::date\")]")
            .vis("pub");
    }

    // Collect nested schemas to process after we're done with the current struct
    let mut nested_schemas = Vec::new();

    // Determine which property contains the actual schema, based on the type property
    let items_to_iterate = if schema.r#type == "array" {
        match &schema.items {
            None => panic!("Array type '{}' contained no items: {:?}", struct_name, schema),
            Some(items) => match items {
                SchemaItems::Single(_) => match &schema.properties {
                    None => panic!("Array type '{}' contained no properties: {:?}", struct_name, schema),
                    Some(properties) => properties,
                },
                SchemaItems::Map(map) => &map
            }
        }
    } else {
        match &schema.properties {

            // some schemas have no properties, e.g. ColonisationBeaconDeployed. It will still have a timestamp added later though.
            None => &BTreeMap::new(),
            Some(properties) => properties

        }
    };

    for property in items_to_iterate {
        let is_required = schema.required.contains(&property.0);

        // Generate a name for the field based on the property name
        let property_name = match property.0.as_str() {
            "Type" => "r#type".to_string(),
            "type" => "r#type".to_string(),
            _ => text::to_snake_case(&property.0)
        };

        // Generate a type for the field based on the property schema
        let mut type_ = match property.1.r#type.as_str() {
            "string" => {
                if &property.1.format.clone().unwrap_or_default() == "date-time" {
                    "DateTime<Utc>".to_string()
                } else {
                    "String".to_string()
                }
            }
            "integer" => "u64".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "object" => {
                // Instead of recursing immediately, collect the schema for later processing. Ensure nested type names are prefixed with parent struct name.
                match &property.1.title {
                    None => panic!("Object type '{}' on '{}' contained no title: {:?}", property_name, struct_name, property.1),
                    Some(title) => {
                        let base_name = format!("{}{}", struct_name, text::to_pascal_case(title));
                        let sub_type_name = text::singularize(&base_name);
                        let effective_name = property.1.struct_name_hint.clone().unwrap_or(sub_type_name.clone());
                        nested_schemas.push((property.1, Some(effective_name.clone())));
                        effective_name
                    }
                }
            }
            "array" => {
                match &property.1.items {
                    None => panic!("Array property '{}' on '{}' missing items field: {:?}", property_name, struct_name, schema),
                    Some(items) => match items {
                        SchemaItems::Single(obj) => {
                            let sub_type_name = match obj.r#type.as_str() {
                                "string" => "String",
                                "integer" => "u64",
                                "number" => "f64",
                                "boolean" => "bool",
                                _ => panic!("Unsupported array type: {} for struct: {}", obj.r#type, struct_name)
                            };

                            format!("Vec<{}>", sub_type_name)
                        }
                        SchemaItems::Map(_) => {
                            match &property.1.title {
                                None => panic!("Array type '{}' on '{}' contained no title: {:?}", property_name, struct_name, property.1),
                                Some(title) => {
                                    let sub_type_name = format!("{}{}", struct_name, text::singularize(title.as_str()));
                                    let effective_name = property.1.struct_name_hint.clone().unwrap_or(sub_type_name.clone());
                                    nested_schemas.push((&property.1, Some(effective_name.clone())));
                                    format!("Vec<{}>", effective_name)
                                }
                            }
                        }
                    }
                }
            }
            _ => {
                println!("cargo:warning=Type '{}' not found for property {} on {}. Skipping property", property.1.r#type, property.0, struct_name);
                "()".to_string()
            }
        };

        // Apply FIELD_TYPES override by struct-qualified field name first, then by plain field name (legacy)
        let qualified_key = format!("{}.{}", struct_name, property_name);
        if let Some(forced) = r#override::FIELD_TYPES.get(qualified_key.as_str()) {
            type_ = (*forced).to_string();
        } else if let Some(forced) = r#override::FIELD_TYPES.get(property_name.as_str()) {
            type_ = (*forced).to_string();
        }

        if !is_required {
            type_.insert_str(0, "Option<");
            type_.push('>');
        }

        let field = struct_.new_field(property_name, type_.clone()).vis("pub");
        field.annotation(format!("#[serde(rename = \"{}\")]", property.0));
        if type_ == "DateTime<Utc>" {
            field.annotation("#[serde(with = \"crate::journal::format::date\")]".to_string());
        } else if type_ == "Option<DateTime<Utc>>" {
            field.annotation("#[serde(with = \"crate::journal::format::optional_date\")]".to_string());
        }
    }

    // Now process all nested schemas after we're done with the current struct
    for nested_schema in nested_schemas {
        build_struct(scope, generated, nested_schema.0, nested_schema.1);
    }

    println!("cargo:warning=BUILT {:?}", struct_);
    
    // Return the actual struct name used/generated
    struct_name
}
