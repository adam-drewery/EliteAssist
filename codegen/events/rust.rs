use crate::codegen::events::json;
use anyhow::Result;

pub fn build(schemas: Vec<json::ObjectSchema>) -> Result<codegen::Scope> {
    let mut scope = codegen::Scope::new();

    let enum_ = scope.new_enum("Event");

    // First add all enum variants
    for schema in &schemas {
        enum_.new_variant(schema.title.as_str())
            .annotation(format!("/// {}", schema.description));
    }

    // Then create all structs
    for schema in schemas {
        let struct_ = scope.new_struct(schema.title.as_str());
    }
    Ok(scope)
}

fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    for ch in s.chars() {
        if ch.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            }
            for lc in ch.to_lowercase() { out.push(lc); }
            prev_lower = false;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_lower = ch.is_ascii_lowercase();
        } else {
            out.push('_');
            prev_lower = false;
        }
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) { format!("_{}", out) } else { out }
}

fn singularize(name: &str) -> String {
    if name.ends_with("ies") { return format!("{}y", &name[..name.len()-3]); }
    if name.ends_with('s') && !name.ends_with("ss") { return name[..name.len()-1].to_string(); }
    name.to_string()
}
