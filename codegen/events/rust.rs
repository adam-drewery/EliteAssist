use crate::codegen::events::json::{SchemaItems, SchemaObject};
use crate::codegen::events::text;
use anyhow::Result;
use std::collections::{BTreeMap, HashMap, HashSet};

mod dedupe {
    use super::*;
    use crate::codegen::events::fixes;

    pub fn merge(schemas: &mut Vec<SchemaObject>) {
        // 1) Collect duplicates by structural signature (including nested),
        //    tracking both the real generated struct name and the local title-based name
        let mut groups: HashMap<String, Vec<(String, String)>> = HashMap::new();

        for schema in schemas.iter() {
            collect_signatures(schema, None, &mut groups);
        }

        // 2) Build rename map based on STRUCT_NAME_MERGES (match by title-based names)
        //    and warn for unmatched duplicates (print real struct names)
        let mut rename_map: HashMap<String, String> = HashMap::new();
        for (_sig, entries) in groups.into_iter() {
            let mut real_names: Vec<String> = entries.iter().map(|(real, _)| real.clone()).collect();
            real_names.sort();
            real_names.dedup();

            if real_names.len() > 1 {
                let mut title_names: Vec<String> = entries.iter().map(|(_, title)| title.clone()).collect();
                title_names.sort();
                title_names.dedup();

                if title_names.len() > 1 {
                    if let Some(target) = find_merge_target(&title_names) {
                        for t in &title_names {
                            if t != target {
                                rename_map.insert(t.clone(), target.to_string());
                            }
                        }
                    } else {
                        // Emit cargo warning with the real struct names formatted like a Rust array
                        let arr = format!("[{}]", real_names.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>().join(", "));
                        println!("cargo:warning=duplicate structures detected {}", arr);
                    }
                }
            }
        }

        // 3) Apply renames recursively (keyed by title-based names)
        for schema in schemas.iter_mut() {
            apply_renames(schema, &rename_map);
        }
    }

    fn find_merge_target(names: &[String]) -> Option<&'static str> {
        // Compare order-insensitively against keys in STRUCT_NAME_MERGES
        for (k, v) in fixes::struct_name_merges().iter() {
            if k.len() != names.len() { continue; }
            let mut k_sorted: Vec<&str> = k.iter().copied().collect();
            k_sorted.sort();
            let mut n_sorted: Vec<&str> = names.iter().map(|s| s.as_str()).collect();
            n_sorted.sort();
            if k_sorted == n_sorted { return Some(*v); }
        }
        None
    }

    fn produces_struct(schema: &SchemaObject) -> bool {
        match schema.r#type.as_str() {
            "object" => true,
            "array" => match &schema.items {
                Some(SchemaItems::Map(_)) => true,   // arrays of objects (we generate a struct for these)
                _ => false,                          // arrays of primitives or unspecified -> no struct generated
            },
            _ => false, // primitives do not generate their own struct
        }
    }

    fn compute_real_name(schema: &SchemaObject, parent: Option<&str>) -> Option<String> {
        if !produces_struct(schema) { return None; }
        let title = schema.title.as_ref()?;
        match schema.r#type.as_str() {
            "object" => {
                match parent {
                    None => Some(text::to_pascal_case(title)),
                    Some(p) => {
                        let base = format!("{}{}", p, text::to_pascal_case(title));
                        Some(text::singularize(&base))
                    }
                }
            }
            "array" => {
                match parent {
                    None => Some(text::to_pascal_case(title)),
                    Some(p) => {
                        let base = format!("{}{}", p, text::singularize(title.as_str()));
                        Some(text::singularize(&base))
                    }
                }
            }
            _ => None
        }
    }

    fn collect_signatures(schema: &SchemaObject, parent_real: Option<String>, groups: &mut HashMap<String, Vec<(String, String)>>) {
        // compute signature and collect names only for schemas that produce structs
        let sig = signature(schema);
        if let Some(real_name) = compute_real_name(schema, parent_real.as_deref()) {
            if let Some(title) = &schema.title {
                let title_name = text::to_pascal_case(title);
                groups.entry(sig.clone()).or_default().push((real_name.clone(), title_name));
            }
            // This schema will serve as the parent for nested generated types
            let next_parent = Some(real_name);

            // Recurse into properties
            if let Some(props) = &schema.properties {
                for (_k, v) in props.iter() {
                    collect_signatures(v, next_parent.clone(), groups);
                }
            }

            // Recurse into array items if present
            if let Some(items) = &schema.items {
                match items {
                    SchemaItems::Single(inner) => {
                        collect_signatures(inner.as_ref(), next_parent.clone(), groups);
                    }
                    SchemaItems::Map(map) => {
                        for (_k, v) in map.iter() {
                            collect_signatures(v, next_parent.clone(), groups);
                        }
                    }
                }
            }
        } else {
            // Not a struct-producing schema, but still traverse its children to find deeper structs
            if let Some(props) = &schema.properties {
                for (_k, v) in props.iter() {
                    collect_signatures(v, parent_real.clone(), groups);
                }
            }
            if let Some(items) = &schema.items {
                match items {
                    SchemaItems::Single(inner) => collect_signatures(inner.as_ref(), parent_real.clone(), groups),
                    SchemaItems::Map(map) => {
                        for (_k, v) in map.iter() { collect_signatures(v, parent_real.clone(), groups); }
                    }
                }
            }
        }
    }

    fn signature(schema: &SchemaObject) -> String {
        // Build a deterministic structural signature ignoring titles/descriptions
        let mut parts: Vec<String> = Vec::new();
        parts.push(format!("type={}", schema.r#type));
        if let Some(fmt) = &schema.format { parts.push(format!("format={}", fmt)); }

        if let Some(props) = &schema.properties {
            let mut prop_parts: Vec<String> = Vec::new();
            for (k, v) in props.iter() {
                prop_parts.push(format!("{}:{}", k, signature(v)));
            }
            parts.push(format!("props=[{}]", prop_parts.join(",")));
        }

        if let Some(items) = &schema.items {
            match items {
                SchemaItems::Single(inner) => {
                    parts.push(format!("items_single:{{{}}}", signature(inner.as_ref())));
                }
                SchemaItems::Map(map) => {
                    let mut item_parts: Vec<String> = Vec::new();
                    for (k, v) in map.iter() {
                        item_parts.push(format!("{}:{}", k, signature(v)));
                    }
                    parts.push(format!("items_map=[{}]", item_parts.join(",")));
                }
            }
        }

        if !schema.required.is_empty() {
            let mut req = schema.required.clone();
            req.sort();
            parts.push(format!("required=[{}]", req.join(",")));
        }

        parts.join("|")
    }

    fn apply_renames(schema: &mut SchemaObject, rename_map: &HashMap<String, String>) {
        if let Some(title) = &schema.title {
            let pascal = text::to_pascal_case(title);
            if let Some(new_name) = rename_map.get(&pascal) {
                schema.title = Some(new_name.clone());
            }
        }

        if let Some(props) = schema.properties.as_mut() {
            for (_k, v) in props.iter_mut() {
                apply_renames(v, rename_map);
            }
        }

        if let Some(items) = schema.items.as_mut() {
            match items {
                SchemaItems::Single(inner) => apply_renames(inner.as_mut(), rename_map),
                SchemaItems::Map(map) => {
                    for (_k, v) in map.iter_mut() { apply_renames(v, rename_map); }
                }
            }
        }
    }
}

pub fn build(mut schemas: Vec<SchemaObject>) -> Result<codegen::Scope> {
    // Merge duplicate schemas by renaming according to mapping and warn for unmatched duplicates
    dedupe::merge(&mut schemas);

    // Remove top-level duplicates by title so only one with that name remains
    let mut seen_titles: HashSet<String> = HashSet::new();
    let mut unique_schemas: Vec<SchemaObject> = Vec::new();
    for s in schemas.into_iter() {
        match &s.title {
            None => panic!("Top-level schema missing title: {:?}", s),
            Some(t) => {
                let t = t.clone();
                if seen_titles.insert(t) {
                    unique_schemas.push(s);
                }
            }
        }
    }

    let mut scope = codegen::Scope::new();
    let mut generated: HashSet<String> = HashSet::new();
    scope.import("chrono", "{DateTime, Utc}");
    scope.import("serde", "Deserialize");

    scope.raw("#[derive(Clone, Debug, Deserialize)]");
    scope.raw("#[serde(tag = \"event\")]");
    let enum_ = scope.new_enum("Event")
        .vis("pub");

    // First, add all enum variants
    for schema in &unique_schemas {
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
    for schema in unique_schemas {
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

    match &schema.title {
        None => panic!("Schema missing title: {:?}", schema),
        Some(title) => title.to_string()
    }
}