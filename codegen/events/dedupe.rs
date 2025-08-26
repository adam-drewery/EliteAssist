use std::collections::HashMap;
use crate::codegen::events::{r#override, text};
use crate::codegen::events::json::{SchemaItems, SchemaObject};

pub fn merge(schemas: &mut Vec<SchemaObject>) {
    // 1) Collect duplicates by structural signature (including nested),
    //    tracking both the real generated struct name and the local title-based name
    let mut groups: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for schema in schemas.iter() {
        collect_signatures(schema, None, &mut groups);
    }

    // 2) Build rename map based on STRUCT_NAME_MERGES (match by title-based names)
    //    Apply overrides even if they are a subset of the duplicates; warn only when no override applied.
    let mut rename_map: HashMap<String, String> = HashMap::new();
    for (_sig, entries) in groups.into_iter() {
        let mut real_names: Vec<String> = entries.iter().map(|(real, _)| real.clone()).collect();
        real_names.sort();
        real_names.dedup();

        let mut title_names: Vec<String> = entries.iter().map(|(_, title)| title.clone()).collect();
        title_names.sort();
        title_names.dedup();

        if real_names.len() > 1 {
            // Apply any overrides whose keys are subsets of the set of real struct names in this group
            let mut applied_any = false;
            let real_set: std::collections::HashSet<&str> = real_names.iter().map(|s| s.as_str()).collect();
            for (k, v) in r#override::struct_names().iter() {
                if k.iter().all(|name| real_set.contains(*name)) {
                    for t in k.iter() {
                        if *t != *v {
                            rename_map.insert((*t).to_string(), (*v).to_string());
                            applied_any = true;
                        }
                    }
                }
            }

            if !applied_any {
                // Emit cargo warning with the real struct names formatted like a Rust array
                let arr = format!("[{}]", real_names.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>().join(", "));
                println!("cargo:warning=duplicate structures detected {}", arr);
            }
        }
    }

    // 2b) Always apply explicit overrides globally, regardless of structural grouping
    for (k, v) in r#override::struct_names().iter() {
        for t in k.iter() {
            if *t != *v {
                rename_map.entry((*t).to_string()).or_insert((*v).to_string());
            }
        }
    }

    // 3) Apply explicit struct name hints for top-level schemas by title
    for schema in schemas.iter_mut() {
        if let Some(title) = &schema.title {
            let pascal = text::to_pascal_case(title);
            if let Some(target) = rename_map.get(&pascal) {
                schema.struct_name_hint = Some(target.clone());
            }
        }
    }

    // 4) Apply struct name hints recursively (also covers nested)
    for schema in schemas.iter_mut() {
        apply_struct_hints(schema, &rename_map);
    }
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

fn apply_struct_hints(schema: &mut SchemaObject, rename_map: &HashMap<String, String>) {
    // Wrapper to preserve original signature; delegate to recursive with parent tracking
    apply_struct_hints_with_parent(schema, rename_map, None);
}

fn apply_struct_hints_with_parent(schema: &mut SchemaObject, rename_map: &HashMap<String, String>, parent_real: Option<String>) {
    // Prefer lookup by fully-qualified real struct name (scoped), fallback to title-based for legacy cases
    if let Some(real) = compute_real_name(schema, parent_real.as_deref()) {
        // Apply hint for both top-level and nested structs. For nested, avoid renaming to parent's name to prevent recursion.
        if let Some(target) = rename_map.get(&real) {
            if Some(target.as_str()) != parent_real.as_deref() {
                schema.struct_name_hint = Some(target.clone());
            }
        } else if let Some(title) = &schema.title {
            let pascal = text::to_pascal_case(title);
            if let Some(target) = rename_map.get(&pascal) {
                if Some(target.as_str()) != parent_real.as_deref() {
                    schema.struct_name_hint = Some(target.clone());
                }
            }
        }

        // Recurse with this real name as parent for nested types
        let next_parent = Some(real);
        if let Some(props) = schema.properties.as_mut() {
            for (_k, v) in props.iter_mut() {
                apply_struct_hints_with_parent(v, rename_map, next_parent.clone());
            }
        }
        if let Some(items) = schema.items.as_mut() {
            match items {
                SchemaItems::Single(inner) => apply_struct_hints_with_parent(inner.as_mut(), rename_map, next_parent.clone()),
                SchemaItems::Map(map) => {
                    for (_k, v) in map.iter_mut() { apply_struct_hints_with_parent(v, rename_map, next_parent.clone()); }
                }
            }
        }
    } else {
        // Not a struct-producing schema; still traverse
        if let Some(props) = schema.properties.as_mut() {
            for (_k, v) in props.iter_mut() {
                apply_struct_hints_with_parent(v, rename_map, parent_real.clone());
            }
        }
        if let Some(items) = schema.items.as_mut() {
            match items {
                SchemaItems::Single(inner) => apply_struct_hints_with_parent(inner.as_mut(), rename_map, parent_real.clone()),
                SchemaItems::Map(map) => {
                    for (_k, v) in map.iter_mut() { apply_struct_hints_with_parent(v, rename_map, parent_real.clone()); }
                }
            }
        }
    }
}
