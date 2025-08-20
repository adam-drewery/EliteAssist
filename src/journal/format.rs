use chrono::DateTime;
use chrono::Utc;
use serde::{self, Deserialize, Deserializer};

pub mod date {
    use super::*;

    // Custom deserializer for DateTime<Utc>
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        DateTime::parse_from_rfc3339(&s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(serde::de::Error::custom)
    }
}

pub mod optional_date {
    use super::*;

    // Custom deserializer for Option<DateTime<Utc>>
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => DateTime::parse_from_rfc3339(&s)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_title_case() {
        assert_eq!(title_case("hello"), "Hello");
        assert_eq!(title_case(""), "");
        assert_eq!(title_case("a"), "A");
        assert_eq!(title_case("already Capitalized"), "Already Capitalized");
    }
}