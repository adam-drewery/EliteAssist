use anyhow::Result;
use crate::codegen::events::json::{SchemaItems, SchemaObject};
use crate::codegen::events::text;

pub fn build(schemas: Vec<SchemaObject>) -> Result<codegen::Scope> {
    let mut scope = codegen::Scope::new();

    let enum_ = scope.new_enum("Event").vis("pub");

    // First, add all enum variants
    for schema in &schemas {
        enum_.new_variant(schema.title.as_str())
            .tuple(schema.title.as_str())
            .annotation(format!("/// {}", schema.description));
    }

    // Then create all structs
    for schema in schemas {
        build_struct(&mut scope, &schema, None);
    }

    Ok(scope)
}

fn build_struct(scope: &mut codegen::Scope, schema: &SchemaObject, parent_prop_name: Option<String>) -> String {
    let is_top_level = parent_prop_name.is_none();
    let name = match parent_prop_name {
        None => text::to_pascal_case(&schema.title),
        Some(name) => text::singularize(&name)
    };

    let struct_ = scope
        .new_struct(name.as_str())
        .doc(schema.description.as_str())
        .vis("pub");

    if is_top_level {
        struct_.derive("Clone")
            .derive("Debug")
            .derive("Deserialize")
            .new_field("timestamp", "DateTime<Utc>");
    }

    // Collect nested schemas to process after we're done with the current struct
    let mut nested_schemas = Vec::new();

    // todo: there's a bug in here for sure. What's this?:
    // todo: its missing the items field on the array schema
    //pub struct Component {
    //     _ref: Option</* UNSUPPORTED TYPE:  on $ref */ ()>,
    // }
    let items_to_iterate = if schema.r#type == "array" {
        match &schema.items {
            None => &schema.properties,
            Some(items) => match items {
                SchemaItems::Single(_) => &schema.properties,
                SchemaItems::Map(map) => &map
            }
        }
    } else {
        &schema.properties
    };

    for property in items_to_iterate {
        let is_required = schema.required.contains(&property.0);

        let name = match property.0.as_str() {
            "Type" => "r#type".to_string(),
            _ => text::to_snake_case(&property.0)
        };

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
                // Instead of recursing immediately, collect the schema for later processing
                nested_schemas.push((property.1, None));
                text::to_pascal_case(&property.1.title).to_string()
            }
            "array" => {
                match &property.1.items.clone().unwrap() {
                    SchemaItems::Single(obj) => {
                        let sub_type_name = match obj.r#type.as_str() {
                            "string" => "String",
                            "integer" => "i64",
                            "number" => "u64",
                            "boolean" => "bool",
                            _ => panic!("Unsupported array type: {}", obj.r#type)
                        };
                        format!("Vec<{}>", sub_type_name)
                    }
                    SchemaItems::Map(map) => {
                        let sub_type_name = text::singularize(property.1.title.as_str());
                        nested_schemas.push((&property.1, Some(sub_type_name.clone())));
                        format!("Vec<{}>", sub_type_name)
                    }
                }
            }
            _ => format!("/* UNSUPPORTED TYPE: {} on {} */ ()", property.1.r#type, property.0)
        };

        if !is_required {
            type_.insert_str(0, "Option<");
            type_.push('>');
        }

        struct_.new_field(name, type_);
    }

    // Now process all nested schemas after we're done with the current struct
    for nested_schema in nested_schemas {
        build_struct(scope, nested_schema.0, nested_schema.1);
    }

    schema.title.to_string()
}