use std::collections::{BTreeMap, HashSet};
use anyhow::Result;
use crate::codegen::events::json::{SchemaItems, SchemaObject};
use crate::codegen::events::text;

pub fn build(schemas: Vec<SchemaObject>) -> Result<codegen::Scope> {
    let mut scope = codegen::Scope::new();
    let mut generated: HashSet<String> = HashSet::new();
    scope.import("chrono", "{DateTime, Utc}");
    scope.import("serde", "Deserialize");

    scope.raw("#[derive(Clone, Debug, Deserialize)]");
    scope.raw("#[serde(tag = \"event\")]");
    let enum_ = scope.new_enum("Event")
        .vis("pub");
    

    // First, add all enum variants
    for schema in &schemas {

        match &schema.title {
            None => panic!("Top-level schema missing title: {:?}", schema),
            Some(title) => {
                let variant = enum_.new_variant(title.as_str())
                    .tuple(title.as_str());

                if let Some(description) = &schema.description {
                    variant.annotation(format!("/// {}", description));
                }
            }
        }
    }

    // Then create all structs
    for schema in schemas {
        build_struct(&mut scope, &mut generated, &schema, None);
    }

    Ok(scope)
}

fn build_struct(scope: &mut codegen::Scope, generated: &mut HashSet<String>, schema: &SchemaObject, parent_prop_name: Option<String>) -> String {
    let is_top_level = parent_prop_name.is_none();

    // Generate a name for the struct based on the schema title or the parent property name
    let struct_name = match parent_prop_name {
        None => match &schema.title {
            None => panic!("Schema missing title and no title hint provided from parent: {:?}", schema),
            Some(title) => text::to_pascal_case(&title),
        },
        Some(name) => text::singularize(&name)
    };

    // Prevent duplicate struct generation
    if !generated.insert(struct_name.clone()) {
        return match &schema.title {
            None => struct_name,
            Some(title) => title.to_string()
        };
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
        struct_.new_field("timestamp", "DateTime<Utc>")
            .annotation("#[serde(with = \"crate::journal::format::date\")]");
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
            "integer" => "i64".to_string(),
            "number" => "f64".to_string(),
            "boolean" => "bool".to_string(),
            "object" => {
                // Instead of recursing immediately, collect the schema for later processing. Ensure nested type names are prefixed with parent struct name.
                match &property.1.title {
                    None => panic!("Object type '{}' on '{}' contained no title: {:?}", property_name, struct_name, property.1),
                    Some(title) => {
                        let base_name = format!("{}{}", struct_name, text::to_pascal_case(title));
                        let sub_type_name = text::singularize(&base_name);
                        nested_schemas.push((property.1, Some(sub_type_name.clone())));
                        sub_type_name
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
                                "integer" => "i64",
                                "number" => "u64",
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
                                    nested_schemas.push((&property.1, Some(sub_type_name.clone())));
                                    format!("Vec<{}>", sub_type_name)
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

        if !is_required {
            type_.insert_str(0, "Option<");
            type_.push('>');
        }

        let field = struct_.new_field(property_name, type_.clone());
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

    match &schema.title {
        None => panic!("Schema missing title: {:?}", schema),
        Some(title) => title.to_string()
    }
}