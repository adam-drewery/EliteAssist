use chrono::{DateTime, Utc};

pub fn prettify_date(date: &DateTime<Utc>) -> String {
    date.to_rfc2822().trim_end_matches("+0000").to_string()
}

/// Converts the first character of a string to uppercase, leaving the rest unchanged.
///
/// # Examples
///
/// ```
/// let result = title_case("hello");
/// assert_eq!(result, "Hello");
///
/// let empty = title_case("");
/// assert_eq!(empty, "");
/// ```
pub fn title_case(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => first_char.to_uppercase().collect::<String>() + chars.as_str()
    }
}