pub fn to_snake_case(s: &str) -> String {
    let mut out = String::new();
    let mut prev_lower = false;
    let mut consecutive_upper = 0;

    for (i, ch) in s.chars().enumerate() {
        if ch.is_ascii_uppercase() {
            if prev_lower {
                out.push('_');
            } else if i > 0 && consecutive_upper > 0 {
                // Handle initialisms - if we have multiple uppercase chars
                // and the next char is lowercase, add underscore before last uppercase
                if let Some(next) = s.chars().nth(i + 1) {
                    if next.is_ascii_lowercase() {
                        out.push('_');
                    }
                }
            }
            for lc in ch.to_lowercase() { out.push(lc); }
            prev_lower = false;
            consecutive_upper += 1;
        } else if ch.is_ascii_alphanumeric() {
            out.push(ch);
            prev_lower = ch.is_ascii_lowercase();
            consecutive_upper = 0;
        } else {
            out.push('_');
            prev_lower = false;
            consecutive_upper = 0;
        }
    }
    if out.starts_with(|c: char| c.is_ascii_digit()) { format!("_{}", out) } else { out }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("USSType"), "uss_type");
        assert_eq!(to_snake_case("JSONData"), "json_data");
        assert_eq!(to_snake_case("SimpleText"), "simple_text");
        assert_eq!(to_snake_case("APIVersion"), "api_version");
        assert_eq!(to_snake_case("1Test"), "_1_test");
        assert_eq!(to_snake_case("HTML5Element"), "html5_element");
    }
}

pub fn singularize(name: &str) -> String {
    if name.ends_with("ies") { return format!("{}y", &name[..name.len()-3]); }
    if name.ends_with('s') && !name.ends_with("ss") { return name[..name.len()-1].to_string(); }
    name.to_string()
}

pub fn to_pascal_case(s: &str) -> String {
    // Check if input is all caps with underscores
    let is_caps_with_underscores = s.chars().all(|c| c.is_uppercase() || c == '_');

    if is_caps_with_underscores {
        s.split('_')
            .filter(|s| !s.is_empty())
            .map(|s| {
                let mut chars = s.chars();
                match chars.next() {
                    Some(c) => {
                        let mut result = c.to_uppercase().collect::<String>();
                        result.extend(chars.map(|c| c.to_lowercase().next().unwrap()));
                        result
                    }
                    None => String::new(),
                }
            })
            .collect()
    } else {
        s.replace("_", "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("ALL_CAPS"), "AllCaps");
        assert_eq!(to_pascal_case("SYSTEM_DATA_UPDATE"), "SystemDataUpdate");
        assert_eq!(to_pascal_case("simple_text"), "SimpleText");
        assert_eq!(to_pascal_case("AlreadyPascalCase"), "AlreadyPascalCase");
        assert_eq!(to_pascal_case("___multiple___underscores___"), "MultipleUnderscores");
    }
}

